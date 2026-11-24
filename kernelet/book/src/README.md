# How to read this book

**Building Kernelets on Linux** explains the design and implementation of the **kernelet**: a complete OS kernel that runs *inside* another kernel, and does so *safely*.

The book is organized around system design, usage guides, and the interfaces exposed by the three main components:

| Part | What it covers |
|---|---|
| [Guides](guides/README.md) | Motivation, basic concepts, and using the complete system |
| [Design](design/README.md) | The accepted design and implementation, including mechanisms that require several components to cooperate |
| [Endovisor](endovisor/README.md) | The UAPI exposed to host programs and the host services exposed to vOSTD |
| [Host Tools](host/README.md) | The command-line interface exposed to users |
| [vOSTD](vostd/README.md) | The Rust API exposed to kernelet kernel code |

[Design proposals](design/proposals/README.md) live within Design. They develop and review optimizations, alternatives, and new material. A proposal is not a promise about the accepted design or the implementation's behavior. Accepted proposals are incorporated into the relevant chapters and, where applicable, the implementation. Declined or withdrawn proposals remain available with the reasons for the decision.

Start with [Why a kernel of your own](guides/why-a-kernel-of-your-own.md) to understand the motivation and concepts, then continue to [System design](design/README.md). To suggest a change, read [Contributing a design proposal](design/proposals/README.md) and use the [proposal template](design/proposals/template.md). Each proposal's title and filename begin with its original submission date, such as `2026-10-27 Improve syscall performance` and `2026-10-27-improve-syscall-performance.md`.

To build and boot the example kernel, follow [Building and running](https://github.com/asterinas/kernelet-prototype/blob/main/kernelet/README.md#building-and-running).

## Design explanations and interface specifications

**Design explains how and why the system works:** principles, tradeoffs, implementation mechanisms, and cooperation between components. Its UAPI and ABI chapters explain interface design through selected details and simplified examples; they are not complete specifications.

**Endovisor, Host Tools, and vOSTD define the interfaces each component exposes.** Their API references and specifications state the contract: operations, types, behavior, errors, caller requirements, and compatibility limits, including binary layouts and calling conventions for ABIs. These specifications stand on their own; design chapters provide supporting rationale.

## Who this is for

The book assumes three things: what one operating-systems course teaches, the ability to read C, and a fair command of Rust. From the course you know what a process is, what virtual memory and a page table are, what a system call does, and what an interrupt and a context switch are, in the abstract.

It assumes nothing about the Linux source. Background knowledge beyond the course, whether about Linux, ELF, compiler behavior or processor details, is explained the first time it is needed in a **background box**. Each box begins with a subject and a topic, such as **Linux: control groups** or **ELF: relocations**. It explains the concept, the names used for it in the relevant code or format, and why it matters to the kernelet design.

<div class="background">

**Linux: control groups.** A control group is a set of tasks that Linux accounts and limits together. The host runtime puts a sandbox's carrier tasks in one group so that Linux can account for their resource use together.

</div>

If you already know a box's topic, you can continue with the main text. The same box style is used for every subject; the label tells you which background knowledge it introduces.

## Code excerpts

Source paths in code captions are relative to `kernelet/`. Design proposals may also show proposed code; they must distinguish it from code already in the implementation.

A caption under each excerpt names the file it comes from and, for a simplified excerpt, the functions it shows, like this:

*Simplified from `endovisor/switch.S`: `klet_switch`.*

The caption's first word says how the block differs from the file. "Simplified" means that logging, counters, error paths and comments have been cut so that the mechanism fits on the page. "Unchanged" means that the block is copied from the file verbatim.

## The Accepted Design chapters

Each chapter uses only what the ones before it have built.

The terminology used below is introduced in [Why a kernel of your own](guides/why-a-kernel-of-your-own.md#the-words). [The first kernel we will run](guides/the-first-kernel-we-will-run.md) walks through the hundred-line example kernel used by the design chapters.

1. **System design** introduces the scope and reading order of the design chapters.
2. **The endovisor UAPI**, its user-space interface, is what a program on the host may ask of the endovisor: the image as a file, the device node and its descriptors, and how user space creates the carriers.
3. **The kernelet ABI**, its binary interface, is how a kernelet talks to its host from the inside: the tables passed between host and kernelet, the grant, and the first services.
4. **Carrier hooks and patch overview** adds the two carrier event hooks, where a carrier enters and leaves kernelet code, and lists the rest of the small patch Kernelet applies to Linux.
5. **Carriers** puts a kernelet on the Linux tasks it runs on.
6. **Two stacks** is how a kernelet keeps its own stacks on a Linux task.
7. **Into user mode and back** is the round trip of a system call, after which the hundred-line kernel prints its greeting.
8. **Tasks and virtual CPUs** gives the kernelet its own tasks and its own scheduler.
9. **Virtual interrupts** gives it a timer.
10. **Sharing a processor politely** keeps it from holding a processor against Linux.
11. **Stopping a kernelet** removes it, even when it will not cooperate.
12. **Tenant memory on Linux's page tables** puts the tenant's memory in a Linux address space that the kernelet edits only by asking.
13. **What vOSTD adds on top** is the bookkeeping that keeps that arrangement memory-safe, or *sound*, as introduced in [Why a kernel of your own](guides/why-a-kernel-of-your-own.md#a-kernel-in-two-layers).
14. **What it costs** gives the numbers.

Chapters 12 and 13, where the tenant's memory moves onto Linux's page tables without giving up soundness, are the climax; everything before them prepares for them.

## Conventions

The book keeps three conventions.

- A few words are used in exactly one sense throughout; the [introductory word table](guides/why-a-kernel-of-your-own.md#the-words) collects the first of them, and the chapter named beside a word there defines it in full.
- Linux functions are linked to the version 6.12 source on `elixir.bootlin.com` the first time they are named, so that you can read the real code.
- Most figures are diagrams written as text in the page source; a few are drawings.
