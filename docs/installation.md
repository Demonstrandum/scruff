# Installing Scruff

Scruff is available as [`scruff`](https://pypi.org/project/scruff/) on PyPI.

Run it directly with [`uvx`](https://docs.astral.sh/uv/):

```shell
uvx scruff check   # Lint all files in the current directory.
uvx scruff format  # Format all files in the current directory.
```

Or install it with `uv`, `pip`, or `pipx`:

```console
$ uv tool install scruff@latest
$ uv add --dev scruff
$ pip install scruff
$ pipx install scruff
```

Once installed, run Scruff from the command line:

```console
$ scruff check
$ scruff format
```

Release archives and standalone installers are published on the
[GitHub releases page](https://github.com/Demonstrandum/scruff/releases).

## Docker

The container image is published as `ghcr.io/demonstrandum/scruff`. Each release has a version tag,
and the newest stable release is also tagged `latest`.

```console
$ docker run -v .:/io --rm ghcr.io/demonstrandum/scruff check
$ docker run -v .:/io --rm ghcr.io/demonstrandum/scruff:0.15.21 check

$ # Or, for Podman on SELinux.
$ podman run -v .:/io:Z --rm ghcr.io/demonstrandum/scruff check
```

Distribution packages named `ruff` install upstream Ruff, not Scruff.
