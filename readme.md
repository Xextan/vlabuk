# vlabuk

## Guide to editing `words.json`

**Before committing** changes to the JSON please
- ensure nothing is empty (regex-search `[^\\]""|\[\]`)
- also ensure there aren't any json syntax errors
- run `cargo run -r`

Properties that exist:

- **`word`.**
- **`def`:** The *short definition*, corresponding to the "concept" spreadsheet column for roots, "meaning" for compounds/freewords, and "function" for particles.
- **`etymology`:** Either an object or a list of them, each with at most these properties:
    - **`lang`:** Source language.
    - **`word`.**
    - **`translit`:** A transliteration if needed.
    - **`urlform`:** For e.g. Klingon, where the URL of the word *qa'* is `https://klingon.wiki/Word/Ka-`; or Arabic, where URLs should lack vowels.
    - **`link`:** Set to `false` to not link the word anywhere, or a URL to force the link to go there.
- **`notes`.**

Root-specific:

- **`alignment`:** The place structure.
- **`type`:** One of the new ones that start with a capital letter.
- **`semantics`:** The semantic space.
- **`derivs`:** Object with any of the keys `xo`/`ko`/`ga`/`ge`/`qu`/`po`/`sa`/`se`/`si`, representing the new `def` this word has when any are attached to it.

The "pat." column is unnecessary here as it can be found via `/[^aeiou]$/.test(word) ? "a" : "b"`.

Compound-specific: **`gloss`**.

Particle-specific: **`type`**.
