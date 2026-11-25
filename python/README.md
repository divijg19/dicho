# Python data utilities

This directory holds the Python package used for offline content preparation and analysis. It is not needed to run the Rust CLI.

## Setup

From this directory:

```sh
uv sync
```

The interpreter is pinned in `.python-version` and dependencies are locked in `uv.lock`.

## Tests

```sh
uv run python -m unittest discover -s tests
```

From the repository root, the equivalent command is:

```sh
uv run --project python python -m unittest discover -s python/tests
```
