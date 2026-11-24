# Contributing a design proposal

Design Proposals is the section within Design for ideas that have not yet become part of the accepted design. A proposal can suggest an optimization, explore an alternative design, or develop new explanatory material. It should help readers understand the problem, assess the evidence, and discuss the tradeoffs. Working code is useful evidence, but it does not by itself make a proposal accepted.

## Starting a proposal

Copy the [proposal template](template.md) into a new file in `book/src/design/proposals/`. Replace the placeholders with the proposal's details and add a child link under **Design proposals** in `book/src/SUMMARY.md`. Use the same dated title for the page and its table-of-contents entry.

Both the title and filename start with the original submission date in `YYYY-MM-DD` form:

- Title: `2026-10-27 Improve syscall performance`
- File: `2026-10-27-improve-syscall-performance.md`

Use a short, descriptive filename with hyphens between words. Keep the original date when revising the proposal so that its identity and links remain stable. Different proposals submitted on the same day have different descriptive filenames. This guide and the reusable template are not proposals and do not need date prefixes.

Link to the affected design or component chapters, relevant code, experiments, and discussion. Links from a proposal to an accepted design chapter use `../<chapter>.md`. Source paths written in prose are relative to `kernelet/`. Label proposed code clearly; reserve “Unchanged from” captions for excerpts that match an existing source file exactly.

For an interface change, distinguish the design rationale from the proposed contract. Identify the component specification that needs to be created or updated, including changes to signatures, behavior, errors, and caller requirements. Follow [Design explanations and interface specifications](../../README.md#design-explanations-and-interface-specifications); a proposal or design example is not a substitute for the resulting specification.

Run `make check` and `make build` from the repository root, or use the same commands from `book/`, before submitting the change for discussion and review. The checker verifies proposal dates and links as well as source excerpts. Proposals can cite external research and discussion; the checker does not verify those external pages.

## Review and decisions

Use one of these statuses in the proposal:

| Status | Meaning |
|---|---|
| Draft | The author is developing the idea and identifying open questions. |
| Under review | The proposal is ready for discussion and evaluation. |
| Accepted | The agreed changes have been incorporated into the relevant chapters and, where applicable, the prototype. |
| Declined | Review concluded that the proposal should not be incorporated. |
| Withdrawn | The author has stopped pursuing the proposal. |

During review, revise the proposal as evidence and discussion resolve questions. Record the decision and its rationale, including any differences between the original idea and the change actually adopted. For an accepted proposal, link to the resulting chapters and implementation. Keep the proposal in this section as a record of the reasoning, with its status visible; the updated design or component chapters describe the resulting system.

A proposal may need an experiment or prototype implementation before a decision can be made. State what was implemented and tested, what remains hypothetical, and which results would support or undermine the proposal. For new explanatory material, show where it belongs and how it improves the reader's understanding. The amount of detail should match the size of the change.
