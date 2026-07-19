# Tali Mode

## Plan for `mode = "tali"`

- [x] `quote-style = "symbol"`:
    - [x] Symbols and identifiers get single quotes, natural language double quotes.
    - [ ] Respected in f-strings.
    - [ ] Triple-single quote strings when containing a `"` double quote.
- [x] Imports for side-effects allowed by `import x as _` instead of comments.
- [x] Preserve manually grouped multiline function parameters, including groups around `/` and `*`.
- [x] Preserve column-aligned rectangular 2-D and N-D list literals.
- [ ] Horizontal white-space alignment:
    - [ ] Allow aligning trailing comments to the right.
        - [ ] Auto-align with surrounding comments when more than 3 spaces away already.
        - [ ] Must not force alignment (comments could be unrelated)
    - [ ] Align annotations by spaces *after* `:` (same ideas as above)
    - [ ] Align assignments by spaces *before* `=` (same ideas as above)
- [ ] Trailing comments should never affect line-length reformats:
    - [ ] Comments that exceed line length should be dropped a line by themself `# ^ ...`
- [ ] Fix silly line-length issues:
    - [ ] Never ever drop something on its own line just to add an extra two lines containing
        surrounding `(` and `)`. Horizontal space is not more precious than vertical space.
- [ ] Nested brackets hug:
    - [ ] If the only argument to a function is a list, set, dict, etc. literal, do *NOT* cause an extra
        layer of indent along with the opening and closing delimiters dropping to their own line!
