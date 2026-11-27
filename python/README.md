# Python data utilities

This directory holds the Python package used for offline content preparation, validation, and analysis. It is not needed to run the Rust CLI.

## Setup

From this directory:

```sh
uv sync
```

The interpreter is pinned in `.python-version` and dependencies are locked in `uv.lock`.

## Validation

From the repository root:

```sh
uv run --project python python -m dicho_data validate
```

The validator checks the curated JSONL collection against the canonical schema and reports structural errors separately from quality findings such as draft translations and missing source citations. It exits with a non zero status on errors. Add `--strict` to fail on findings too, `--json` for a machine readable report, and `--data PATH` to validate another file.

## Data pipeline

The pipeline moves data from `data/raw` to derived layers without writing to `data/curated`:

```sh
uv run --project python python -m dicho_data import
uv run --project python python -m dicho_data normalise
uv run --project python python -m dicho_data report
uv run --project python python -m dicho_data export
uv run --project python python -m dicho_data regenerate
```

`import` normalises raw source excerpts into `data/imported`. `normalise` builds deduplicated candidates in `data/processed`. `report` writes `data/artifacts/quality-report.json`, and `export` writes `data/artifacts/corpus.json`. `regenerate` runs all stages in order. Every stage accepts path overrides, and `regenerate` accepts `--root`.

Generated files are committed. CI runs `regenerate` and fails if the generated files differ, so changes to raw or curated data must be accompanied by regenerated artifacts. Quality findings are baselined through the committed report rather than by failing validation.

## Analysis

From the repository root:

```sh
uv run --project python python -m dicho_data stats
uv run --project python python -m dicho_data transform
```

`stats` uses pandas to write `data/artifacts/corpus-statistics.json` and a Markdown summary. Its word frequency and type token ratio use surface level tokenisation and are not linguistic annotations. `transform` uses Polars to write `entries.csv`, `vocabulary.csv`, and `tags.csv` in `data/processed`. Both commands accept path overrides. They are also part of `regenerate`, so CI verifies their outputs.

pandas and polars are regular dependencies of the package and are locked in `uv.lock`.

## Tests

```sh
uv run python -m unittest discover -s tests
```

From the repository root, the equivalent command is:

```sh
uv run --project python python -m unittest discover -s python/tests
```
