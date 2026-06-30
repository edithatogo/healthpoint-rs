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
        "Public repository metadata and MCP server install/use documentation.",
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

GLAMA_SCORE_CONTRACT = {
    "source": "https://glama.ai/mcp/servers/edithatogo/healthpoint-rs/score",
    "observed_at": "2026-06-30",
    "observed_profile_score_percent": 83,
    "latest_release": "v0.1.0",
    "profile_checks": {
        "has_glama_release": True,
        "maintenance_grade": "A",
        "license": "Apache 2.0",
        "has_readme": True,
        "active_usage_30d": 2,
        "author_verified": True,
        "glama_json_visible_on_glama": False,
        "related_servers_configured": False,
    },
    "quality_formula": {
        "overall": "70% Tool Definition Quality + 30% Server Coherence",
        "tool_definition_quality": "60% mean TDQS + 40% minimum TDQS",
        "tool_definition_dimensions": {
            "purpose_clarity": 0.25,
            "usage_guidelines": 0.20,
            "behavioral_transparency": 0.20,
            "parameter_semantics": 0.15,
            "conciseness_structure": 0.10,
            "contextual_completeness": 0.10,
        },
        "server_coherence_dimensions": [
            "disambiguation",
            "naming_consistency",
            "tool_count_appropriateness",
            "completeness",
        ],
    },
    "server_coherence": {
        "grade": "A",
        "disambiguation": 5,
        "naming_consistency": 5,
        "tool_count": 5,
        "completeness": 5,
    },
    "tool_definition_quality": {
        "grade": "A",
        "average": 3.7,
        "tool_count_scored": 10,
        "lowest": 3.1,
        "visible_low_score_reasons": [
            "Descriptions often omit usage guidance and alternatives.",
            "Descriptions often rely on read-only wording but omit auth, error, rate-limit, pagination, and return-shape behavior.",
            "Complex search tools do not describe filter interaction, return format, or pagination enough for first-attempt use.",
            "Simple get-by-id tools do not clearly distinguish when to use them instead of search or sibling get tools.",
            "Glama reports no tool annotations, so descriptions carry the behavioral-disclosure burden.",
        ],
    },
    "local_status": {
        "glama_json_present": True,
        "glama_json_not_yet_visible_until_main_merge_or_sync": True,
        "related_servers_are_external_directory_metadata": True,
    },
}


def cargo(*args: str) -> str:
    return subprocess.check_output(["cargo", *args], cwd=ROOT, text=True, stderr=subprocess.STDOUT)


def main() -> int:
    errors: list[str] = []
    server_path = ROOT / "server.json"
    server = json.loads(server_path.read_text(encoding="utf-8"))
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
    glama_path = ROOT / "glama.json"
    if not glama_path.exists():
        errors.append("glama.json must exist so Glama can ingest repository metadata after main sync")
    else:
        glama = json.loads(glama_path.read_text(encoding="utf-8"))
        if glama.get("$schema") != "https://glama.ai/mcp/schemas/server.json":
            errors.append("glama.json must use Glama's server schema URL")
        if glama.get("repository") != "https://github.com/edithatogo/healthpoint-rs":
            errors.append("glama.json repository must point to the public GitHub repository")
        if glama.get("license") != "Apache-2.0":
            errors.append("glama.json license must be Apache-2.0")
        if glama.get("transport") != "stdio":
            errors.append("glama.json transport must be stdio")
        if not glama.get("quality", {}).get("readOnly"):
            errors.append("glama.json quality.readOnly must be true")
        run_env = glama.get("run", {}).get("env", {})
        if not run_env.get("HEALTHPOINT_API_KEY", {}).get("secret"):
            errors.append("glama.json must mark HEALTHPOINT_API_KEY as secret")

    metadata = json.loads(cargo("metadata", "--format-version", "1", "--no-deps"))
    names = {pkg["name"] for pkg in metadata["packages"]}
    for required in ["healthpoint-mcp", "healthpoint-cli"]:
        if required not in names:
            errors.append(f"Cargo metadata missing {required}")

    report = {
        "server": server.get("name"),
        "version": version,
        "registries": [registry.__dict__ for registry in REGISTRIES],
        "glama_score_contract": GLAMA_SCORE_CONTRACT,
        "automated_submit_ready": not errors,
        "errors": errors,
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
