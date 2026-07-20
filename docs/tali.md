# Tali mode

Tali mode enables Scruff's opinionated linting and formatting behavior. It is still evolving, so
pin Scruff's version before enabling it in automated formatting workflows.

Enable it in `pyproject.toml`:

```toml
[tool.scruff]
mode = "tali"
```

Or in `scruff.toml`:

```toml
mode = "tali"
```

## Symbol quote style

The `symbol` quote style uses single quotes for identifier-like strings and double quotes for
natural-language text:

```toml
[tool.scruff]
mode = "tali"

[tool.scruff.format]
quote-style = "symbol"
```

Use `quote-symbol-regex` to define the strings that should be considered symbols:

```toml
[tool.scruff.format]
quote-style = "symbol"
quote-symbol-regex = "^[A-Za-z_][A-Za-z0-9_.-]*$"
```

The setting is also available on the command line:

```console
scruff format --config "format.quote-style = 'symbol'" .
```

## Side-effect imports

Tali mode treats an underscore alias as an explicit side-effect import:

```python
import package as _
```

This applies only to module imports. `from package import member as _` remains an unused import.

## Grouped signatures

Tali mode preserves deliberate parameter groups in multiline function signatures:

```python
def keyword_only(self, *,
    first,
    second,
):
    ...


def positional_only(
    a, b, /,
    c, d,
):
    ...
```

## Rectangular data

When any row in a comment-free, rectangular nested list uses extra whitespace to align columns,
Tali mode formats every cell normally and then enforces consistent column widths across the entire
expression. Rows may be lists or tuples, and this works recursively for two-dimensional and
higher-dimensional data. Unaligned or ragged sequences continue to use the standard formatter
layout. Numeric columns align on their decimal point (or the implied ones-place boundary for
integers).

## Current limitations

The following Tali behavior is planned but not implemented:

- Applying symbol quote selection to f-strings.
- Selecting triple-single quotes for strings containing double quotes.
- Optional alignment of comments, annotations, and assignments.
- Comment-aware line wrapping and nested-bracket hugging.

The implementation checklist is maintained in
[`TALI-MODE.md`](https://github.com/Demonstrandum/scruff/blob/master/TALI-MODE.md).
