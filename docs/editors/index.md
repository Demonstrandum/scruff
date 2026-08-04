# Editor Integrations

Scruff can be integrated with various editors and IDEs to provide a seamless development experience.
This section provides instructions on [how to set up Scruff with your editor](./setup.md) and [configure it to your
liking](./settings.md).

## Language Server Protocol

The editor integration is mainly powered by the Scruff language server, which implements the
[Language Server Protocol](https://microsoft.github.io/language-server-protocol/). The server is
written in Rust and is available through `scruff server`. It is built directly into Scruff and is a
replacement for the older Python-based `ruff-lsp` implementation.

The server supports surfacing Scruff diagnostics, providing Code Actions to fix them, and
formatting code using Scruff's built-in formatter. Currently, the server is intended to be used
alongside another Python Language Server in order to support features like navigation and
autocompletion.

!!! note

    This is the documentation for Scruff's built-in language server (`scruff server`).
