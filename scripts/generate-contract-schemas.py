#!/usr/bin/env python3
"""Generate conservative hand-authored JSON Schemas for public contracts.

The CLI's `healthpoint schema ...` command is the canonical schema source after
Rust compilation. These generated files are interim contracts for documentation,
mocking, and MCP/client integration planning in non-Rust environments.
"""
from __future__ import annotations

import json
import pathlib
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / "contracts/schemas"
OUT.mkdir(parents=True, exist_ok=True)

DRAFT = "https://json-schema.org/draft/2020-12/schema"


def obj(title: str, props: dict[str, Any], required: list[str] | None = None, **extra: Any) -> dict[str, Any]:
    schema: dict[str, Any] = {
        "$schema": DRAFT,
        "$id": f"https://github.com/edithatogo/healthpoint-rs/contracts/schemas/{title}.schema.json",
        "title": title,
        "type": "object",
        "additionalProperties": True,
        "properties": props,
    }
    if required:
        schema["required"] = required
    schema.update(extra)
    return schema

string = {"type": "string"}
nullable_string = {"anyOf": [{"type": "string"}, {"type": "null"}]}
code = {
    "type": "object",
    "additionalProperties": True,
    "required": ["code"],
    "properties": {
        "system": nullable_string,
        "code": string,
        "display": nullable_string,
    },
}
ref = {
    "type": "object",
    "additionalProperties": True,
    "required": ["reference"],
    "properties": {"reference": string, "display": nullable_string},
}
contact = {
    "type": "object",
    "additionalProperties": True,
    "properties": {"system": nullable_string, "value": nullable_string, "use_code": nullable_string},
}
identifier = {
    "type": "object",
    "additionalProperties": True,
    "properties": {"use_code": nullable_string, "system": nullable_string, "value": nullable_string},
}
provenance = {
    "type": "object",
    "additionalProperties": True,
    "properties": {
        "source_name": {"type": "string"},
        "source_url": nullable_string,
        "retrieved_at": {"type": "string"},
        "access_mode": {"type": "string"},
        "redistribution": {"type": "string"},
    },
}

schemas: dict[str, dict[str, Any]] = {}

schemas["service-query"] = obj("service-query", {
    "text": nullable_string,
    "categories": {"type": "array", "items": code},
    "service_types": {"type": "array", "items": code},
    "specialties": {"type": "array", "items": code},
    "nearby": {"anyOf": [{"type": "object", "properties": {"lat": {"type": "number", "minimum": -90, "maximum": 90}, "lon": {"type": "number", "minimum": -180, "maximum": 180}}, "required": ["lat", "lon"]}, {"type": "null"}]},
    "radius_km": {"anyOf": [{"type": "number", "exclusiveMinimum": 0, "maximum": 500}, {"type": "null"}]},
    "limit": {"anyOf": [{"type": "integer", "minimum": 1, "maximum": 100}, {"type": "object", "properties": {"0": {"type": "integer", "minimum": 1, "maximum": 100}}}]},
    "cursor": nullable_string,
})

schemas["service-record"] = obj("service-record", {
    "id": string,
    "identifiers": {"type": "array", "items": identifier},
    "name": nullable_string,
    "active": {"anyOf": [{"type": "boolean"}, {"type": "null"}]},
    "provided_by": {"anyOf": [ref, {"type": "null"}]},
    "locations": {"type": "array", "items": ref},
    "coverage_areas": {"type": "array", "items": ref},
    "endpoints": {"type": "array", "items": ref},
    "categories": {"type": "array", "items": code},
    "service_types": {"type": "array", "items": code},
    "specialties": {"type": "array", "items": code},
    "service_provision_codes": {"type": "array", "items": code},
    "programs": {"type": "array", "items": code},
    "characteristics": {"type": "array", "items": code},
    "communications": {"type": "array", "items": code},
    "referral_methods": {"type": "array", "items": code},
    "eligibilities": {"type": "array"},
    "appointment_required": {"anyOf": [{"type": "boolean"}, {"type": "null"}]},
    "comment": nullable_string,
    "extra_details": nullable_string,
    "available_times": {"type": "array"},
    "not_available": {"type": "array"},
    "contacts": {"type": "array", "items": contact},
    "provenance": provenance,
    "raw_fhir": {"type": "object"},
}, required=["id", "provenance", "raw_fhir"])

schemas["location-record"] = obj("location-record", {
    "id": string,
    "identifiers": {"type": "array", "items": identifier},
    "name": nullable_string,
    "status": nullable_string,
    "mode": nullable_string,
    "location_types": {"type": "array", "items": code},
    "physical_types": {"type": "array", "items": code},
    "contacts": {"type": "array", "items": contact},
    "address": {"anyOf": [{"type": "object"}, {"type": "null"}]},
    "position": {"anyOf": [{"type": "object", "properties": {"latitude": {"type": "number"}, "longitude": {"type": "number"}, "altitude": {"anyOf": [{"type": "number"}, {"type": "null"}]}}, "required": ["latitude", "longitude"]}, {"type": "null"}]},
    "managing_organization": {"anyOf": [ref, {"type": "null"}]},
    "part_of": {"anyOf": [ref, {"type": "null"}]},
    "endpoints": {"type": "array", "items": ref},
    "hours_of_operation": {"type": "array"},
    "provenance": provenance,
    "raw_fhir": {"type": "object"},
}, required=["id", "provenance", "raw_fhir"])

schemas["organization-record"] = obj("organization-record", {
    "id": string,
    "identifiers": {"type": "array", "items": identifier},
    "organization_types": {"type": "array", "items": code},
    "name": nullable_string,
    "aliases": {"type": "array", "items": string},
    "active": {"anyOf": [{"type": "boolean"}, {"type": "null"}]},
    "part_of": {"anyOf": [ref, {"type": "null"}]},
    "endpoints": {"type": "array", "items": ref},
    "contacts": {"type": "array", "items": contact},
    "provenance": provenance,
    "raw_fhir": {"type": "object"},
}, required=["id", "provenance", "raw_fhir"])

schemas["access-policy"] = obj("access-policy", {
    "access_mode": {"enum": ["public", "bring_your_own_key", "licensed", "unknown"]},
    "redistribution": {"enum": ["allowed", "prohibited", "unknown", "requires_review"]},
    "export_policy": {"enum": ["local_only", "licensed_share", "open_approved"]},
    "notes": {"type": "array", "items": string},
})

schemas["export-manifest"] = obj("export-manifest", {
    "manifest_version": {"type": "string"},
    "tool": {"type": "string"},
    "tool_version": {"type": "string"},
    "created_at": {"type": "string"},
    "contains_healthpoint_data": {"type": "boolean"},
    "provenance": provenance,
    "warnings": {"type": "array", "items": string},
})

schemas["resource-uri"] = {
    "$schema": DRAFT,
    "$id": "https://github.com/edithatogo/healthpoint-rs/contracts/schemas/resource-uri.schema.json",
    "title": "resource-uri",
    "type": "string",
    "pattern": r"^healthpoint://(service|location|organization)/[A-Za-z0-9][A-Za-z0-9.\-]{0,127}$",
}

schemas["mcp-tools"] = obj("mcp-tools", {
    "tools": {"type": "array", "items": {"type": "object", "required": ["name", "read_only"], "properties": {"name": string, "read_only": {"const": True}, "params_schema": {"type": "object"}}}},
})

for name, schema in schemas.items():
    (OUT / f"{name}.schema.json").write_text(json.dumps(schema, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(f"wrote {len(schemas)} schemas to {OUT.relative_to(ROOT)}")
