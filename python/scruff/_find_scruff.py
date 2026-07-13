from __future__ import annotations

import os
import sys
import sysconfig


class ScruffNotFound(FileNotFoundError): ...


def find_scruff_bin() -> str:
    """Return the Scruff binary path."""

    scruff_exe = "scruff" + sysconfig.get_config_var("EXE")

    targets = [
        sysconfig.get_path("scripts"),
        sysconfig.get_path("scripts", vars={"base": sys.base_prefix}),
        (
            _join(
                _matching_parents(_module_path(), "Lib/site-packages/scruff"),
                "Scripts",
            )
            if sys.platform == "win32"
            else _join(
                _matching_parents(_module_path(), "lib/python*/site-packages/scruff"),
                "bin",
            )
        ),
        _join(_matching_parents(_module_path(), "scruff"), "bin"),
        sysconfig.get_path("scripts", scheme=_user_scheme()),
    ]

    seen = []
    for target in targets:
        if not target or target in seen:
            continue
        seen.append(target)
        path = os.path.join(target, scruff_exe)
        if os.path.isfile(path):
            return path

    locations = "\n".join(f" - {target}" for target in seen)
    raise ScruffNotFound(
        "Could not find the Scruff binary in any of the following locations:"
        f"\n{locations}\n"
    )


def _module_path() -> str:
    return os.path.dirname(__file__)


def _matching_parents(path: str | None, match: str) -> str | None:
    """Return the parent after trimming a slash-separated match from the path."""
    from fnmatch import fnmatch

    if not path:
        return None
    parts = path.split(os.sep)
    match_parts = match.split("/")
    if len(parts) < len(match_parts):
        return None
    if not all(
        fnmatch(part, match_part)
        for part, match_part in zip(reversed(parts), reversed(match_parts))
    ):
        return None
    return os.sep.join(parts[: -len(match_parts)])


def _join(path: str | None, *parts: str) -> str | None:
    if not path:
        return None
    return os.path.join(path, *parts)


def _user_scheme() -> str:
    if sys.version_info >= (3, 10):
        return sysconfig.get_preferred_scheme("user")
    if os.name == "nt":
        return "nt_user"
    if sys.platform == "darwin" and sys._framework:  # ty: ignore[unresolved-attribute]
        return "osx_framework_user"
    return "posix_user"
