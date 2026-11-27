"""Tabular transformations using Polars."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import polars as pl

from .paths import data_path

ENTRY_COLUMNS = (
    "id",
    "es",
    "en",
    "category",
    "author",
    "source",
    "license",
    "translation_status",
    "difficulty",
)


def frames(entries: list[dict[str, Any]]) -> dict[str, pl.DataFrame]:
    """Build deterministic tables from curated entries."""
    entry_rows = [{column: entry.get(column) for column in ENTRY_COLUMNS} for entry in entries]
    entries_frame = pl.DataFrame(
        entry_rows,
        schema={
            "id": pl.String,
            "es": pl.String,
            "en": pl.String,
            "category": pl.String,
            "author": pl.String,
            "source": pl.String,
            "license": pl.String,
            "translation_status": pl.String,
            "difficulty": pl.Float64,
        },
    ).select(list(ENTRY_COLUMNS)).sort("id")

    vocabulary_rows = [
        {"id": entry.get("id", ""), "term": item.get("term", ""), "meaning": item.get("meaning", "")}
        for entry in entries
        for item in entry.get("vocabulary", [])
    ]
    vocabulary_frame = pl.DataFrame(
        vocabulary_rows,
        schema={"id": pl.String, "term": pl.String, "meaning": pl.String},
    ).sort(["id", "term"])

    tag_rows = [
        {"id": entry.get("id", ""), "tag": tag}
        for entry in entries
        for tag in entry.get("tags", [])
    ]
    tag_frame = pl.DataFrame(
        tag_rows,
        schema={"id": pl.String, "tag": pl.String},
    ).sort(["id", "tag"])

    return {
        "entries": entries_frame,
        "vocabulary": vocabulary_frame,
        "tags": tag_frame,
    }


def write_tables(
    entries: list[dict[str, Any]],
    entries_path: Path | None = None,
    vocabulary_path: Path | None = None,
    tags_path: Path | None = None,
) -> dict[str, pl.DataFrame]:
    """Write the tabular artifacts and return the frames used."""
    entries_path = entries_path or data_path("processed") / "entries.csv"
    vocabulary_path = vocabulary_path or data_path("processed") / "vocabulary.csv"
    tags_path = tags_path or data_path("processed") / "tags.csv"

    tables = frames(entries)
    for path, frame in (
        (entries_path, tables["entries"]),
        (vocabulary_path, tables["vocabulary"]),
        (tags_path, tables["tags"]),
    ):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(frame.write_csv(), encoding="utf-8")
    return tables


def transform(
    corpus_path: Path | None = None,
    entries_path: Path | None = None,
    vocabulary_path: Path | None = None,
    tags_path: Path | None = None,
) -> dict[str, pl.DataFrame]:
    """Transform the corpus artifact into the processed tables."""
    corpus_path = corpus_path or data_path("artifacts") / "corpus.json"
    payload = json.loads(corpus_path.read_text(encoding="utf-8"))
    return write_tables(payload.get("entries", []), entries_path, vocabulary_path, tags_path)
