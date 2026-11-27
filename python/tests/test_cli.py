"""Tests for the Python data command line interface."""

import io
import json
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path

from dicho_data.cli import build_parser, main

SAMPLE = (
    '{"id":"test-001","es":"Hola.","en":"Hello.","category":"proverb",'
    '"source":"Sbarbi y Osuna, Diccionario de refranes (1922)",'
    '"license":"Public domain","translation_status":"reviewed",'
    '"vocabulary":[{"term":"hola","meaning":"hello"}]}\n'
)


class CliTest(unittest.TestCase):
    def test_parser_requires_a_command(self):
        with self.assertRaises(SystemExit):
            build_parser().parse_args([])

    def test_validate_accepts_a_clean_file(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "entries.jsonl"
            path.write_text(SAMPLE, encoding="utf-8")
            output = io.StringIO()
            with redirect_stdout(output):
                status = main(["validate", "--data", str(path)])
            self.assertEqual(status, 0)
            self.assertIn("Entries: 1", output.getvalue())

    def test_validate_is_strict_only_on_request(self):
        draft = SAMPLE.replace('"reviewed"', '"draft"')
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "entries.jsonl"
            path.write_text(draft, encoding="utf-8")
            with redirect_stdout(io.StringIO()):
                self.assertEqual(main(["validate", "--data", str(path)]), 0)
                self.assertEqual(main(["validate", "--data", str(path), "--strict"]), 1)

    def test_validate_can_emit_json(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "entries.jsonl"
            path.write_text(SAMPLE, encoding="utf-8")
            output = io.StringIO()
            with redirect_stdout(output):
                status = main(["validate", "--data", str(path), "--json"])
            self.assertEqual(status, 0)
            payload = json.loads(output.getvalue())
            self.assertEqual(payload["entry_count"], 1)


    def test_pipeline_commands_run(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            data = root / "data"
            for name in ("raw", "imported", "curated", "processed", "artifacts"):
                (data / name).mkdir(parents=True, exist_ok=True)
            (data / "raw" / "source.jsonl").write_text(
                json.dumps(
                    {
                        "text": "Hola.",
                        "source": "Sbarbi y Osuna, Diccionario de refranes (1922)",
                        "license": "Public domain",
                        "locator": "test",
                        "access_date": "2026-11-27",
                    },
                    ensure_ascii=False,
                    sort_keys=True,
                )
                + "\n",
                encoding="utf-8",
            )
            (data / "curated" / "entries.jsonl").write_text(SAMPLE, encoding="utf-8")

            output = io.StringIO()
            with redirect_stdout(output):
                self.assertEqual(main(["import", "--raw", str(data / "raw"), "--output", str(data / "imported")]), 0)
                self.assertEqual(
                    main(
                        [
                            "normalise",
                            "--input",
                            str(data / "imported"),
                            "--curated",
                            str(data / "curated" / "entries.jsonl"),
                            "--output",
                            str(data / "processed" / "candidates.jsonl"),
                        ]
                    ),
                    0,
                )
                self.assertEqual(
                    main(
                        [
                            "report",
                            "--curated",
                            str(data / "curated" / "entries.jsonl"),
                            "--processed",
                            str(data / "processed" / "candidates.jsonl"),
                            "--output",
                            str(data / "artifacts" / "quality-report.json"),
                        ]
                    ),
                    0,
                )
                self.assertEqual(
                    main(
                        [
                            "export",
                            "--curated",
                            str(data / "curated" / "entries.jsonl"),
                            "--output",
                            str(data / "artifacts" / "corpus.json"),
                        ]
                    ),
                    0,
                )
            self.assertTrue((data / "artifacts" / "corpus.json").exists())
            self.assertTrue((data / "artifacts" / "quality-report.json").exists())

    def test_regenerate_command_accepts_a_root(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in ("raw", "imported", "curated", "processed", "artifacts"):
                (root / "data" / name).mkdir(parents=True, exist_ok=True)
            (root / "data" / "curated" / "entries.jsonl").write_text(SAMPLE, encoding="utf-8")
            with redirect_stdout(io.StringIO()):
                status = main(["regenerate", "--root", str(root)])
            self.assertEqual(status, 0)
            self.assertTrue((root / "data" / "artifacts" / "corpus.json").exists())


if __name__ == "__main__":
    unittest.main()
