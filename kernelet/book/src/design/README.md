# System design

This part explains how the host tools, endovisor, and vOSTD cooperate to run kernelets inside Linux. It covers execution, scheduling, virtual interrupts, memory management, and termination, including the principles and tradeoffs behind the interfaces.

The chapters build on one another, starting with [The endovisor UAPI](the-endovisor-uapi.md). For motivation, basic concepts, and terminology, read [Why a kernel of your own](../guides/why-a-kernel-of-your-own.md) in Guides first. [The first kernel we will run](../guides/the-first-kernel-we-will-run.md) explains the example kernel used throughout the design. Proposed changes and their review history are collected in [Design proposals](proposals/README.md).
