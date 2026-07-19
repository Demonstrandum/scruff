# /// script
# requires-python = ">=3.13"
# dependencies = []
# ///

from __future__ import annotations

import json
import pathlib
import subprocess

GENERATED_HEADER = "<!-- This file is generated. DO NOT EDIT -->"
GENERATED_SECTION_START = "<!-- BEGIN GENERATED CRATE VERSIONING -->"
GENERATED_SECTION_END = "<!-- END GENERATED CRATE VERSIONING -->"

SCRUFF_TEMPLATE = """{GENERATED_HEADER}

# Scruff

Scruff is a fork of Ruff with additional, opinionated linting and formatting modes.

See the [repository](https://github.com/Demonstrandum/scruff) for more information.

This crate is the entry point to the Scruff command-line interface. The Rust API exposed here is not
considered public interface.

This is version {scruff_version}. The source can be found [here]({source_url}).

The following Scruff workspace members are also available:

{WORKSPACE_MEMBERS}

Scruff's workspace members are considered internal and will have frequent breaking changes.
"""


MEMBER_VERSIONING_TEMPLATE = """\
This crate is an internal component of [Scruff](https://crates.io/crates/scruff). The Rust API exposed
here is unstable and will have frequent breaking changes.

This version ({crate_version}) is a component of [Scruff {scruff_version}]({scruff_crates_io_url}). The
source can be found [here]({source_url}).
"""

MEMBER_TEMPLATE = """{GENERATED_HEADER}

# {name}

{versioning}"""


REPO_URL = "https://github.com/Demonstrandum/scruff"
PRETTIER_VERSION = "3.8.3"


def replace_generated_section(readme: str, generated_content: str) -> str:
    start = readme.find(GENERATED_SECTION_START)
    end = readme.find(GENERATED_SECTION_END, start)
    if start == -1 or end == -1:
        raise ValueError("README has an incomplete generated crate versioning section")

    content_start = start + len(GENERATED_SECTION_START)
    return (
        readme[:content_start]
        + "\n\n"
        + generated_content.strip()
        + "\n\n"
        + readme[end:]
    )


def main() -> None:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        capture_output=True,
        text=True,
        check=True,
    )
    content = json.loads(result.stdout)
    packages = {package["id"]: package for package in content["packages"]}

    # Find the Scruff version from the main crate.
    scruff_version = None
    for package in content["packages"]:
        if package["name"] == "scruff":
            scruff_version = package["version"]
            break
    if scruff_version is None:
        raise RuntimeError("Could not find scruff crate")

    workspace_root = pathlib.Path(content["workspace_root"])
    readme_path = workspace_root / "crates" / "scruff" / "README.md"

    workspace_members = []
    for workspace_member in content["workspace_members"]:
        package = packages[workspace_member]
        name = package["name"]
        # Skip the main Scruff crate.
        if name == "scruff":
            continue
        # Skip crates with publish = false.
        if package.get("publish") == []:
            continue
        workspace_members.append(name)

    workspace_members.sort()

    members_list = "\n".join(
        f"- [{name}](https://crates.io/crates/{name})" for name in workspace_members
    )

    # Generate the README for the main Scruff crate.
    scruff_source_url = f"{REPO_URL}/blob/{scruff_version}/crates/scruff"
    readme_content = SCRUFF_TEMPLATE.format(
        GENERATED_HEADER=GENERATED_HEADER,
        WORKSPACE_MEMBERS=members_list,
        scruff_version=scruff_version,
        source_url=scruff_source_url,
    )
    readme_path.write_text(readme_content)

    # Track all generated README paths for formatting at the end.
    generated_paths = [readme_path]

    # Generate READMEs for all workspace members.
    for workspace_member in content["workspace_members"]:
        package = packages[workspace_member]
        name = package["name"]

        # Skip the main Scruff crate (already handled above).
        if name == "scruff":
            continue
        # Skip crates that aren't released to crates.io.
        if package.get("publish") == []:
            continue

        manifest_path = pathlib.Path(package["manifest_path"])
        crate_dir = manifest_path.parent
        member_readme_path = crate_dir / "README.md"

        handwritten_readme = None
        # Preserve hand-written crate READMEs unless they opt in to a generated versioning section.
        if member_readme_path.exists():
            existing_content = member_readme_path.read_text()
            if not existing_content.startswith(GENERATED_HEADER):
                if (
                    GENERATED_SECTION_START not in existing_content
                    and GENERATED_SECTION_END not in existing_content
                ):
                    print(f"Skipping {name}: existing README without generated header")
                    continue
                handwritten_readme = existing_content

        crate_version = package["version"]
        relative_crate_path = crate_dir.relative_to(workspace_root)
        source_url = f"{REPO_URL}/blob/{scruff_version}/{relative_crate_path}"

        scruff_crates_io_url = f"https://crates.io/crates/scruff/{scruff_version}"
        member_versioning_content = MEMBER_VERSIONING_TEMPLATE.format(
            crate_version=crate_version,
            scruff_version=scruff_version,
            scruff_crates_io_url=scruff_crates_io_url,
            source_url=source_url,
        )
        if handwritten_readme is None:
            member_readme_content = MEMBER_TEMPLATE.format(
                GENERATED_HEADER=GENERATED_HEADER,
                name=name,
                versioning=member_versioning_content,
            )
        else:
            member_readme_content = replace_generated_section(
                handwritten_readme, member_versioning_content
            )
        member_readme_path.write_text(member_readme_content)
        generated_paths.append(member_readme_path)

        print(f"Generated README for {name}")

    # Format all generated READMEs once at the end.
    subprocess.run(
        ["npx", "--yes", f"prettier@{PRETTIER_VERSION}", "--write"]
        + [str(path) for path in generated_paths],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


if __name__ == "__main__":
    main()
