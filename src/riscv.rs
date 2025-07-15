#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]

//! Placeholder riscv.rs module for RISC-V support.
//! Implement actual feature and topology detection for RISC-V targets as needed.

use crate::common::{CoreTypeInfo, CpuFeatures, CpuTopology};

/// Minimal RISC-V feature detection (returns default features).
pub fn get_features() -> CpuFeatures {
    CpuFeatures::default()
}

/// Minimal RISC-V topology (returns no hybrid, empty core_types).
pub fn get_topology() -> CpuTopology {
    CpuTopology {
        is_hybrid: false,
        core_types: Vec::new(),
    }
}
