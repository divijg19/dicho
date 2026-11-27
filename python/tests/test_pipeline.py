"""Tests for the reproducible data pipeline."""

import json
import tempfile
import unittest
from pathlib import Path

from dicho_data.pipeline import (
    build_report,
    export_corpus,
    import_raw,
    normalise_candidates,
    regenerate,
)

CURATED = (
    '{"id":"test-001","es":"Hola.","en":"Hello.","category":"proverb",'
    '"source":"Sbarbi y Osuna, Diccionario de refranes (1922)",'
    '"license":"Public domain","translation_status":"reviewed",'
    '"vocabulary":[{"term":"hola","meaning":"hello"}]}\n'
)

RAW = "".join(
    json.dumps(row, ensure_ascii=False, sort_keys=True) + "\n"
    for row in [
        {
            "text": "Hola.",
            "source": "Sbarbi y Osuna, Diccionario de refranes (1922)",
            "license": "Public domain",
            "locator": "test",
            "access_date": "2026-11-27",
        },
        {
            "text": "Adiós.",
            "source": "Sbarbi y Osuna, Diccionario de refranes (1922)",
            "license": "Public domain",
            "locator": "test",
            "access_date": "2026-11-27",
        },
    ]
)


def make_root(base: Path) -> Path:
    for name in ("raw", "imported", "curated", "processed", "artifacts"):
        (base / "data" / name).mkdir(parents=True, exist_ok=True)
    (base / "data" / "raw" / "source.jsonl").write_text(RAW, encoding="utf-8")
    (base / "data" / "curated" / "entries.jsonl").write_text(CURATED, encoding="utf-8")
    return base


class PipelineTest(unittest.TestCase):
    def test_import_is_stable(self):
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory))
            raw = root / "data" / "raw"
            imported = root / "data" / "imported"
            first = import_raw(raw, imported)
            before = first[0].read_bytes()
            second = import_raw(raw, imported)
            self.assertEqual(first, second)
            self.assertEqual(before, second[0].read_bytes())
            rows = [json.loads(line) for line in before.decode().splitlines()]
            self.assertEqual([row["es"] for row in rows], ["Adiós.", "Hola."])

    def test_normalise_marks_duplicates(self):
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory))
            data = root / "data"
            import_raw(data / "raw", data / "imported")
            candidates = normalise_candidates(
                data / "imported",
                data / "curated" / "entries.jsonl",
                data / "processed" / "candidates.jsonl",
            )
            statuses = {row["es"]: row["status"] for row in candidates}
            self.assertEqual(statuses["Hola."], "duplicate")
            self.assertEqual(statuses["Adiós."], "new")

    def test_report_and_export(self):
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory))
            data = root / "data"
            import_raw(data / "raw", data / "imported")
            normalise_candidates(
                data / "imported",
                data / "curated" / "entries.jsonl",
                data / "processed" / "candidates.jsonl",
            )
            report = build_report(
                data / "curated" / "entries.jsonl",
                data / "processed" / "candidates.jsonl",
                data / "raw",
                data / "artifacts" / "quality-report.json",
            )
            self.assertEqual(report["entries"], 1)
            self.assertEqual(report["errors"], 0)
            self.assertEqual(report["candidates"]["duplicate"], 1)
            self.assertEqual(report["candidates"]["new"], 1)
            self.assertIn("sha256", report["inputs"]["curated"])

            corpus = export_corpus(
                data / "curated" / "entries.jsonl",
                data / "artifacts" / "corpus.json",
            )
            self.assertEqual(len(corpus["entries"]), 1)

    def test_regenerate_is_byte_deterministic(self):
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory))
            first = regenerate(root)
            generated = [
                root / "data" / "imported" / "source.jsonl",
                root / "data" / "processed" / "candidates.jsonl",
                root / "data" / "artifacts" / "quality-report.json",
                root / "data" / "artifacts" / "corpus.json",
            ]
            before = [path.read_bytes() for path in generated]
            second = regenerate(root)
            self.assertEqual(first, second)
            self.assertEqual(before, [path.read_bytes() for path in generated])


if __name__ == "__main__":
    unittest.main()
