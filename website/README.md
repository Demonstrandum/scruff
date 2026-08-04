# Scruff Tali showcase (GitHub Pages)

Static site documenting Tali mode formatting rules with before/after showcases
and amalgamated examples.

Deployed from this directory by `.github/workflows/github-pages.yml` to
GitHub Pages on pushes to `master` that touch `website/**`.

Local preview:

```sh
python -m http.server -d website 8080
```

Then open http://127.0.0.1:8080/
