# Building Kernelets on Linux

A book and working prototype about running a kernel of your own inside Linux.

[Read the book online](https://asterinas.github.io/kernelet-prototype/).

The prototype has its own Cargo workspace, lockfile, and Rust toolchain. Paths in this book and the commands below are relative to `kernelet/`; from the repository root, use `make -C kernelet <target>`.

The publishing workflow lives at [`../.github/workflows/kernelet-book-pages.yml`](../.github/workflows/kernelet-book-pages.yml). It checks and publishes this book to `https://asterinas.github.io/kernelet-prototype/` when `main` receives changes under `kernelet/` or to the workflow. See [publishing setup](book/README.md#publishing).

A **kernelet** is a complete kernel, written in safe Rust against the Asterinas kernel framework, that runs *inside* a host kernel as an ordinary piece of loaded code and is kept apart from it by the language rather than by hardware. This repository holds a book that explains how that works on Linux, chapter by chapter, and the prototype the book describes: a small kernel running as a kernelet inside a patched Linux 6.12, serving a user program's system calls, managing its memory in a Linux page table, scheduled and stopped by Linux.

Read the [book](book/src/README.md) for the design and the [implementation guide](IMPLEMENTATION.md) for the source layout, build requirements, and QEMU tests.

The book covers system design, usage guides, and the interfaces exposed by Endovisor, Host Tools, and vOSTD. **Design Proposals**, under `design/proposals/`, holds optimizations, alternatives, and new material for discussion and review. Accepted proposals are incorporated into the relevant chapters and implementation; other proposals retain their decision history. Use the [proposal guide](book/src/design/proposals/README.md) and [template](book/src/design/proposals/template.md) to contribute. Each proposal's title and filename start with its submission date, for example `2026-10-27 Improve syscall performance` and `2026-10-27-improve-syscall-performance.md`.

The `Makefile` in this directory provides shortcuts for both parts, and `AGENTS.md` records shared contribution conventions. The repository's Asterinas formatting, spelling, and license-header checks exclude `kernelet/`; use the checks described below for the book and prototype.

## Building the book

```sh
make build          # into book/build/; needs mdbook and mdbook-mermaid
make check          # every link resolves; every file and identifier the captions name exists in the source tree
make render PAGE=design/carrier-hooks.html OUT=/tmp/page.png   # screenshot a page and check its diagrams rendered
```

These commands delegate to `book/Makefile`; they also work directly from `book/`. The generated site starts at `book/build/index.html`.

The book is built with mdbook 0.5.2 and mdbook-mermaid 0.17.0. `make build` then prints `Warning: The mdbook-mermaid preprocessor was built against version 0.5.0 of mdbook, but we're being called from version 0.5.2`; it is harmless, and the diagrams render.

## Building and running the prototype

```sh
make crates         # once, with a network: the Rust crates (make test does it too if they are missing)
make test           # build everything, boot it in QEMU, run the tests the book mentions
make bench          # the measurements of the last chapter
```

Run individual tests with targets such as `make hello`. To remove build products, use `make clean` for the book and `make clean-runtime` for the implementation.
