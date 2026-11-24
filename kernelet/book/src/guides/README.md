# Using kernelets

Running a kernelet brings together a kernel written against vOSTD, an image loaded by the endovisor, and host tools that create and run a sandbox. This part is the entry point for guides covering that complete workflow.

Start with [Why a kernel of your own](why-a-kernel-of-your-own.md) for the motivation, basic concepts, and terminology. Then continue to [System design](../design/README.md) for the mechanisms that make the components work together.

[The first kernel we will run](the-first-kernel-we-will-run.md) explains the Hello World kernel and the services it needs. [The endovisor UAPI](../design/the-endovisor-uapi.md#a-prototype-kernelet-runtime) describes how the host runtime registers an image and runs it.

For build requirements, commands, and the QEMU test suite, see the [implementation guide](https://github.com/asterinas/kernelet-prototype/blob/main/kernelet/IMPLEMENTATION.md).
