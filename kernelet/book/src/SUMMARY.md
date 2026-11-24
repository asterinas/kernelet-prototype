# Summary

[How to read this book](README.md)

# Introduction

- [Kernelets on Linux](introduction/README.md)
- [Why a kernel of your own](introduction/why-a-kernel-of-your-own.md)
- [The first kernel we will run](introduction/the-first-kernel-we-will-run.md)

# Design

- [System design](design/README.md)
- [The endovisor UAPI](design/the-endovisor-uapi.md)
- [The kernelet ABI](design/the-kernelet-abi.md)
- [Carrier hooks and patch overview](design/carrier-hooks.md)
- [Carriers](design/carriers.md)
- [Two stacks](design/two-stacks.md)
- [Into user mode and back](design/into-user-mode-and-back.md)
- [Tasks and virtual CPUs](design/tasks-and-virtual-cpus.md)
- [Virtual interrupts](design/virtual-interrupts.md)
- [Sharing a processor politely](design/sharing-a-processor-politely.md)
- [Stopping a kernelet](design/stopping-a-kernelet.md)
- [Tenant memory on Linux's page tables](design/tenant-memory-on-linux-page-tables.md)
- [What vOSTD adds on top](design/what-vostd-adds-on-top.md)
- [What it costs](design/what-it-costs.md)

- [Design proposals](design/proposals/README.md)
    - [Design proposal template](design/proposals/template.md)
    - [2026-10-31 Reduce syscall overhead](design/proposals/2026-10-31-reduce-syscall-overhead.md)
    - [2026-11-08 TypedIO](design/proposals/2026-11-08-typed-io.md)
    - [2026-11-09 FPU state management](design/proposals/2026-11-09-fpu-state-management.md)

# Endovisor

- [Endovisor interfaces](endovisor/README.md)

# Host Tools

- [Host tool interfaces](host/README.md)

# vOSTD

- [vOSTD API](vostd/README.md)
