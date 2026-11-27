# Generated artifacts

Files produced by the Python pipeline and consumed by analysis or verification.

- `quality-report.json` is written by `python -m dicho_data report`. It contains input digests, validation counts, quality finding counts, and candidate counts. Its findings form the committed baseline checked by CI.
- `corpus.json` is written by `python -m dicho_data export`. It contains the curated entries with a source digest and is the stable input for later analysis.
- `corpus-statistics.json` and `corpus-statistics.md` are written by `python -m dicho_data stats`. They contain pandas statistics, including counts, distributions, completeness, text lengths, and surface level word frequency.

These artifacts are generated and reproducible. Do not hand edit them.
