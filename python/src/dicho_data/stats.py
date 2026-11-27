"""Exploratory corpus statistics using pandas."""

from __future__ import annotations

import json
import math
import re
from collections import Counter
from pathlib import Path
from typing import Any

import pandas as pd

from .paths import data_path
from .pipeline import write_json

SURFACE_WORD = re.compile(r"[^\W\d_]+", re.UNICODE)
COMPLETENESS_FIELDS = (
    "id",
    "es",
    "en",
    "category",
    "source",
    "license",
    "translation_status",
    "vocabulary",
)


def _rounded(value: float, digits: int = 4) -> float:
    number = float(value)
    return round(number, digits) if math.isfinite(number) else 0.0


def _cell(value: Any) -> str:
    """Escape a value for a Markdown table cell."""
    return str(value).replace("|", "\\|").replace("\n", " ")


def _distribution(series: pd.Series) -> dict[str, int]:
    counts = series.fillna("<missing>").astype(str).value_counts()
    return {str(key): int(value) for key, value in counts.sort_index().items()}


def _empty_length_stats() -> dict[str, float | int]:
    return {
        "chars_mean": 0.0,
        "chars_min": 0,
        "chars_max": 0,
        "words_mean": 0.0,
        "words_min": 0,
        "words_max": 0,
    }


def _length_stats(series: pd.Series) -> dict[str, float | int]:
    values = series.fillna("").astype(str)
    if values.empty:
        return _empty_length_stats()
    characters = values.str.len()
    words = values.str.count(r"\S+")
    return {
        "chars_mean": _rounded(characters.mean()),
        "chars_min": int(characters.min()),
        "chars_max": int(characters.max()),
        "words_mean": _rounded(words.mean()),
        "words_min": int(words.min()),
        "words_max": int(words.max()),
    }


def describe(entries: list[dict[str, Any]], source_sha256: str) -> dict[str, Any]:
    """Describe a curated collection using stable, serialisable values."""
    frame = pd.DataFrame(entries)
    for column in ("es", "en", "category", "translation_status", "license", "source"):
        if column not in frame.columns:
            frame[column] = pd.NA

    if frame.empty:
        return {
            "schema_version": 1,
            "source_sha256": source_sha256,
            "entry_count": 0,
            "distributions": {
                name: {}
                for name in ("category", "translation_status", "license", "source", "tag")
            },
            "completeness": {field: 0 for field in COMPLETENESS_FIELDS},
            "difficulty": {"count": 0, "min": 0.0, "max": 0.0, "mean": 0.0, "median": 0.0},
            "text_lengths": {"es": _empty_length_stats(), "en": _empty_length_stats()},
            "surface_words": {
                "token_count": 0,
                "unique_count": 0,
                "type_token_ratio": 0.0,
                "top_words": [],
                "note": "Surface level tokenisation, not linguistic analysis",
            },
            "vocabulary": {"total_items": 0, "mean_per_entry": 0.0, "max_per_entry": 0},
        }

    tags = [tag for entry in entries for tag in entry.get("tags", [])]
    vocabulary_counts = [len(entry.get("vocabulary", [])) for entry in entries]
    difficulty = (
        pd.to_numeric(frame["difficulty"], errors="coerce")
        if "difficulty" in frame.columns
        else pd.Series(dtype=float)
    )
    tokens = [
        match.group(0).lower()
        for text in frame["es"].fillna("").astype(str)
        for match in SURFACE_WORD.finditer(text)
    ]
    token_counts = Counter(tokens)
    top_words = [
        {"word": word, "count": count}
        for word, count in sorted(
            token_counts.items(), key=lambda item: (-item[1], item[0])
        )[:10]
    ]

    return {
        "schema_version": 1,
        "source_sha256": source_sha256,
        "entry_count": len(entries),
        "distributions": {
            "category": _distribution(frame["category"]),
            "translation_status": _distribution(frame["translation_status"]),
            "license": _distribution(frame["license"]),
            "source": _distribution(frame["source"]),
            "tag": dict(sorted(Counter(tags).items())),
        },
        "completeness": {
            field: sum(
                1
                for entry in entries
                if str(entry.get(field, "")).strip() not in ("", "[]")
            )
            for field in COMPLETENESS_FIELDS
        },
        "difficulty": {
            "count": int(difficulty.notna().sum()),
            "min": _rounded(difficulty.min()),
            "max": _rounded(difficulty.max()),
            "mean": _rounded(difficulty.mean()),
            "median": _rounded(difficulty.median()),
        },
        "text_lengths": {
            "es": _length_stats(frame["es"]),
            "en": _length_stats(frame["en"]),
        },
        "surface_words": {
            "token_count": len(tokens),
            "unique_count": len(token_counts),
            "type_token_ratio": _rounded(len(token_counts) / len(tokens)) if tokens else 0.0,
            "top_words": top_words,
            "note": "Surface level tokenisation, not linguistic analysis",
        },
        "vocabulary": {
            "total_items": sum(vocabulary_counts),
            "mean_per_entry": _rounded(sum(vocabulary_counts) / len(entries)) if entries else 0.0,
            "max_per_entry": max(vocabulary_counts) if vocabulary_counts else 0,
        },
    }


def render_markdown(stats: dict[str, Any]) -> str:
    """Render a short human readable report."""
    lines = [
        "# Corpus statistics",
        "",
        f"- Entries: {stats['entry_count']}",
        f"- Source digest: {stats['source_sha256']}",
        "",
    ]
    for name, distribution in stats["distributions"].items():
        lines.extend([f"## {name} distribution", "", "| value | count |", "| --- | --- |"])
        lines.extend(f"| {_cell(key)} | {_cell(value)} |" for key, value in distribution.items())
        lines.append("")
    summary = stats["surface_words"]
    lines.extend(
        [
            "## Surface level words",
            "",
            f"- Tokens: {summary['token_count']}",
            f"- Unique tokens: {summary['unique_count']}",
            f"- Type token ratio: {summary['type_token_ratio']}",
            "",
            summary["note"],
            "",
        ]
    )
    return "\n".join(lines)


def write_statistics(
    stats: dict[str, Any],
    json_path: Path | None = None,
    markdown_path: Path | None = None,
) -> None:
    json_path = json_path or data_path("artifacts") / "corpus-statistics.json"
    markdown_path = markdown_path or data_path("artifacts") / "corpus-statistics.md"
    write_json(json_path, stats)
    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    markdown_path.write_text(render_markdown(stats) + "\n", encoding="utf-8")


def statistics(
    corpus_path: Path | None = None,
    json_path: Path | None = None,
    markdown_path: Path | None = None,
) -> dict[str, Any]:
    """Build and write statistics from the corpus artifact."""
    corpus_path = corpus_path or data_path("artifacts") / "corpus.json"
    payload = json.loads(corpus_path.read_text(encoding="utf-8"))
    stats = describe(payload.get("entries", []), payload.get("source_sha256", ""))
    write_statistics(stats, json_path, markdown_path)
    return stats
