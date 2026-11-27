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


if __name__ == "__main__":
    unittest.main()
