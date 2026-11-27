"""Tests for pandas corpus statistics."""

import json
import tempfile
import unittest
from pathlib import Path

from dicho_data.stats import describe, render_markdown, statistics

ENTRIES = [
    {
        "id": "test-001",
        "es": "Más vale tarde que nunca.",
        "en": "Better late than never.",
        "category": "proverb",
        "source": "Sbarbi y Osuna, Diccionario de refranes (1922)",
        "license": "Public domain",
        "translation_status": "draft",
        "tags": ["time"],
        "difficulty": 0.2,
        "vocabulary": [{"term": "vale", "meaning": "it is worth"}],
    },
    {
        "id": "test-002",
        "es": "Poco a poco se va lejos.",
        "en": "Little by little one goes far.",
        "category": "saying",
        "source": "Sbarbi y Osuna, Diccionario de refranes (1922)",
        "license": "Public domain",
        "translation_status": "reviewed",
        "tags": ["time", "patience"],
        "difficulty": 0.4,
        "vocabulary": [
            {"term": "poco", "meaning": "little"},
            {"term": "lejos", "meaning": "far"},
        ],
    },
]


class StatsTest(unittest.TestCase):
    def test_describe_counts_and_distributions(self):
        stats = describe(ENTRIES, "digest")
        self.assertEqual(stats["entry_count"], 2)
        self.assertEqual(stats["source_sha256"], "digest")
        self.assertEqual(stats["distributions"]["category"], {"proverb": 1, "saying": 1})
        self.assertEqual(stats["distributions"]["tag"], {"patience": 1, "time": 2})
        self.assertEqual(stats["completeness"]["es"], 2)
        self.assertEqual(stats["difficulty"]["count"], 2)
        self.assertEqual(stats["difficulty"]["mean"], 0.3)

    def test_describe_surface_words(self):
        stats = describe(ENTRIES, "digest")
        surface = stats["surface_words"]
        self.assertGreater(surface["token_count"], 0)
        self.assertLessEqual(surface["unique_count"], surface["token_count"])
        self.assertLessEqual(len(surface["top_words"]), 10)
        self.assertEqual(surface["note"], "Surface level tokenisation, not linguistic analysis")
        self.assertEqual(
            surface["top_words"],
            sorted(surface["top_words"], key=lambda item: (-item["count"], item["word"])),
        )

    def test_describe_empty_collection(self):
        stats = describe([], "digest")
        self.assertEqual(stats["entry_count"], 0)
        self.assertEqual(stats["distributions"]["category"], {})
        self.assertEqual(stats["difficulty"]["count"], 0)
        self.assertEqual(stats["surface_words"]["top_words"], [])
        self.assertEqual(stats["text_lengths"]["es"]["words_max"], 0)

    def test_render_markdown_escapes_cells(self):
        entries = [dict(ENTRIES[0], category="a|b")]
        text = render_markdown(describe(entries, "digest"))
        self.assertIn("| a\\|b | 1 |", text)

    def test_render_markdown(self):
        text = render_markdown(describe(ENTRIES, "digest"))
        self.assertIn("# Corpus statistics", text)
        self.assertIn("## category distribution", text)
        self.assertIn("| proverb | 1 |", text)

    def test_statistics_writes_reports(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            corpus = root / "corpus.json"
            corpus.write_text(
                json.dumps({"schema_version": 1, "source_sha256": "digest", "entries": ENTRIES}),
                encoding="utf-8",
            )
            json_path = root / "corpus-statistics.json"
            markdown_path = root / "corpus-statistics.md"
            stats = statistics(corpus, json_path, markdown_path)
            self.assertEqual(stats["entry_count"], 2)
            self.assertTrue(json_path.exists())
            self.assertTrue(markdown_path.exists())


if __name__ == "__main__":
    unittest.main()
