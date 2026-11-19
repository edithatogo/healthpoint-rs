#!/usr/bin/env python3
"""Refuse to publish a checkout's inventory under an existing different source tag."""

import argparse
import json
import re
import subprocess


def check_source(tag, run=subprocess.run):
    def git(*args):
        return run(["git", *args], text=True, capture_output=True, check=False, timeout=120)

    ref = "refs/tags/" + tag
    if git("check-ref-format", ref).returncode:
        raise ValueError("invalid release tag")
    current = git("rev-parse", "--verify", "HEAD^{commit}")
    source = current.stdout.strip()
    if current.returncode or not re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", source):
        raise ValueError("cannot identify exact checked-out source commit")
    lookup = git("ls-remote", "--exit-code", "--refs", "origin", ref)
    if lookup.returncode == 2:
        return {"source_commit": source, "tag": tag, "existing_tag": False}
    if lookup.returncode != 0:
        raise ValueError("release tag lookup failed; absence has not been established")
    rows = [line.split() for line in lookup.stdout.splitlines() if line.strip()]
    if len(rows) != 1 or len(rows[0]) != 2 or rows[0][1] != ref or not re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", rows[0][0]):
        raise ValueError("release tag lookup returned an ambiguous identity")
    # Fetch the observed immutable tag object, not a second mutable-name lookup.
    if git("fetch", "--no-tags", "--depth=1", "origin", rows[0][0]).returncode:
        raise ValueError("cannot fetch the observed release tag object")
    peeled = git("rev-parse", "--verify", "FETCH_HEAD^{commit}")
    if peeled.returncode or peeled.stdout.strip() != source:
        raise ValueError("existing release tag targets a different source commit; choose that source or a new release version")
    return {"source_commit": source, "tag": tag, "existing_tag": True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tag", required=True)
    args = parser.parse_args()
    try:
        print(json.dumps(check_source(args.tag), sort_keys=True))
    except (ValueError, OSError, subprocess.TimeoutExpired) as error:
        parser.exit(1, f"release source check failed: {error}\n")


if __name__ == "__main__":
    main()
