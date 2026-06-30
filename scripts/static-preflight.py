#!/usr/bin/env python3
"""Metadata-only preflight checks for environments without Cargo.

This script deliberately avoids Rust compilation. It catches handoff mistakes that
would otherwise waste the first Rust-enabled CI run: malformed JSON/TOML, missing
Conductor pointers, non-executable scripts, secret-shaped fixture accidents, and
obvious Rust delimiter drift.
"""
from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import stat
import sys
import tomllib
from dataclasses import dataclass

ROOT = pathlib.Path(__file__).resolve().parents[1]

SECRET_PATTERNS = [
    re.compile(r"(?i)healthpoint[_-]?api[_-]?key\s*[:=]\s*['\"]?[A-Za-z0-9_\-]{20,}"),
    re.compile(r"(?i)authorization\s*[:=]\s*['\"]?bearer\s+[A-Za-z0-9._\-]{20,}"),
    re.compile(r"hp_live_[A-Za-z0-9_\-]{16,}"),
]

ALLOWED_SECRET_PLACEHOLDERS = {
    "HEALTHPOINT_API_KEY",
    "<your-licensed-key>",
    "${HEALTHPOINT_API_KEY}",
    "...",
}

TEXT_SUFFIXES = {
    ".rs", ".toml", ".json", ".md", ".yml", ".yaml", ".sh", ".py", ".example",
    ".txt", ".gitignore", ".dockerignore", ".editorconfig",
}

@dataclass
class Finding:
    level: str
    path: str
    message: str


def iter_files() -> list[pathlib.Path]:
    files: list[pathlib.Path] = []
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if ".git" in path.parts:
            continue
        if "target" in path.parts:
            continue
        files.append(path)
    return sorted(files)


def rel(path: pathlib.Path) -> str:
    return str(path.relative_to(ROOT))


def check_json_toml(findings: list[Finding]) -> None:
    for path in iter_files():
        if path.suffix == ".json":
            try:
                json.loads(path.read_text(encoding="utf-8"))
            except Exception as exc:  # noqa: BLE001 - report all parse errors
                findings.append(Finding("error", rel(path), f"invalid JSON: {exc}"))
        elif path.suffix == ".toml":
            try:
                tomllib.loads(path.read_text(encoding="utf-8"))
            except Exception as exc:  # noqa: BLE001
                findings.append(Finding("error", rel(path), f"invalid TOML: {exc}"))


def check_workspace_shape(findings: list[Finding]) -> None:
    cargo = ROOT / "Cargo.toml"
    if not cargo.exists():
        findings.append(Finding("error", "Cargo.toml", "workspace Cargo.toml missing"))
        return
    data = tomllib.loads(cargo.read_text(encoding="utf-8"))
    members = data.get("workspace", {}).get("members", [])
    if not members:
        findings.append(Finding("error", "Cargo.toml", "workspace members missing"))
    for member in members:
        member_path = ROOT / member
        if not (member_path / "Cargo.toml").exists():
            findings.append(Finding("error", member, "member Cargo.toml missing"))
        if not ((member_path / "src/lib.rs").exists() or (member_path / "src/main.rs").exists()):
            findings.append(Finding("error", member, "member has no src/lib.rs or src/main.rs"))


def check_conductor(findings: list[Finding]) -> None:
    state_path = ROOT / "conductor/state.json"
    if not state_path.exists():
        findings.append(Finding("error", "conductor/state.json", "missing Conductor state"))
        return
    state = json.loads(state_path.read_text(encoding="utf-8"))
    checkpoint = state.get("latest_checkpoint")
    if not checkpoint:
        findings.append(Finding("error", rel(state_path), "latest_checkpoint missing"))
    elif not (ROOT / checkpoint).exists():
        findings.append(Finding("error", rel(state_path), f"latest_checkpoint does not exist: {checkpoint}"))

    required = [
        "conductor/brief.md",
        "conductor/tracks.md",
        "workflow.md",
        "AGENTS.md",
        "conductor.json",
    ]
    for item in required:
        if not (ROOT / item).exists():
            findings.append(Finding("error", item, "required Conductor context file missing"))

    tracks_dir = ROOT / "conductor/tracks"
    track_files = list(tracks_dir.glob("*.md")) if tracks_dir.exists() else []
    if len(track_files) < 8:
        findings.append(Finding("warning", rel(tracks_dir), "expected at least 8 track files"))


def check_scripts_executable(findings: list[Finding]) -> None:
    for path in [ROOT / "bin/conductor-setup", ROOT / "script/server", ROOT / "bin/mock-healthpoint-server", ROOT / "bin/conductor-status"]:
        if not path.exists():
            findings.append(Finding("error", rel(path), "expected script missing"))
            continue
        mode = path.stat().st_mode
        if not (mode & stat.S_IXUSR):
            findings.append(Finding("warning", rel(path), "script is not executable"))


def text_file(path: pathlib.Path) -> bool:
    if path.suffix in TEXT_SUFFIXES:
        return True
    return path.name in {"justfile", "Dockerfile", "LICENSE", "README.md", "SECURITY.md", "CONTRIBUTING.md"}


