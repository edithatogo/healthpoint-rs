#!/usr/bin/env python3
"""Static Smithery/MCPB compliance checks for healthpoint-rs."""
from __future__ import annotations

import json
import re
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
METADATA = ROOT / "packaging" / "mcpb" / "manifest-metadata.json"
README = ROOT / "README.md"
PACKAGE_SCRIPT = ROOT / "scripts" / "package-mcpb"
REPORT_DIR = ROOT / "target" / "mcp-validation"

EXPECTED_TOOLS = {
    "healthpoint_diagnostic_status",
    "healthpoint_api_access_notes",
    "healthpoint_access_policy",
    "healthpoint_search_services",
    "healthpoint_search_by_snomed",
    "healthpoint_find_nearby_services",
    "healthpoint_get_service",
    "healthpoint_get_location",
    "healthpoint_get_organization",
    "healthpoint_read_resource_uri",
}
EXPECTED_RESOURCES = {
    "healthpoint://diagnostic/status",
    "healthpoint://api/access-notes",
    "healthpoint://access/policy",
}
EXPECTED_RESOURCE_TEMPLATES = {
    "healthpoint://service/{id}",
    "healthpoint://location/{id}",
    "healthpoint://organization/{id}",
    "healthpoint://query/services?text={text}&region={region}&limit={limit}",
}
EXPECTED_PROMPTS = {"healthpoint_safe_search", "healthpoint_license_check"}
SECRET_PATTERNS = [re.compile(r"HEALTHPOINT_API_KEY=[A-Za-z0-9_\-]{12,}"), re.compile(r"sk-[A-Za-z0-9]{20,}")]


def fail(msg: str) -> None:
    print(f"FAIL: {msg}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except Exception as exc:  # noqa: BLE001
        fail(f"{path} is not valid JSON: {exc}")


def validate_metadata() -> None:
    data = load_json(METADATA)
    tools = {tool.get("name") for tool in data.get("tools", [])}
    resources = {res.get("uri") for res in data.get("resources", [])}
    templates = {tpl.get("uriTemplate") for tpl in data.get("resource_templates", [])}
    prompts = {prompt.get("name") for prompt in data.get("prompts", [])}
    if tools != EXPECTED_TOOLS:
        fail(f"tool metadata mismatch: missing={sorted(EXPECTED_TOOLS - tools)} extra={sorted(tools - EXPECTED_TOOLS)}")
    if resources != EXPECTED_RESOURCES:
        fail("resource metadata mismatch")
    if templates != EXPECTED_RESOURCE_TEMPLATES:
        fail("resource template metadata mismatch")
    if prompts != EXPECTED_PROMPTS:
        fail("prompt metadata mismatch")
    for tool in data["tools"]:
        if not tool.get("description"):
            fail(f"tool {tool.get('name')} missing description")
        schema = tool.get("inputSchema")
        if not isinstance(schema, dict) or schema.get("type") != "object":
            fail(f"tool {tool.get('name')} missing object inputSchema")
        if not isinstance(tool.get("outputSchema"), dict):
            fail(f"tool {tool.get('name')} missing outputSchema")
    api_key = data.get("user_config", {}).get("healthpoint_api_key", {})
    if not api_key.get("sensitive") or not api_key.get("required"):
        fail("HEALTHPOINT_API_KEY must be required and sensitive")


def validate_readme() -> None:
    text = README.read_text()
    badge = "[![smithery badge](https://smithery.ai/badge/edithatogo/healthpoint-rs)](https://smithery.ai/servers/edithatogo/healthpoint-rs)"
    if badge not in text:
        fail("README is missing Smithery badge backlink")
    if "mcp-name: io.github.edithatogo/healthpoint-rs" not in text:
        fail("README is missing official MCP registry ownership marker")


def validate_script() -> None:
    if not PACKAGE_SCRIPT.exists() or not PACKAGE_SCRIPT.stat().st_mode & 0o111:
        fail("scripts/package-mcpb must exist and be executable")


def validate_bundle(path: Path) -> None:
    if not path.exists():
        fail(f"bundle does not exist: {path}")
    with zipfile.ZipFile(path) as bundle:
        names = set(bundle.namelist())
        required = {"manifest.json", "README.md", "LICENSE", "server.json"}
        if not required.issubset(names):
            fail(f"bundle missing required files: {sorted(required - names)}")
        manifest = json.loads(bundle.read("manifest.json"))
        if manifest.get("manifest_version") != "0.3":
            fail("bundle manifest_version must be 0.3")
        tool_names = {tool.get("name") for tool in manifest.get("tools", [])}
        if tool_names != EXPECTED_TOOLS:
            fail("bundle tool list does not match expected MCP tools")
        prompt_names = {prompt.get("name") for prompt in manifest.get("prompts", [])}
        if prompt_names != EXPECTED_PROMPTS:
            fail("bundle prompt list does not match expected MCP prompts")
        meta = manifest.get("_meta", {}).get("io.modelcontextprotocol", {})
        resource_uris = {res.get("uri") for res in meta.get("resources", [])}
        template_uris = {tpl.get("uriTemplate") for tpl in meta.get("resourceTemplates", [])}
        if resource_uris != EXPECTED_RESOURCES or template_uris != EXPECTED_RESOURCE_TEMPLATES:
            fail("bundle resource metadata does not match expected MCP resources/templates")
        for name in names:
            if name.endswith(('.json', '.md', '.txt', 'server.json')):
                content = bundle.read(name).decode('utf-8', errors='ignore')
                for pattern in SECRET_PATTERNS:
                    if pattern.search(content):
                        fail(f"possible secret pattern found in bundle file {name}")


def main() -> int:
    validate_metadata()
    validate_readme()
    validate_script()
    bundles = sorted((ROOT / "target" / "mcpb").glob("healthpoint-rs-*.mcpb"))
    if bundles:
        for bundle in bundles:
            validate_bundle(bundle)
    report = REPORT_DIR / "healthpoint-mcp-validation.json"
    if report.exists():
        load_json(report)
    print("Smithery compliance static checks passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
