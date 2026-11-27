# Generated artifacts

Files produced by the Python pipeline and consumed by analysis or verification.

- `quality-report.json` is written by `python -m dicho_data report`. It contains input digests, validation counts, quality finding counts, and candidate counts. Its findings form the committed baseline checked by CI.
- `corpus.json` is written by `python -m dicho_data export`. It contains the curated entries with a source digest and is the stable input for later analysis.

These artifacts are generated and reproducible. Do not hand edit them.