def check_secret_shapes(findings: list[Finding]) -> None:
    for path in iter_files():
        if not text_file(path):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for pattern in SECRET_PATTERNS:
            for match in pattern.finditer(text):
                snippet = match.group(0)
                if any(placeholder in snippet for placeholder in ALLOWED_SECRET_PLACEHOLDERS):
                    continue
                findings.append(Finding("error", rel(path), f"secret-shaped value: {snippet[:48]}..."))


def check_fixtures_are_synthetic(findings: list[Finding]) -> None:
    fixtures = ROOT / "crates/healthpoint-testkit/fixtures"
    if not fixtures.exists():
        findings.append(Finding("error", rel(fixtures), "fixtures directory missing"))
        return
    for path in fixtures.rglob("*.json"):
        text = path.read_text(encoding="utf-8")
        if "healthpointapi.com" in text or "healthpoint.co.nz" in text:
            findings.append(Finding("error", rel(path), "fixture appears to contain Healthpoint-origin URL"))
        if "Synthetic" not in text and "example.test" not in text:
            findings.append(Finding("warning", rel(path), "fixture does not clearly identify itself as synthetic"))


def check_server_json(findings: list[Finding]) -> None:
    server = ROOT / "server.json"
    if not server.exists():
        findings.append(Finding("error", "server.json", "missing MCP server manifest"))
        return
    data = json.loads(server.read_text(encoding="utf-8"))
    packages = data.get("packages", [])
    official_env = []
    official_transport = None
    for package in packages:
        identifier = package.get("identifier", "")
        if identifier == "healthpoint-mcp" or identifier.startswith("ghcr.io/edithatogo/healthpoint-mcp:"):
            official_env = package.get("environmentVariables", [])
            official_transport = package.get("transport", {}).get("type")
            break
    if official_env:
        env = {item.get("name"): item for item in official_env}
        api_key = env.get("HEALTHPOINT_API_KEY", {})
        if not api_key.get("isSecret"):
            findings.append(Finding("error", "server.json", "HEALTHPOINT_API_KEY must be marked isSecret"))
        if official_transport != "stdio":
            findings.append(Finding("warning", "server.json", "expected stdio MCP transport"))
        return

    env = data.get("env", {})
    api_key = env.get("HEALTHPOINT_API_KEY", {})
    if not api_key.get("secret"):
        findings.append(Finding("error", "server.json", "HEALTHPOINT_API_KEY must be marked secret"))
    if data.get("transport") != "stdio":
        findings.append(Finding("warning", "server.json", "expected stdio MCP transport"))


def check_rust_delimiters(findings: list[Finding]) -> None:
    pairs = {"(": ")", "[": "]", "{": "}"}
    closing = set(pairs.values())
    for path in ROOT.rglob("*.rs"):
        if ".git" in path.parts or "target" in path.parts:
            continue
        stack: list[tuple[str, int, int]] = []
        in_line_comment = False
        in_block_comment = 0
        in_string = False
        escape = False
        text = path.read_text(encoding="utf-8")
        line = 1
        col = 0
        i = 0
        while i < len(text):
            ch = text[i]
            nxt = text[i + 1] if i + 1 < len(text) else ""
            col += 1
            if ch == "\n":
                line += 1
                col = 0
                in_line_comment = False
                i += 1
                continue
            if in_line_comment:
                i += 1
                continue
            if in_block_comment:
                if ch == "/" and nxt == "*":
                    in_block_comment += 1
                    i += 2
                    continue
                if ch == "*" and nxt == "/":
                    in_block_comment -= 1
                    i += 2
                    continue
                i += 1
                continue
            if in_string:
                if escape:
                    escape = False
                elif ch == "\\":
                    escape = True
                elif ch == '"':
                    in_string = False
                i += 1
                continue
            if ch == "/" and nxt == "/":
                in_line_comment = True
                i += 2
                continue
            if ch == "/" and nxt == "*":
                in_block_comment = 1
                i += 2
                continue
            if ch == '"':
                in_string = True
                i += 1
                continue
            if ch in pairs:
                stack.append((ch, line, col))
            elif ch in closing:
                if not stack or pairs[stack[-1][0]] != ch:
                    findings.append(Finding("error", rel(path), f"unmatched {ch!r} at {line}:{col}"))
                    break
                stack.pop()
            i += 1
        if stack:
            opener, opener_line, opener_col = stack[-1]
            findings.append(Finding("error", rel(path), f"unclosed {opener!r} from {opener_line}:{opener_col}"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--warnings-as-errors", action="store_true")
    args = parser.parse_args()

    findings: list[Finding] = []
    check_json_toml(findings)
    check_workspace_shape(findings)
    check_conductor(findings)
    check_scripts_executable(findings)
    check_secret_shapes(findings)
    check_fixtures_are_synthetic(findings)
    check_server_json(findings)
    check_rust_delimiters(findings)

    for finding in findings:
        print(f"{finding.level.upper()}: {finding.path}: {finding.message}")

    errors = [f for f in findings if f.level == "error" or (args.warnings_as_errors and f.level == "warning")]
    if errors:
        print(f"preflight failed: {len(errors)} blocking finding(s)")
        return 1
    print("metadata-only preflight passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
