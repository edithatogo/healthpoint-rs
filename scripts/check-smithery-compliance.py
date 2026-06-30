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
CONTRACT = ROOT / "docs" / "smithery-compliance-contract.md"
SMITHERY_YAML = ROOT / "smithery.yaml"
SERVER_JSON = ROOT / "server.json"
REPORT_DIR = ROOT / "target" / "mcp-validation"

EXPECTED_TOOLS = {
    "healthpoint.diagnostic.status",
    "healthpoint.access.notes",
    "healthpoint.access.policy",
    "healthpoint.services.search",
    "healthpoint.services.search_snomed",
    "healthpoint.services.nearby",
    "healthpoint.service.get",
    "healthpoint.location.get",
    "healthpoint.organization.get",
    "healthpoint.resource.read",
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
REQUIRED_README_SECTIONS = ["## Tools", "## Installation", "## Usage", "## Configuration", "## Safety boundary"]
SECRET_PATTERNS = [
    re.compile(r"HEALTHPOINT_API_KEY=[A-Za-z0-9_\-]{12,}"),
    re.compile(r"sk-[A-Za-z0-9]{20,}"),
]


def fail(msg: str) -> None:
    print(f"FAIL: {msg}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except Exception as exc:  # noqa: BLE001
        fail(f"{path} is not valid JSON: {exc}")


def validate_tool_schema(tool: dict) -> None:
    name = tool.get("name")
    if not name or "." not in name or "_" in name.split(".", 1)[0]:
        fail(f"tool {name!r} does not use Smithery-friendly dot notation")
    if not tool.get("description") or len(tool["description"]) < 40:
        fail(f"tool {name} missing meaningful description")
    annotations = tool.get("annotations")
    if not isinstance(annotations, dict) or annotations.get("readOnlyHint") is not True:
        fail(f"tool {name} missing readOnlyHint annotation")
    if annotations.get("destructiveHint") is not False:
        fail(f"tool {name} must explicitly set destructiveHint false")
    schema = tool.get("inputSchema")
    if not isinstance(schema, dict) or schema.get("type") != "object":
        fail(f"tool {name} missing object inputSchema")
    for param_name, param_schema in schema.get("properties", {}).items():
        if not isinstance(param_schema, dict) or not param_schema.get("description"):
            fail(f"tool {name} parameter {param_name} missing description")
    output = tool.get("outputSchema")
    if not isinstance(output, dict) or not output.get("description"):
        fail(f"tool {name} missing descriptive outputSchema")


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
        validate_tool_schema(tool)
    config = data.get("user_config", {})
    api_key = config.get("healthpoint_api_key", {})
    if not api_key.get("sensitive") or api_key.get("required"):
        fail("HEALTHPOINT_API_KEY must be optional and sensitive for Smithery Configuration UX")
    mode = config.get("healthpoint_mode", {})
    if mode.get("default") != "synthetic" or mode.get("required"):
        fail("healthpoint_mode must default to optional synthetic mode")
    if any(value.get("required") for value in config.values() if isinstance(value, dict)):
        fail("Smithery user_config must not require fields")


def validate_listing_metadata() -> None:
    server = load_json(SERVER_JSON)
    for field in ["description", "repository", "websiteUrl", "license", "homepage", "icons"]:
        if not server.get(field):
            fail(f"server.json missing {field}")
    env_vars = server.get("packages", [{}])[0].get("environmentVariables", [])
    api_key = next((item for item in env_vars if item.get("name") == "HEALTHPOINT_API_KEY"), {})
    mode = next((item for item in env_vars if item.get("name") == "HEALTHPOINT_MODE"), {})
    if api_key.get("isRequired") is not False or api_key.get("isSecret") is not True:
        fail("server.json HEALTHPOINT_API_KEY must be optional and secret")
    if mode.get("default") != "synthetic":
        fail("server.json HEALTHPOINT_MODE must default to synthetic")


def validate_smithery_yaml() -> None:
    if not SMITHERY_YAML.exists():
        fail("root smithery.yaml is missing")
    text = SMITHERY_YAML.read_text()
    required = ["startCommand:", "type: stdio", "configSchema:", "commandFunction:", "healthpoint_mode", "healthpoint_api_key", "sensitive: true", "synthetic", "live"]
    missing = [phrase for phrase in required if phrase not in text]
    if missing:
        fail(f"smithery.yaml missing required phrases: {missing}")
    if re.search(r"required\s*:", text):
        fail("smithery.yaml must not declare required config fields")


def validate_readme() -> None:
    text = README.read_text()
    badge = "[![smithery badge](https://smithery.ai/badge/edithatogo/healthpoint-rs)](https://smithery.ai/servers/edithatogo/healthpoint-rs)"
    if badge not in text:
        fail("README is missing Smithery badge backlink")
    if "mcp-name: io.github.edithatogo/healthpoint-rs" not in text:
        fail("README is missing official MCP registry ownership marker")
    missing_sections = [section for section in REQUIRED_README_SECTIONS if section not in text]
    if missing_sections:
        fail(f"README missing Smithery-rendered sections: {missing_sections}")
    for tool in EXPECTED_TOOLS:
        if tool not in text:
            fail(f"README missing tool {tool}")
    if "https://smithery.ai/servers/edithatogo/healthpoint-rs" not in text:
        fail("README missing Smithery install/listing link")


def validate_contract() -> None:
    if not CONTRACT.exists():
        fail("docs/smithery-compliance-contract.md is missing")
    text = CONTRACT.read_text()
    required_phrases = [
        "Capability Quality",
        "Server Metadata",
        "Configuration UX",
        "Parameter descriptions",
        "Tool annotations",
        "Optional configuration",
        "Smithery build docs",
        "Smithery publish docs",
        "Smithery triggers docs",
        "SlowMist MCP Security Checklist",
        "MCP-Manager checklist themes",
        "implemented",
        "not_applicable",
        "external_gate",
        "10 tools",
        "3 resources",
        "4 resource templates",
        "2 prompts",
    ]
    missing = [phrase for phrase in required_phrases if phrase not in text]
    if missing:
        fail(f"contract is missing required phrases: {missing}")


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
        for tool in manifest.get("tools", []):
            validate_tool_schema(tool)
        prompt_names = {prompt.get("name") for prompt in manifest.get("prompts", [])}
        if prompt_names != EXPECTED_PROMPTS:
            fail("bundle prompt list does not match expected MCP prompts")
        resource_uris = {res.get("uri") for res in manifest.get("resources", [])}
        template_uris = {tpl.get("uriTemplate") for tpl in manifest.get("resourceTemplates", [])}
        if resource_uris != EXPECTED_RESOURCES or template_uris != EXPECTED_RESOURCE_TEMPLATES:
            fail("bundle top-level resource metadata does not match expected MCP resources/templates")
        api_key = manifest.get("user_config", {}).get("healthpoint_api_key", {})
        if api_key.get("required") or api_key.get("sensitive") is not True:
            fail("bundle user_config must keep healthpoint_api_key optional and sensitive")
        for name in names:
            if name.endswith((".json", ".md", ".txt", "server.json")):
                content = bundle.read(name).decode("utf-8", errors="ignore")
                for pattern in SECRET_PATTERNS:
                    if pattern.search(content):
                        fail(f"possible secret pattern found in bundle file {name}")


def main() -> int:
    validate_metadata()
    validate_listing_metadata()
    validate_smithery_yaml()
    validate_readme()
    validate_contract()
    validate_script()
    bundles = sorted((ROOT / "target" / "mcpb").glob("healthpoint-rs-*.mcpb"), key=lambda path: path.stat().st_mtime)
    if bundles:
        validate_bundle(bundles[-1])
    report = REPORT_DIR / "healthpoint-mcp-validation.json"
    if report.exists():
        load_json(report)
    print("Smithery compliance static checks passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
