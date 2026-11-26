# dicho

A Spanish language companion for your terminal.

`dicho` is a local-first, offline-capable command-line tool whose purpose is to make Spanish a recurring part of everyday computing. It combines curated expressions and literature, a personal vocabulary collection, learning history, linguistic analysis, and personalised content discovery.

> Status: v0.2.1. The CLI shows a daily Spanish entry, provides a browsable collection, and can search it. Translations are drafts awaiting review.

## Principles

- The daily Spanish experience comes first. It is the reason the project exists.
- Core functionality is local and offline, with no network and no account required.
- Rust is the application language. The CLI must work with Python and Mojo absent.
- Python prepares data and analysis offline, never as a runtime dependency.
- Mojo work stays isolated from the core application.
- The project remains small enough to maintain as a personal learning tool.

## Quick start

```sh
cargo run                                                            # today's entry
cargo run -- random                                                  # a random entry
cargo run -- repeat                                                  # today's entry again
cargo test                                                           # Rust tests
uv run --project python python -m unittest discover -s python/tests  # Python skeleton test
```

The default command shows one Spanish entry, its English translation, its category, its source, and a featured vocabulary item. Daily selection is deterministic for a given date and dataset. The date is UTC, so the entry changes at 00:00 UTC rather than local midnight.

## Commands

| Command | Description |
| --- | --- |
| `dicho` | Show today's entry |
| `dicho daily` | Show today's entry |
| `dicho random` | Show a random entry, never today's entry |
| `dicho repeat` | Show today's entry again |
| `dicho browse` | List all entries in compact form |
| `dicho show <id>` | Show one entry in full |
| `dicho categories` | List categories with counts |
| `dicho tags` | List tags with counts |
| `dicho search <terms>` | Search text, translation, tags, source, and vocabulary |

Search is case insensitive and ignores Spanish accents, so `abarca` finds `abarca` in either form.

## Repository layout

| Path | Purpose |
| --- | --- |
| `src/` | Rust CLI and application logic |
| `data/raw/` | Immutable source material with provenance and licence records |
| `data/imported/` | Normalised material derived from `raw/` |
| `data/curated/` | Canonical entries, currently draft pending review |
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
