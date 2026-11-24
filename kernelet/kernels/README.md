# kernels: the kernels proper

Each subdirectory is one kernel proper, a crate compiled against vOSTD
(`../vostd`). The five teaching kernels embed a tenant program (`<name>.S`,
assembled by the Makefile into `<name>/<name>`); the PIE test runs entirely
in kernel mode. Each has a `README.md`. `make <name>` builds the image
with that kernel, boots it and checks the serial log; the Makefile's
`KLET_KERNEL` variable selects one.

| directory | the kernel | the book |
|---|---|---|
| `hello/` | Asterinas's 100-line Hello World kernel, byte for byte (`upstream/` is the original; the build compares the two before every `make hello`) | [The first kernel we will run](../book/src/guides/the-first-kernel-we-will-run.md) |
| `pie/` | concurrent instances of one PIE kind, sharing physical code pages while checking private initialized data, BSS and relocated pointers; destroy and replace an instance while another runs | chapter 2 |
| `probe/` | demand paging, a refused allocation, exceptions that are not page faults, and the kills and signals that end a sandbox | chapters 7 and 11 |
| `yield/` | preemption of kernelet code, the watch timer, the forced yield | chapters 9 and 10 |
| `sched/` | a scheduler of the kernelet's own, fairness and the measurements of chapter 14 | chapters 8 and 14 |
| `mem/` | tenant memory on Linux page tables, and its measurements | chapters 12 to 14 |
