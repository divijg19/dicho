"""Command line interface for the Python data package."""

from __future__ import annotations

import argparse
import json
from collections import Counter

from . import __version__
from .paths import repo_root
from .schema import SCHEMA_VERSION, validate_file


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="dicho-data",
        description="Prepare and validate data for dicho",
    )
    parser.add_argument(
        "--version",
        action="version",
        version=f"dicho-data {__version__}",
    )
    commands = parser.add_subparsers(dest="command", required=True)

    validate = commands.add_parser(
        "validate",
        help="validate the curated entry collection",
    )
    validate.add_argument(
        "--data",
        type=str,
        help="path to entries.jsonl, defaulting to the repository copy",
    )
    validate.add_argument(
        "--json",
        action="store_true",
        help="emit the full report as JSON",
    )
    validate.add_argument(
        "--strict",
        action="store_true",
        help="fail when quality findings are present",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)

    if args.command == "validate":
        from pathlib import Path

        path = Path(args.data) if args.data else repo_root() / "data" / "curated" / "entries.jsonl"
        report = validate_file(path)

        if args.json:
            print(json.dumps(report.to_dict(), indent=2, ensure_ascii=False))
        else:
            print(f"Source: {report.source}")
            print(f"Schema version: {SCHEMA_VERSION}")
            print(f"Entries: {len(report.entries)}")
            print(f"Errors: {len(report.errors)}")
            for finding in report.errors:
                print(f"  line {finding.line} {finding.entry_id} {finding.kind}: {finding.message}")
            counts = Counter(finding.kind for finding in report.findings)
            print("Findings:")
            for kind, count in sorted(counts.items()):
                print(f"  {kind}: {count}")

        if report.errors:
            return 1
        if args.strict and report.findings:
            return 1
        return 0

    return 2
