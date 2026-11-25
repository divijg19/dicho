# Content provenance policy

Every content item in `dicho` must retain appropriate provenance. This policy applies to text in `data/raw/`, `data/curated/`, and any derived dataset.

## Categories of material

| Category | Treatment |
| --- | --- |
| Traditional expressions and refranes | Record the collection or tradition where known; anonymous where not. |
| Anonymous or uncertain material | Mark explicitly as anonymous or uncertain; do not guess an author. |
| Attributed quotations | Record author and work only when the attribution is verifiable. |
| Public-domain literature | Record edition, source, and public-domain evidence. |
| Original learning material | Mark as original to `dicho`. |
| Third-party licensed material | Record licence, source, and attribution requirements. |

## Rules

- Never invent attribution. Unknown means unknown; it is not a gap to be filled.
- Keep original source material in `data/raw/` with a URL or bibliographic reference, access date, and licence evidence. Derived files must remain distinguishable from it.
- Do not reproduce copyrighted books or modern literary works beyond what is legally appropriate. Prefer public-domain editions and short, well-attributed excerpts.
- Record translation status. Where no direct English equivalent exists, distinguish literal meaning, natural translation, and cultural or contextual explanation. Alternative or uncertain translations are permitted.
- Provenance fields are part of the entry schema and are validated by the Python pipeline.
