# Scruff

Scruff is an opinionated fork of [Ruff](https://github.com/astral-sh/ruff), the fast Python linter
and formatter written in Rust. It tracks Ruff while adding formatting and linting behavior for
projects that want stronger style conventions.

The fork currently adds:

- `mode = "tali"`, an evolving opinionated formatting and linting mode.
- `quote-style = "symbol"`, which uses single quotes for symbol-like strings and double quotes for
    natural-language text.
- `quote-symbol-regex`, for customizing which strings are treated as symbols.
- `[tool.scruff]`, `scruff.toml`, and `.scruff.toml` configuration, while retaining Ruff
    configuration names as compatibility aliases.

See [TALI-MODE.md](TALI-MODE.md) for the Tali roadmap and [examples](examples/) for complete
configurations.

## Installation

Install Scruff from PyPI:

```console
pip install scruff
```

Or run the container:

```console
docker run --rm -v .:/io ghcr.io/demonstrandum/scruff check
```

## Usage

Lint the current directory:

```console
scruff check .
```

Format the current directory:

```console
scruff format .
```

Enable the fork-specific mode in `pyproject.toml`:

```toml
[tool.scruff]
mode = "tali"

[tool.scruff.format]
quote-style = "symbol"
```

The equivalent standalone configuration can be placed in `scruff.toml` or `.scruff.toml` without
the `[tool.scruff]` table.

## Status

Scruff is usable as a linter and formatter, but the Tali roadmap is not complete. In particular,
f-string symbol quoting, side-effect import syntax, alignment, comment-aware line wrapping, and
nested-bracket hugging are still planned.

Compatibility with Ruff is maintained where practical, but Scruff's opinionated modes can
intentionally produce different output. Pin Scruff's version in automated environments.

## Development

Build and run Scruff:

```console
cargo run --bin scruff -- check path/to/file.py
cargo run --bin scruff -- format path/to/file.py
```

Run the main CLI tests:

```console
CARGO_PROFILE_DEV_OPT_LEVEL=1 \
INSTA_FORCE_PASS=1 \
INSTA_UPDATE=always \
CARGO_PROFILE_DEV_DEBUG="line-tables-only" \
MDTEST_UPDATE_SNAPSHOTS=1 \
cargo nextest run -p scruff
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contributor workflow.

## Upstream

Most of Scruff's implementation comes from Ruff and remains under Ruff's MIT license. Internal
crates retain their `ruff_*` names to keep upstream merges manageable. Ruff configuration files,
environment variables, rule codes, and suppression syntax may also remain supported for
compatibility; those names are not product-branding mistakes.

Scruff is maintained at [Demonstrandum/scruff](https://github.com/Demonstrandum/scruff).
