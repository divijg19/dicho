"""Tests for Spanish text normalisation."""

import unicodedata
import unittest

from dicho_data.text import fold_spanish, normalise

# These cases are mirrored in the Rust search tests.
CASES = [
    ("ÁBARCA", "abarca"),
    ("Corazón", "corazon"),
    ("Dime con quién andas", "dime con quien andas"),
    ("¿Qué tal?", "que tal?"),
]


class TextTest(unittest.TestCase):
    def test_fold_cases(self):
        for source, expected in CASES:
            with self.subTest(source=source):
                self.assertEqual(fold_spanish(source), expected)

    def test_normalise_uses_nfc(self):
        decomposed = "A" + "\u0301"
        self.assertEqual(normalise(decomposed), unicodedata.normalize("NFC", decomposed))

    def test_normalise_trims_whitespace(self):
        self.assertEqual(normalise("  hola  "), "hola")


if __name__ == "__main__":
    unittest.main()
