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

Symbol classification also applies to nested strings in f-string expressions. On Python 3.12 and
newer, PEP 701 allows the selected quote to match the outer f-string delimiter; older targets use
the opposite delimiter when required for valid syntax. Triple-quoted strings containing `"` use
triple-single delimiters when doing so is safe.

## Side-effect imports

Tali mode treats an underscore alias as an explicit side-effect import:

```python
import package as _
```

This applies only to module imports. `from package import member as _` remains an unused import.

## Grouped signatures

Tali mode treats `/` and `*` as semantic block delimiters in multiline function signatures. It
formats parameters normally, then packs parameters within each block:

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

When a rectangular nested list or tuple uses extra whitespace to align columns, Tali mode formats
every cell normally and then enforces consistent column widths across the entire expression. This
supports normally formatted single-line expressions such as strings, unary numbers, attributes,
calls, and subscripts. Rows may be lists or tuples, and this works recursively for two-dimensional
and higher-dimensional data. Numeric columns align on their decimal point (or the implied
ones-place boundary for integers).

End-of-line comments on rows are retained and formatted normally. Whitespace used to line up row
comments also counts as alignment intent; the formatted comments remain aligned after cell widths
change. Unaligned, ragged, or multiline-cell sequences continue to use the standard formatter
layout.

## Nested container arguments

When a call's only argument is a multiline list, tuple, set, or dictionary, Tali mode hugs the
container delimiters instead of adding another indentation layer:

```python
render([
    first,
    second,
])
```

## Current limitations

The following Tali behavior is planned but not implemented:
- Optional alignment of comments, annotations, and assignments.
- Comment-aware line wrapping.

The implementation checklist is maintained in
[`TALI-MODE.md`](https://github.com/Demonstrandum/scruff/blob/master/TALI-MODE.md).
