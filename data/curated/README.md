# Curated content

The canonical entry collection served by the Rust application.

`entries.jsonl` contains 40 traditional Spanish refranes and sayings. Every entry has a stable identifier, an English translation marked as draft, a source statement, a licence statement, tags, a difficulty estimate, and one or more vocabulary items.

Difficulty is an optional number from 0.0 to 1.0. It is a heuristic estimate based on sentence length, vocabulary count, and apparent idiomatic content. It is not a CEFR classification and should not be presented as one.

Entries refran-021 through refran-040 were checked against Sbarbi y Osuna's 1922 dictionary. See `data/raw/SOURCES.md` for the full citation. All translations are drafts. Do not promote a translation to reviewed status until it has been checked by a person.

The Rust application embeds this file at build time and does not require Python.
