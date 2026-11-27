"""Tests for Polars tabular transformations."""

import tempfile
import unittest
from pathlib import Path

from dicho_data.transform import frames, write_tables

ENTRIES = [
    {
        "id": "test-002",
        "es": "Poco a poco.",
        "en": "Little by little.",
        "category": "saying",
        "source": "Test source (1922)",
        "license": "Public domain",
        "translation_status": "draft",
        "tags": ["time", "patience"],
        "difficulty": 0.4,
        "vocabulary": [{"term": "poco", "meaning": "little"}, {"term": "lejos", "meaning": "far"}],
    },
    {
        "id": "test-001",
        "es": "Más vale tarde.",
        "en": "Better late.",
        "category": "proverb",
        "source": "Test source (1922)",
        "license": "Public domain",
        "translation_status": "draft",
        "tags": ["time"],
        "difficulty": 0.2,
        "vocabulary": [{"term": "vale", "meaning": "it is worth"}],
    },
]


class TransformTest(unittest.TestCase):
    def test_frames_are_sorted_and_exploded(self):
        tables = frames(ENTRIES)
        self.assertEqual(tables["entries"].height, 2)
        self.assertEqual(tables["entries"]["id"].to_list(), ["test-001", "test-002"])
        self.assertEqual(tables["vocabulary"].height, 3)
        self.assertEqual(tables["tags"].height, 3)
        self.assertEqual(tables["tags"]["tag"].to_list(), ["time", "patience", "time"])

    def test_write_tables_is_deterministic(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = {
                "entries": root / "entries.csv",
                "vocabulary": root / "vocabulary.csv",
                "tags": root / "tags.csv",
            }
            write_tables(ENTRIES, paths["entries"], paths["vocabulary"], paths["tags"])
            before = [path.read_bytes() for path in paths.values()]
            write_tables(ENTRIES, paths["entries"], paths["vocabulary"], paths["tags"])
            self.assertEqual(before, [path.read_bytes() for path in paths.values()])
            header = "id,es,en,category,author,source,license,translation_status,difficulty\n"
            self.assertTrue(paths["entries"].read_text().startswith(header))


if __name__ == "__main__":
    unittest.main()
