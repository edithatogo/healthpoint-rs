#!/usr/bin/env python3
"""Validate MCP registry submission readiness for healthpoint-rs.

This is intentionally conservative: it verifies local manifest/package evidence
and records which registries are automatable versus manually gated.
"""
from __future__ import annotations

import json
import pathlib
import re
import subprocess
import sys
import tomllib
from dataclasses import dataclass

ROOT = pathlib.Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class Registry:
    name: str
    url: str
    submission: str
    requirement: str
    automatable: bool


REGISTRIES = [
    Registry(
        "Official MCP Registry",
        "https://github.com/modelcontextprotocol/registry",
        "mcp-publisher login && mcp-publisher publish",
        "Valid server.json plus a published package in a supported trusted registry. The live registry currently accepts OCI, npm, PyPI, and NuGet package ownership validation; Cargo remains the Rust distribution path but is not accepted by the live publisher.",
        True,
    ),
    Registry(
        "Smithery",
        "https://smithery.ai/",
        "Provider/account submission or repository import.",
        "Public repository, install instructions, server metadata, and maintainer account.",
        False,
    ),
    Registry(
        "Glama MCP server directory",
        "https://glama.ai/mcp/servers",
        "Directory submission/indexing.",
        "Public repository metadata, a repo-local glama.json manifest, and MCP server install/use documentation.",
        False,
    ),
    Registry(
        "PulseMCP",
        "https://www.pulsemcp.com/",
        "Directory submission/indexing.",
        "Public repository metadata and MCP server install/use documentation.",
        False,
    ),
    Registry(
        "mcp.so",
        "https://mcp.so/",
        "Directory submission/indexing.",
        "Public repository metadata and MCP server install/use documentation.",
        False,
    ),
]


def cargo(*args: str) -> str:
    return subprocess.check_output(["cargo", *args], cwd=ROOT, text=True, stderr=subprocess.STDOUT)


def load_json(path: pathlib.Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    errors: list[str] = []
    server_path = ROOT / "server.json"
    server = load_json(server_path)
    glama_path = ROOT / "glama.json"
    glama = load_json(glama_path)
    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    version = workspace["workspace"]["package"]["version"]

    if server.get("$schema") != "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json":
        errors.append("server.json must use the official MCP Registry schema URL")
    if server.get("name") != "io.github.edithatogo/healthpoint-rs":
        errors.append("server.json name must be io.github.edithatogo/healthpoint-rs for GitHub auth")
    if server.get("version") != version:
        errors.append("server.json version must match workspace.package.version")
    repository = server.get("repository", {})
    if repository.get("url") != "https://github.com/edithatogo/healthpoint-rs" or repository.get("source") != "github":
        errors.append("server.json repository must point to the GitHub source repository")
    packages = server.get("packages", [])
    oci_packages = [pkg for pkg in packages if pkg.get("registryType") == "oci"]
    mcp_pkg = next(
        (
            pkg
            for pkg in oci_packages
            if pkg.get("identifier") == f"ghcr.io/edithatogo/healthpoint-mcp:{version}"
        ),
        None,
    )
    if not mcp_pkg:
        errors.append(
            "server.json packages must include ghcr.io/edithatogo/healthpoint-mcp:<version> with registryType oci"
        )
    elif "version" in mcp_pkg:
        errors.append("healthpoint-mcp OCI package must not use a separate version field")
    elif mcp_pkg.get("transport", {}).get("type") != "stdio":
        errors.append("healthpoint-mcp package transport must be stdio")
    env_vars = {item.get("name"): item for item in (mcp_pkg or {}).get("environmentVariables", [])}
    if not env_vars.get("HEALTHPOINT_API_KEY", {}).get("isSecret"):
        errors.append("HEALTHPOINT_API_KEY must be marked isSecret")
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    if "mcp-name: io.github.edithatogo/healthpoint-rs" not in readme:
        errors.append("README.md must contain visible Cargo ownership token mcp-name: io.github.edithatogo/healthpoint-rs")
    dockerfile = (ROOT / "Dockerfile").read_text(encoding="utf-8")
    if not re.search(
        r"io\.modelcontextprotocol\.server\.name\s*=\s*[\"']io\.github\.edithatogo/healthpoint-rs[\"']",
        dockerfile,
    ):
        errors.append("Dockerfile must set io.modelcontextprotocol.server.name label for OCI ownership validation")
    if "USER mcp" not in dockerfile:
        errors.append("Dockerfile runtime image must run as the non-root mcp user")

    if glama.get("$schema") != "https://glama.ai/mcp/schemas/server.json":
        errors.append("glama.json must use the official Glama schema URL")
    if glama.get("name") != "healthpoint-rs":
        errors.append("glama.json name must be healthpoint-rs")
    if glama.get("displayName") != "Healthpoint MCP Server":
        errors.append("glama.json displayName must be Healthpoint MCP Server")
    if glama.get("license") != "Apache-2.0":
        errors.append("glama.json license must be Apache-2.0")
    if glama.get("repository") != "https://github.com/edithatogo/healthpoint-rs":
        errors.append("glama.json repository must point to the GitHub source repository")
    if glama.get("transport") != "stdio":
        errors.append("glama.json transport must be stdio")
    glama_run = glama.get("run", {})
    glama_env = glama_run.get("env", {})
    if glama_run.get("command") != "healthpoint-mcp":
        errors.append("glama.json run.command must be healthpoint-mcp")
    if glama_env.get("HEALTHPOINT_MODE", {}).get("default") != "synthetic":
        errors.append("glama.json must default HEALTHPOINT_MODE to synthetic")
    if glama_env.get("HEALTHPOINT_API_KEY", {}).get("secret") is not True:
        errors.append("glama.json must mark HEALTHPOINT_API_KEY as secret")
    if glama_env.get("HEALTHPOINT_API_KEY", {}).get("required") is True:
        errors.append("glama.json must keep HEALTHPOINT_API_KEY optional")
    if glama_env.get("HEALTHPOINT_BASE_URL", {}).get("required") is True:
        errors.append("glama.json must keep HEALTHPOINT_BASE_URL optional in synthetic mode")
    install = glama.get("install", {})
    if install.get("command") != "cargo":
        errors.append("glama.json install.command must be cargo")
    if install.get("args") != ["install", "--locked", "--version", "0.1.0", "healthpoint-mcp"]:
        errors.append("glama.json install.args must pin healthpoint-mcp 0.1.0 with --locked")

    metadata = json.loads(cargo("metadata", "--format-version", "1", "--no-deps"))
    names = {pkg["name"] for pkg in metadata["packages"]}
    for required in ["healthpoint-mcp", "healthpoint-cli"]:
        if required not in names:
            errors.append(f"Cargo metadata missing {required}")

    report = {
        "server": server.get("name"),
        "glama": glama.get("name"),
        "version": version,
        "registries": [registry.__dict__ for registry in REGISTRIES],
        "automated_submit_ready": not errors,
        "errors": errors,
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
