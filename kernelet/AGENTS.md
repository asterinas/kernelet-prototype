# Documentation conventions

## Repository and book structure

- These instructions apply within `kernelet/`. References below to the repository root, root Makefile, and code-caption paths mean this directory; run commands here or use `make -C kernelet <target>` from the enclosing repository. The publishing workflow lives at `../.github/workflows/kernelet-book-pages.yml`.
- `book/` contains the mdBook source, assets, and tools; `endovisor/`, `host/`, and `vostd/` contain the main components. Shared instructions and build shortcuts stay at the repository root.
- The book is titled **Building Kernelets on Linux**. Its top-level sections are **Introduction**, **Design**, **Endovisor**, **Host Tools**, and **vOSTD**, with **Design Proposals** nested under Design.
- Design chapters live in `book/src/design/` and retain their reading order. The motivation and terminology chapter, `why-a-kernel-of-your-own.md`, and the first-kernel walkthrough, `the-first-kernel-we-will-run.md`, live in `book/src/introduction/`. Proposed changes live in `book/src/design/proposals/`, using the guide and template there.
- `book/src/introduction/` introduces kernelets through their motivation, terminology, and the Hello World kernel. `book/src/endovisor/`, `book/src/host/`, and `book/src/vostd/` document only the interfaces each component exposes and their contracts (UAPI, ABI, CLI, or Rust API as appropriate). Design rationale, implementation mechanisms, cooperation between components, and soundness arguments belong in `design/`.
- Keep the remaining existing chapters, including interface descriptions, in `design/` for now. Obtain the user's confirmation for each subsequent chapter move or extraction of interface documentation; do not redistribute design chapters by component.
- Each proposal's filename starts with its original submission date: `YYYY-MM-DD-descriptive-title.md`. Its heading and table-of-contents label start with the same date followed by the title. Keep that date when revising the proposal.
- A proposal's status distinguishes discussion from acceptance. When accepted, integrate the agreed changes into the relevant design or component chapters and the implementation where applicable; retain the proposal and decision rationale as history.
- Code-caption paths such as `endovisor/image.c` are relative to the repository root. Label proposed code separately from excerpts of existing implementation.
- Run book commands from `book/`, or use the root Makefile's shortcuts. Generated HTML belongs in `book/build/`, never alongside the book source.

## Design explanations and interface specifications

- Follow the distinction documented in [How to read this book](book/src/README.md#design-explanations-and-interface-specifications). Design chapters explain principles, tradeoffs, mechanisms, and cooperation. Component sections independently define the concrete specifications and API references for the interfaces each component exposes.
- A component specification must state the contract explicitly: declarations, behavior, errors, caller requirements, ownership and lifetimes, concurrency and blocking rules, and compatibility limits as applicable. Check it against the implementation and confirmed requirements; identify unsupported or unresolved behavior.
- Do not use a design chapter, a simplified excerpt, or a link to either as a substitute for a specification. Design chapters should reference the concrete specifications; specifications may link back only for explanatory background, not to supply required behavior.
- For an unwritten section, keep at most a one- or two-sentence HTML TODO comment in its README describing its intended content. Do not add visible pending-status notices or lists of work to be done. Existing design chapters remain in `design/`; writing a concrete specification is a separate task from moving or indexing those chapters.

## Background knowledge in the book

- Introduce background beyond the assumed operating-systems course and C/Rust knowledge at its first use in a background box. This applies to Linux, ELF, compilers, processor architecture, and other subjects.
- Use `<div class="background">` with blank lines around the Markdown content and before the closing `</div>`.
- Begin with a bold label in the form `**Subject: topic.**`, such as `**Linux: control groups.**` or `**ELF: relocations.**`. Use the same visual style for every subject.
- Explain the concept, the terminology used in the relevant source or format, and its relevance to the design. Keep the box focused on the knowledge needed at that point.
- Use the bold label instead of a chapter subsection heading for the background explanation. Keep the main argument and implementation requirements in the surrounding text.

## C comments

These conventions apply to all C comments in source files, headers, and
the book's C code excerpts. The basic principle is to model documentation prose on Rust documentation comments. More specifically,

- Prefer `//` to block comments, including short field annotations.
- Begin with a concise, complete summary sentence describing the documented item.
  - Short comments beside fields needn’t be complete sentences:
- Follow the summary with the details needed to understand the item.
- Group related information into paragraphs separated by an empty `//` line.
- Group related paragraphs under a section.
- Wrap identifiers, types, field names, error constants, paths, and code expressions in backticks within comments.
- Use semantic line breaks: start each sentence on a new line, and split long sentences at clause or phrase boundaries. Keep closely related words together rather than wrapping mechanically at a fixed width.
  - Limit the use of semantic line breaks in source code; documentation such as README and Markdown files do not follow semantic line breaks. 

## Ioctl definitions

Apply the general C comment conventions to every ioctl in the UAPI header
and keep the book's excerpts consistent with that header.

- Begin with a summary of the operation,
  such as “The ioctl for doing sth.”
- Document the arguments, behavior, constraints, return value, and relevant errors.
  Explain privilege requirements under `// # Security` when applicable.

## C code excerpts in the book

- Keep excerpts labeled “Unchanged” verbatim with their source, including comments. Use C code fences for C declarations, even though the comments follow Rust's style.
- Run `make check` and `make build` after updating the book and its source excerpts.
