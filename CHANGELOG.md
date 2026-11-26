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

Version milestones are tracked here until tagging begins at v0.3.x.
