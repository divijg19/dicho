# dicho

A Spanish language companion for your terminal.

`dicho` is a local-first, offline-capable command-line tool whose purpose is to make Spanish a recurring part of everyday computing. It combines curated expressions and literature, a personal vocabulary collection, learning history, linguistic analysis, and personalised content discovery.

> Status: v0.0.2 (foundation). The CLI builds and runs, but only prints a placeholder. Daily Spanish content arrives in v0.1.

## Principles

- The daily Spanish experience comes first. It is the reason the project exists.
- Core functionality is local and offline, with no network and no account required.
- Rust is the application language. The CLI must work with Python and Mojo absent.
- Python prepares data and analysis offline, never as a runtime dependency.
- Mojo work stays isolated from the core application.
- The project remains small enough to maintain as a personal learning tool.

## Quick start

```sh
cargo run                                                            # foundation placeholder
cargo test                                                           # Rust tests
uv run --project python python -m unittest discover -s python/tests  # Python skeleton test
```

## Repository layout

| Path | Purpose |
| --- | --- |
| `src/` | Rust CLI and application logic |
| `data/raw/` | Immutable source material with provenance and licence records |
| `data/imported/` | Normalised material derived from `raw/` |
| `data/curated/` | Canonical, human-reviewed entries |
| `data/processed/` | Derived linguistic features |
| `data/artifacts/` | Generated files consumed by the Rust application |
| `python/` | Standalone data-preparation package (`dicho-data`) |
| `experiments/mojo/` | Optional numerical experiments |
| `docs/` | Provenance policy and roadmap |
| `tests/` | Rust integration tests |

## Documentation

- [Content provenance policy](docs/CONTENT_PROVENANCE.md)
- [Roadmap](docs/ROADMAP.md)
- [Development workflow](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## Licence

Code is licensed under the [MIT licence](LICENSE). Content licensing and attribution are governed separately by the [content provenance policy](docs/CONTENT_PROVENANCE.md).
