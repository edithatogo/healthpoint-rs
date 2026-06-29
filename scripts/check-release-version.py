#!/usr/bin/env python3
"""Validate release version consistency before tagging or publishing."""
from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def fail(message: str) -> None:
    print(f"release-version: {message}", file=sys.stderr)
    sys.exit(1)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tag", help="Expected release tag, e.g. v0.1.0")
    args = parser.parse_args()

    cargo = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    version = cargo.get("workspace", {}).get("package", {}).get("version")
    if not version:
        fail("workspace.package.version is missing")

    if args.tag and args.tag != f"v{version}":
        fail(f"tag {args.tag!r} does not match workspace version v{version}")

    for manifest in sorted((ROOT / "crates").glob("*/Cargo.toml")):
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        package = data.get("package", {})
        if package.get("version") != {"workspace": True}:
            fail(f"{manifest.relative_to(ROOT)} must inherit version.workspace = true")

    server = json.loads((ROOT / "server.json").read_text(encoding="utf-8"))
    if server.get("version") != version:
        fail(f"server.json version {server.get('version')!r} does not match {version}")

    changelog = (ROOT / "CHANGELOG.md").read_text(encoding="utf-8")
    changelog_headings = [line.strip() for line in changelog.splitlines() if line.startswith("## ")]
    if not any(heading == f"## v{version}" or heading.startswith(f"## v{version} -") or heading.startswith(f"## [{version}]") for heading in changelog_headings):
        fail(f"CHANGELOG.md has no section for {version}")

    print(f"release-version: v{version} is consistent")


if __name__ == "__main__":
    main()
