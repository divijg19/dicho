"""Spanish text normalisation shared by the data pipeline."""

from __future__ import annotations

import unicodedata

IGNORED = {"¿": "", "¡": ""}


def normalise(text: str) -> str:
    """Normalise Unicode and trim outer whitespace."""
    return unicodedata.normalize("NFC", text).strip()


def fold_spanish(text: str) -> str:
    """Lowercase and fold Spanish diacritics to their base letters."""
    decomposed = unicodedata.normalize("NFD", text).lower()
    folded: list[str] = []
    for character in decomposed:
        if unicodedata.combining(character):
            continue
        folded.append(IGNORED.get(character, character))
    return "".join(folded)
