# Scruff Tali showcase (GitHub Pages)

Static site documenting Tali mode formatting rules with before/after showcases
and amalgamated examples.

`.github/workflows/github-pages.yml` publishes this directory to the `gh-pages`
branch:

- pushes to `master` publish to the Pages root
- pull requests touching `website/**` publish a preview at
    `/pr-preview/pr-<number>/` and get a comment linking to it

Pages must be configured as **Deploy from a branch** using `gh-pages` (root).

Local preview:

```sh
python -m http.server -d website 8080
```

Then open <http://127.0.0.1:8080/>
