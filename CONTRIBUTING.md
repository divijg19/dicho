# Development workflow

## Branches

Development happens on version side branches off `main`, named by minor version: `v0.0.x`, `v0.1.x`, and so on through `v0.9.x`. The `v0.0.x` branch holds the foundation.

## Releases

- One commit is one release. A release branch receives one commit, which is pushed individually and proposed through its own pull request into `main`.
- Keep each commit small enough to build, pass tests, and be describable in one sentence. Use `version: title` for the commit subject.
- No Git tags are created before `v0.3.x`. Version milestones are tracked in `CHANGELOG.md` until then.

## Before proposing a release

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
uv run --project python python -m unittest discover -s python/tests
```

## Rules that should not be relaxed

- The CLI must remain usable offline and without Python installed.
- Experimental components (Mojo, and later the optional TUI) must not become mandatory runtime dependencies.
- Attribution must never be invented; see [docs/CONTENT_PROVENANCE.md](docs/CONTENT_PROVENANCE.md).
