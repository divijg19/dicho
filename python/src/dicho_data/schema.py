"""Canonical schema and validation for the curated entry collection."""

from __future__ import annotations

import json
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

SCHEMA_VERSION = 1

VALID_CATEGORIES = frozenset({"proverb", "saying"})
VALID_TRANSLATION_STATUSES = frozenset({"draft", "reviewed"})
REQUIRED_FIELDS = (
    "id",
    "es",
    "en",
    "category",
    "source",
    "license",
    "translation_status",
    "vocabulary",
)
REQUIRED_STRINGS = ("id", "es", "en", "source", "license")
DATED_SOURCE = re.compile(r"\b(?:1[5-9]\d{2}|20\d{2})\b")


@dataclass(frozen=True)
class Finding:
    line: int
    entry_id: str
    kind: str
    message: str


@dataclass
class Report:
    source: str
    entries: list[dict[str, Any]] = field(default_factory=list)
    errors: list[Finding] = field(default_factory=list)
    findings: list[Finding] = field(default_factory=list)

    @property
    def ok(self) -> bool:
        return not self.errors

    def to_dict(self) -> dict[str, Any]:
        return {
            "source": self.source,
            "schema_version": SCHEMA_VERSION,
            "entry_count": len(self.entries),
            "errors": [finding.__dict__ for finding in self.errors],
            "findings": [finding.__dict__ for finding in self.findings],
        }


def _error(report: Report, line: int, entry_id: str, kind: str, message: str) -> None:
    report.errors.append(Finding(line, entry_id, kind, message))


def validate_text(text: str, source: str = "<text>") -> Report:
    """Validate JSONL text and collect structural errors and quality findings."""
    report = Report(source=source)
    seen: set[str] = set()

    for line_number, line in enumerate(text.splitlines(), start=1):
        if not line.strip():
            continue

        try:
            entry = json.loads(line)
        except json.JSONDecodeError as exc:
            _error(report, line_number, "", "invalid_json", str(exc))
            continue

        if not isinstance(entry, dict):
            _error(report, line_number, "", "invalid_entry", "entry must be an object")
            continue

        entry_id = str(entry.get("id", ""))

        missing = [name for name in REQUIRED_FIELDS if name not in entry]
        for name in missing:
            _error(report, line_number, entry_id, "missing_field", f"missing {name}")

        for name in REQUIRED_STRINGS:
            value = entry.get(name)
            if value is not None and (not isinstance(value, str) or not value.strip()):
                _error(report, line_number, entry_id, "empty_field", f"{name} must not be empty")

        if entry_id and entry_id in seen:
            _error(report, line_number, entry_id, "duplicate_id", "duplicate identifier")
        seen.add(entry_id)

        category = entry.get("category")
        if category is not None and category not in VALID_CATEGORIES:
            _error(report, line_number, entry_id, "invalid_category", f"unknown category {category!r}")

        status = entry.get("translation_status")
        if status is not None and status not in VALID_TRANSLATION_STATUSES:
            _error(
                report,
                line_number,
                entry_id,
                "invalid_translation_status",
                f"unknown translation status {status!r}",
            )

        difficulty = entry.get("difficulty")
        if difficulty is not None and (
            isinstance(difficulty, bool)
            or not isinstance(difficulty, (int, float))
            or not 0.0 <= difficulty <= 1.0
        ):
            _error(
                report,
                line_number,
                entry_id,
                "invalid_difficulty",
                "difficulty must be between 0.0 and 1.0",
            )

        vocabulary = entry.get("vocabulary")
        if vocabulary is not None and (not isinstance(vocabulary, list) or not vocabulary):
            _error(report, line_number, entry_id, "empty_vocabulary", "vocabulary must not be empty")
        elif isinstance(vocabulary, list):
            for item in vocabulary:
                if not isinstance(item, dict) or not str(item.get("term", "")).strip():
                    _error(report, line_number, entry_id, "invalid_vocabulary", "vocabulary term must not be empty")
                elif not str(item.get("meaning", "")).strip():
                    _error(report, line_number, entry_id, "invalid_vocabulary", "vocabulary meaning must not be empty")

        tags = entry.get("tags", [])
        if not isinstance(tags, list):
            _error(report, line_number, entry_id, "invalid_tags", "tags must be a list")
        elif any(not isinstance(tag, str) or not tag.strip() for tag in tags):
            _error(report, line_number, entry_id, "invalid_tags", "tags must not be empty")

        report.entries.append(entry)

        if status == "draft":
            report.findings.append(
                Finding(line_number, entry_id, "draft_translation", "translation awaiting human review")
            )
        source_text = entry.get("source")
        if isinstance(source_text, str) and not DATED_SOURCE.search(source_text):
            report.findings.append(
                Finding(line_number, entry_id, "source_citation_missing", "source has no dated collection citation")
            )

    if not report.entries:
        _error(report, 0, "", "empty_collection", "collection is empty")

    return report


def validate_file(path: Path) -> Report:
    """Read and validate a JSONL collection."""
    text = path.read_text(encoding="utf-8")
    return validate_text(text, str(path))
