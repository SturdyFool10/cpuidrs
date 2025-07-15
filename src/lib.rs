#![cfg_attr(feature = "baremetal", no_std)]
#![cfg_attr(not(feature = "baremetal"), allow(unused))]

//! Cross-platform CPU feature and topology detection
//!
//! Supports:
//! - **Operating Systems**: Windows, macOS, Linux, Bare-metal
//! - **Architectures**: x86/x86_64, ARM/AArch64, RISC-V
//! - **Topology**: Hybrid core detection and grouping

#[cfg(all(
    not(feature = "baremetal"),
    any(target_arch = "arm", target_arch = "aarch64")
))]
mod arm;
#[cfg(feature = "baremetal")]
mod bare;
mod common;
#[cfg(all(
    not(feature = "baremetal"),
    any(target_arch = "riscv32", target_arch = "riscv64")
))]
mod riscv;
#[cfg(all(
    not(feature = "baremetal"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
mod x86;

pub use common::{CoreTypeInfo, CpuFeatures, CpuInfo, CpuTopology};

/// Detect CPU features and topology for the current platform.
///
/// # Examples
///
/// ```no_run
/// let info = cpuid::detect();
/// println!("CPU: {}\nFeatures: {:?}\nTopology: {:?}",
///          info.architecture, info.features, info.topology);
/// ```
pub fn detect() -> CpuInfo {
    common::detect()
}
