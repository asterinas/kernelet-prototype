// SPDX-License-Identifier: MPL-2.0

//! Architecture-specific code. vOSTD supports x86-64.

pub mod cpu;
pub(crate) mod irq;
pub mod mm;
pub mod task;
pub mod trap;

pub use crate::time::read_tsc;
