#![cfg(feature = "baremetal")]
#![no_std]

//! Placeholder bare.rs module for baremetal support.
//! Implement actual feature and topology detection for baremetal targets as needed.

use crate::common::{CoreTypeInfo, CpuFeatures, CpuTopology};

/// Minimal bare-metal feature detection (returns default features).
pub fn get_features() -> CpuFeatures {
    CpuFeatures::default()
}

/// Minimal bare-metal topology (returns no hybrid, empty core_types).
pub fn get_topology() -> CpuTopology {
    CpuTopology {
        is_hybrid: false,
        core_types: Vec::new(),
    }
}
