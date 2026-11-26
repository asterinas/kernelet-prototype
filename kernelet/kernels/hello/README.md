# hello: the 100-line kernel, unedited

The kernel proper of `make hello` is Asterinas's "write a kernel in 100 lines"
example: a Hello World kernel in safe Rust against OSTD, which loads a tenant
program, gives it an address space and a user context, and serves its `write`
and `exit` system calls. The crate's `src/lib.rs` and `hello.S` link to the
[OSDK teaching example](../../../osdk/tests/examples_in_book/write_a_kernel_in_100_lines_templates/).
Kernelet compiles the same sources against vOSTD.

The book walks through this kernel and the OSTD API it uses in
[The first kernel we will run](../../book/src/introduction/the-first-kernel-we-will-run.md); the rest of the book is what it takes for this file to
run on Linux.

## How it is built

`make image` (the default `KLET_KERNEL=hello`) builds the crate with vOSTD as
its `ostd` and pre-links both into the image, `.build/hello.image`.
`hello.S` is assembled with `gcc -static -nostdlib` into `kernels/hello/hello`, which
the kernel embeds with `include_bytes!("../hello")`.
