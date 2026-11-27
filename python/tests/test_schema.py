"""Tests for the canonical entry schema."""

import json
import unittest

from dicho_data.paths import repo_root
from dicho_data.schema import validate_file, validate_text

SAMPLE = {
    "id": "test-001",
    "es": "Hola.",
    "en": "Hello.",
    "category": "proverb",
    "source": "Sbarbi y Osuna, Diccionario de refranes (1922)",
    "license": "Public domain",
    "translation_status": "reviewed",
    "difficulty": 0.25,
    "tags": ["greeting"],
    "vocabulary": [{"term": "hola", "meaning": "hello"}],
}


def line(**changes):
    entry = SAMPLE | changes
    return json.dumps(entry, ensure_ascii=False)


class SchemaTest(unittest.TestCase):
    def test_embedded_collection_is_valid(self):
        path = repo_root() / "data" / "curated" / "entries.jsonl"
        report = validate_file(path)
        self.assertTrue(report.ok)
        self.assertEqual(len(report.entries), 40)
        self.assertEqual(len(report.errors), 0)

    def test_embedded_collection_reports_known_debt(self):
        path = repo_root() / "data" / "curated" / "entries.jsonl"
        report = validate_file(path)
        kinds = [finding.kind for finding in report.findings]
        self.assertEqual(kinds.count("draft_translation"), 40)
        self.assertEqual(kinds.count("source_citation_missing"), 20)

    def test_sample_is_valid(self):
        report = validate_text(line())
        self.assertTrue(report.ok)
        self.assertEqual(len(report.findings), 0)

    def test_duplicate_ids_are_errors(self):
        report = validate_text(f"{line()}\n{line()}")
        self.assertFalse(report.ok)
        self.assertTrue(any(error.kind == "duplicate_id" for error in report.errors))

    def test_invalid_values_are_errors(self):
        report = validate_text(line(category="unknown") + "\n" + line(difficulty=1.5))
        kinds = [error.kind for error in report.errors]
        self.assertIn("invalid_category", kinds)
        self.assertIn("invalid_difficulty", kinds)

    def test_report_serialises(self):
        report = validate_text(line(translation_status="draft"))
        payload = report.to_dict()
        self.assertEqual(payload["entry_count"], 1)
        self.assertEqual(payload["findings"][0]["kind"], "draft_translation")


if __name__ == "__main__":
    unittest.main()
