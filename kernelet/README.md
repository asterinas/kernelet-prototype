# Kernelet

Kernelet runs kernels written in safe Rust inside Linux. A kernel uses the Rust APIs in [vOSTD](vostd/README.md). The [endovisor](endovisor/README.md) loads the kernel image, and a [host program](host/runtime/README.md) creates and runs each sandbox.

[Read the book online](https://asterinas.github.io/kernelet-prototype/) for the design and a walkthrough of the Hello World kernel.

## Building and running

Run these commands from `kernelet/` on an x86-64 Linux host with QEMU and access to `/dev/kvm`. The first build needs network access and at least 6 GiB free on the filesystem containing `.build/`.

Install rustup, GCC with static libc and pthread support, GNU Make, binutils, flex, bison, bc, Perl, libelf headers, Python 3, curl, patch, tar, xz, gzip, and `qemu-system-x86_64`. Rustup selects the toolchain in `rust-toolchain.toml`.

```sh
make hello    # Build and boot the Hello World kernel, then check its serial log
make test     # Run the full QEMU test suite
make bench    # Run the scheduling and memory benchmarks
```

The first boot downloads Linux 6.12, applies the patches in `linux/patches/`, and builds the guest kernel. Build products and serial logs are in `.build/`.

## Building the book

Use mdBook 0.5.2 and mdbook-mermaid 0.17.0. Run `make check build` to check the source references and generate `book/build/index.html`, or `make serve` for a local preview. See the [book README](book/README.md) for publishing instructions.
