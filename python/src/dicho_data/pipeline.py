"""Reproducible stages from raw source material to derived data artifacts."""

from __future__ import annotations

import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any

from .paths import data_path, repo_root
from .schema import SCHEMA_VERSION, validate_file
from .text import fold_spanish, normalise


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_text(path.read_text(encoding="utf-8"))


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    text = "".join(
        json.dumps(row, ensure_ascii=False, sort_keys=True) + "\n" for row in rows
    )
    path.write_text(text, encoding="utf-8")


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    text = json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    path.write_text(text, encoding="utf-8")


def relative_path(path: Path) -> str:
    """Render a path relative to the repository when possible."""
    try:
        return str(path.resolve().relative_to(repo_root().resolve()))
    except ValueError:
        return path.name


def import_raw(raw_dir: Path | None = None, output_dir: Path | None = None) -> list[Path]:
    """Normalise raw source excerpts into the imported layer."""
    raw_dir = raw_dir or data_path("raw")
    output_dir = output_dir or data_path("imported")
    written: list[Path] = []

    for source in sorted(raw_dir.glob("*.jsonl")):
        rows = []
        for record in read_jsonl(source):
            text = normalise(str(record.get("text", "")))
            folded = fold_spanish(text)
            rows.append(
                {
                    "id": "sb-" + sha256_text(folded)[:12],
                    "es": text,
                    "folded": folded,
                    "source": str(record.get("source", "")),
                    "license": str(record.get("license", "")),
                    "locator": str(record.get("locator", "")),
                    "access_date": str(record.get("access_date", "")),
                    "source_file": source.name,
                }
            )
        rows.sort(key=lambda row: row["id"])
        destination = output_dir / source.name
        write_jsonl(destination, rows)
        written.append(destination)

    return written


def normalise_candidates(
    imported_dir: Path | None = None,
    curated_path: Path | None = None,
    output_path: Path | None = None,
) -> list[dict[str, Any]]:
    """Build deduplicated candidates from imported rows."""
    imported_dir = imported_dir or data_path("imported")
    curated_path = curated_path or data_path("curated") / "entries.jsonl"
    output_path = output_path or data_path("processed") / "candidates.jsonl"

    curated_texts = {
        fold_spanish(str(entry.get("es", ""))) for entry in read_jsonl(curated_path)
    }
    seen: set[str] = set()
    candidates: list[dict[str, Any]] = []

    for imported in sorted(imported_dir.glob("*.jsonl")):
        for row in read_jsonl(imported):
            folded = str(row.get("folded") or fold_spanish(str(row.get("es", ""))))
            duplicate_of = folded if folded in curated_texts else None
            if duplicate_of is None and folded in seen:
                duplicate_of = folded
            seen.add(folded)
            candidates.append(
                {
                    "id": str(row.get("id", "")),
                    "es": str(row.get("es", "")),
                    "folded": folded,
                    "en": "",
                    "category": "proverb",
                    "source": str(row.get("source", "")),
                    "license": str(row.get("license", "")),
                    "tags": [],
                    "difficulty": None,
                    "translation_status": "draft",
                    "vocabulary": [],
                    "status": "duplicate" if duplicate_of else "new",
                    "duplicate_of": duplicate_of,
                }
            )

    candidates.sort(key=lambda row: row["id"])
    write_jsonl(output_path, candidates)
    return candidates


def build_report(
    curated_path: Path | None = None,
    processed_path: Path | None = None,
    raw_dir: Path | None = None,
    output_path: Path | None = None,
) -> dict[str, Any]:
    """Write a deterministic quality report with the debt baseline."""
    curated_path = curated_path or data_path("curated") / "entries.jsonl"
    processed_path = processed_path or data_path("processed") / "candidates.jsonl"
    raw_dir = raw_dir or curated_path.parent.parent / "raw"
    output_path = output_path or data_path("artifacts") / "quality-report.json"

    report = validate_file(curated_path)
    candidates = read_jsonl(processed_path) if processed_path.exists() else []
    statuses = Counter(str(candidate.get("status")) for candidate in candidates)
    findings = Counter(finding.kind for finding in report.findings)
    errors = Counter(error.kind for error in report.errors)

    payload = {
        "schema_version": SCHEMA_VERSION,
        "inputs": {
            "curated": {
                "path": relative_path(curated_path),
                "sha256": sha256_file(curated_path),
            },
            "processed": {
                "path": relative_path(processed_path),
                "sha256": sha256_file(processed_path) if processed_path.exists() else None,
            },
            "raw": [
                {"name": path.name, "sha256": sha256_file(path)}
                for path in sorted(raw_dir.glob("*.jsonl"))
            ],
        },
        "entries": len(report.entries),
        "errors": len(report.errors),
        "error_kinds": dict(sorted(errors.items())),
        "findings": dict(sorted(findings.items())),
        "candidates": {
            "total": len(candidates),
            "new": statuses.get("new", 0),
            "duplicate": statuses.get("duplicate", 0),
        },
    }
    write_json(output_path, payload)
    return payload


def export_corpus(
    curated_path: Path | None = None, output_path: Path | None = None
) -> dict[str, Any]:
    """Export the curated collection as a stable analysis artifact."""
    curated_path = curated_path or data_path("curated") / "entries.jsonl"
    output_path = output_path or data_path("artifacts") / "corpus.json"
    payload = {
        "schema_version": SCHEMA_VERSION,
        "source_sha256": sha256_file(curated_path),
        "entries": read_jsonl(curated_path),
    }
    write_json(output_path, payload)
    return payload


def regenerate(root: Path | None = None) -> dict[str, Any]:
    """Run every derived stage in order without touching raw or curated files."""
    root = root or repo_root()
    data = root / "data"
    curated = data / "curated" / "entries.jsonl"

    imported = import_raw(data / "raw", data / "imported")
    candidates = normalise_candidates(data / "imported", curated, data / "processed" / "candidates.jsonl")
    report = build_report(
        curated,
        data / "processed" / "candidates.jsonl",
        data / "raw",
        data / "artifacts" / "quality-report.json",
    )
    corpus = export_corpus(curated, data / "artifacts" / "corpus.json")

    from .stats import statistics
    from .transform import transform

    corpus_path = data / "artifacts" / "corpus.json"
    stats = statistics(
        corpus_path,
        data / "artifacts" / "corpus-statistics.json",
        data / "artifacts" / "corpus-statistics.md",
    )
    transform(
        corpus_path,
        data / "processed" / "entries.csv",
        data / "processed" / "vocabulary.csv",
        data / "processed" / "tags.csv",
    )

    return {
        "imported_files": len(imported),
        "candidates": len(candidates),
        "entries": report["entries"],
        "errors": report["errors"],
        "corpus_entries": len(corpus["entries"]),
        "statistics_entries": stats["entry_count"],
    }
