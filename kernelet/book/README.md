# Building Kernelets on Linux

This directory contains the mdBook source and its tooling. Start reading at
[How to read this book](src/README.md).

The book has two parts: **Accepted Design** describes the endorsed design and
the companion implementation in `../prototype/`; **Design Proposals** holds
ideas under discussion and the record of their review. To contribute an idea,
read the [proposal guide](src/design-proposals/README.md) and copy the
[template](src/design-proposals/template.md) into a dated proposal file.

| Path | Contents |
|---|---|
| `book.toml` | Book title, build settings, and preprocessors |
| `src/README.md`, `src/SUMMARY.md` | Introduction and table of contents |
| `src/accepted-design/` | The accepted design and implementation chapters |
| `src/design-proposals/` | Proposal guide, template, and dated proposals |
| `assets/` | Styles and diagram rendering scripts |
| `tools/` | Source checker and browser renderer |
| `build/` | Generated HTML, ignored by Git |

Run these commands from this directory:

```sh
make check
make build
make serve
make render PAGE=accepted-design/carrier-hooks.html OUT=/tmp/carrier-hooks.png
```

The build needs `mdbook` and `mdbook-mermaid`; rendering also needs a headless
Chrome browser. The `kernelet/Makefile` delegates the same book commands
here. `make clean` removes generated HTML and renderer output, keeping the book's
source and the prototype intact.

## Publishing

The book is published at <https://asterinas.github.io/kernelet-prototype/>.
The workflow in [`../../.github/workflows/kernelet-book-pages.yml`](../../.github/workflows/kernelet-book-pages.yml) checks and builds the book
when a push to `main` changes `kernelet/**` or the workflow, then deploys
`kernelet/book/build/` to the `asterinas/kernelet-prototype` repository's GitHub Pages site.
It can also be started manually from the repository's Actions tab on `main`.

The repository's **Settings → Pages → Build and deployment → Source** must be
set to **GitHub Actions**. The workflow uses GitHub's built-in token, with
`pages: write` and `id-token: write` scoped to the deployment job; no personal
access token or generated-site branch is needed. This follows GitHub's
[custom Pages workflow](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages).

The workflow pins mdBook and Mermaid to the versions documented in `kernelet/README.md`.
It sets the site's base path from Pages metadata when building, so the
generated 404 page can load assets under `/kernelet-prototype/`. A failed check or build
prevents deployment and leaves the previous published site in place.
