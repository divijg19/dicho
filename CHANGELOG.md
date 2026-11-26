# Changelog

## v0.0.1: Foundation

- Establish project identity and documentation.
- Add Rust CLI scaffolding with a runnable root command and tests.
- Add the Python data package skeleton and a smoke test with no dependencies.
- Define the raw / imported / curated / processed / artifacts data layout.
- Record the content provenance policy and development workflow.

## v0.0.2: Python tooling

- Pin the Python interpreter for the data package.
- Commit the uv lockfile and ignore setuptools build artifacts.
- Document the uv setup and test workflow.

## v0.1.0: Daily Spanish foundation

- Show a daily Spanish entry with translation, category, source, and featured vocabulary.
- Add `random` and `repeat` commands alongside the default daily command.
- Select daily content by a stable date hash over identifier sorted entries.
- Embed an initial collection of 20 traditional refranes and sayings.
- Mark all initial translations as drafts pending human review.

## v0.1.1: Collection validation

- Validate embedded entries when loading, with line numbers in error messages.
- Reject empty collections, duplicate identifiers, blank fields, and empty vocabulary.
- Correct the daily date documentation to state UTC.
- Add negative tests for malformed collections.

## v0.1.2: Selection and documentation polish

- Skip today's entry when serving a random entry.
- Test that a range of dates maps to more than one entry.
- Add a command reference to the README and document the UTC rollover.

## v0.2.0: Browsing and inspection

- Add browse, show, categories, and tags commands.
- Add a numeric heuristic difficulty estimate to every entry.
- Grow the curated collection from 20 to 40 entries using Sbarbi 1922 as the primary public domain source.
- Record the source citation in data/raw/SOURCES.md.

## v0.2.1: Search

- Add search across Spanish text, English translation, tags, source, author, and vocabulary.
- Fold Spanish diacritics and case so unaccented queries match accented text.
- Add unit and integration tests for search behaviour.

## v0.2.2: Filters and saved entries

- Add category, tag, and difficulty range filters to browse and search.
- Add save, unsave, and saved commands backed by a local JSON file.
- Resolve the data path from DICHO_DATA_DIR, XDG_DATA_HOME, or the home directory.
- Add unit and integration tests for filters and saved entry round trips.

## v0.2.3: Recent content history

- Add an append only JSONL event log under the local data directory.
- Record daily encounters once per UTC date, plus save and unsave events.
- Add a history command with a limit option.
- Add unit and integration tests for event rendering and deduplication.

## v0.2.4: Search and filter polish

- Fold searchable fields once when building an index instead of on every query.
- Match whole words by default, with a substring option.
- Fold category filter values for consistent case handling.
- Consolidate repeated entry lookup, line printing, and daily selection logic.

## v0.2.5: Storage robustness and tooling consistency

- Write saved state through a temporary file and rename so writes are atomic.
- Validate the saved state version and deduplicate identifiers on load.
- Align the Python package version with the CLI release.
- Add a clippy gate to the documented verification commands.

Version milestones are tracked here until tagging begins at v0.3.x.
