# Processed data

Derived candidates produced by `python -m dicho_data normalise`.

`candidates.jsonl` records whether each imported excerpt is new or a duplicate of a curated entry, using folded Spanish text as the identity key. Candidates are not curated entries and are not served to the application.

`entries.csv`, `vocabulary.csv`, and `tags.csv` are Polars transformations of the curated corpus artifact. Rows are sorted deterministically and vocabulary and tags are exploded into one row per item.

All files here are generated and reproducible. Do not hand edit them.
