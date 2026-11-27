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

## Tests

```sh
uv run python -m unittest discover -s tests
```

From the repository root, the equivalent command is:

```sh
uv run --project python python -m unittest discover -s python/tests
```
