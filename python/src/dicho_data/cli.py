"""Command line interface for the Python data package."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path

from . import __version__
from .paths import data_path, repo_root
from .pipeline import (
    build_report,
    export_corpus,
    import_raw,
    normalise_candidates,
    regenerate,
)
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

    import_command = commands.add_parser(
        "import",
        help="normalise raw source excerpts into the imported layer",
    )
    import_command.add_argument("--raw", type=str, help="raw directory")
    import_command.add_argument("--output", type=str, help="imported directory")

    normalise = commands.add_parser(
        "normalise",
        help="build deduplicated candidates from imported rows",
    )
    normalise.add_argument("--input", type=str, help="imported directory")
    normalise.add_argument("--curated", type=str, help="curated entries.jsonl")
    normalise.add_argument("--output", type=str, help="processed candidates.jsonl")

    report = commands.add_parser(
        "report",
        help="write the quality report artifact",
    )
    report.add_argument("--curated", type=str, help="curated entries.jsonl")
    report.add_argument("--processed", type=str, help="processed candidates.jsonl")
    report.add_argument("--output", type=str, help="quality report path")

    export = commands.add_parser(
        "export",
        help="export the curated collection as a corpus artifact",
    )
    export.add_argument("--curated", type=str, help="curated entries.jsonl")
    export.add_argument("--output", type=str, help="corpus path")

    regenerate_command = commands.add_parser(
        "regenerate",
        help="run every derived stage in order",
    )
    regenerate_command.add_argument("--root", type=str, help="repository root")

    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)

    if args.command == "validate":
        path = Path(args.data) if args.data else data_path("curated") / "entries.jsonl"
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

    if args.command == "import":
        raw = Path(args.raw) if args.raw else data_path("raw")
        output = Path(args.output) if args.output else data_path("imported")
        written = import_raw(raw, output)
        print(f"Imported {len(written)} source file(s)")
        return 0

    if args.command == "normalise":
        imported = Path(args.input) if args.input else data_path("imported")
        curated = Path(args.curated) if args.curated else data_path("curated") / "entries.jsonl"
        output = Path(args.output) if args.output else data_path("processed") / "candidates.jsonl"
        candidates = normalise_candidates(imported, curated, output)
        duplicates = sum(candidate["status"] == "duplicate" for candidate in candidates)
        print(f"Candidates: {len(candidates)}, duplicates: {duplicates}")
        return 0

    if args.command == "report":
        curated = Path(args.curated) if args.curated else data_path("curated") / "entries.jsonl"
        processed = Path(args.processed) if args.processed else data_path("processed") / "candidates.jsonl"
        output = Path(args.output) if args.output else data_path("artifacts") / "quality-report.json"
        payload = build_report(curated, processed, None, output)
        print(f"Report: {output}, errors: {payload['errors']}")
        return 0

    if args.command == "export":
        curated = Path(args.curated) if args.curated else data_path("curated") / "entries.jsonl"
        output = Path(args.output) if args.output else data_path("artifacts") / "corpus.json"
        payload = export_corpus(curated, output)
        print(f"Exported {len(payload['entries'])} entries")
        return 0

    if args.command == "regenerate":
        root = Path(args.root) if args.root else repo_root()
        summary = regenerate(root)
        print(
            "Regenerated: "
            f"{summary['imported_files']} imported file(s), "
            f"{summary['candidates']} candidates, "
            f"{summary['entries']} entries"
        )
        return 0

    return 2
