"""Locate repository data files without depending on the current directory."""

from __future__ import annotations

import os
from pathlib import Path

MARKER = Path("data") / "curated" / "entries.jsonl"


def repo_root() -> Path:
    """Return the repository root, honouring the DICHO_ROOT override."""
    override = os.environ.get("DICHO_ROOT")
    if override:
        return Path(override).expanduser().resolve()

    start = Path(__file__).resolve()
    for candidate in (*start.parents, Path.cwd().resolve()):
        if (candidate / MARKER).exists():
            return candidate
    raise RuntimeError("cannot locate the dicho repository root")
