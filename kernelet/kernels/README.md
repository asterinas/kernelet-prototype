# kernels: the kernels proper

Each subdirectory is one kernel proper, a crate compiled against vOSTD
(`../vostd`). The five teaching kernels embed a tenant program (`<name>.S`,
assembled by the Makefile into `<name>/<name>`); the PIE test runs entirely
in kernel mode. Each has a `README.md`. `make <name>` builds the image
with that kernel, boots it and checks the serial log; the Makefile's
`KLET_KERNEL` variable selects one.

| directory | the kernel | the book |
|---|---|---|
| `hello/` | Asterinas's 100-line Hello World kernel, byte for byte (`upstream/` is the original; the build compares the two before every `make hello`) | [The first kernel we will run](../book/src/introduction/the-first-kernel-we-will-run.md) |
| `pie/` | concurrent instances of one PIE kind, sharing physical code pages while checking private initialized data, BSS and relocated pointers; destroy and replace an instance while another runs | chapter 5 |
| `probe/` | demand paging, a refused allocation, exceptions that are not page faults, and the kills and signals that end a sandbox | chapters 10 and 14 |
| `yield/` | preemption of kernelet code, the watch timer, the forced yield | chapters 12 and 13 |
| `sched/` | a scheduler of the kernelet's own, fairness and the measurements of chapter 17 | chapters 11 and 17 |
| `mem/` | tenant memory on Linux page tables, and its measurements | chapters 15 to 17 |
