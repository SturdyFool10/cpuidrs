#![allow(dead_code)]
use core::fmt;
#[cfg(any(target_os = "linux", not(target_os = "linux")))]
use libc::{sysconf, _SC_NPROCESSORS_ONLN};
#[cfg(target_os = "linux")]
use std::fs;

bitflags! {
    #[derive(Debug)]
    /// Packed feature flags for RISC-V
    pub struct RiscVFeatures: u32 {
        const I = 1 << 0;  // Base integer ISA
        const M = 1 << 1;  // Integer multiply/divide
        const A = 1 << 2;  // Atomic instructions
        const F = 1 << 3;  // Single-precision FP
        const D = 1 << 4;  // Double-precision FP
        const C = 1 << 5;  // Compressed instructions
    }
}

/// RISC-V CPU information
#[derive(Debug)]
pub struct RiscVCpuInfo {
    pub vendor: String,
    pub brand: String,
    pub features: RiscVFeatures,
    pub cores: u32,
    pub threads_per_core: u32,
}

/// Gather RISC-V CPU info
pub fn gather() -> RiscVCpuInfo {
    // Vendor & ISA parsing
    let (vendor, brand, features) = {
        #[cfg(target_os = "linux")]
        {
            let info = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
            let mut vendor = String::new();
            let mut isa_line = String::new();
            for line in info.lines() {
                if line.starts_with("vendor\t:") {
                    if let Some(val) = line.split(':').nth(1) {
                        vendor = val.trim().to_string();
                    }
                }
                if line.starts_with("isa\t:") {
                    if let Some(val) = line.split(':').nth(1) {
                        isa_line = val.trim().to_string();
                    }
                }
            }
            let mut feats = RiscVFeatures::empty();
            for token in isa_line.split('_').flat_map(|s| s.split('v')) {
                match token {
                    "i" => feats.insert(RiscVFeatures::I),
                    "m" => feats.insert(RiscVFeatures::M),
                    "a" => feats.insert(RiscVFeatures::A),
                    "f" => feats.insert(RiscVFeatures::F),
                    "d" => feats.insert(RiscVFeatures::D),
                    "c" => feats.insert(RiscVFeatures::C),
                    _ => (),
                }
            }
            (vendor, isa_line, feats)
        }
        #[cfg(not(target_os = "linux"))]
        unsafe {
            // Bare-metal: read misa CSR
            let misa: usize;
            core::arch::asm!("csrr {0}, misa", out(reg) misa);
            let mut feats = RiscVFeatures::empty();
            if misa & (1 << 0) != 0 {
                feats.insert(RiscVFeatures::I);
            }
            if misa & (1 << 1) != 0 {
                feats.insert(RiscVFeatures::M);
            }
            if misa & (1 << 2) != 0 {
                feats.insert(RiscVFeatures::A);
            }
            if misa & (1 << 3) != 0 {
                feats.insert(RiscVFeatures::F);
            }
            if misa & (1 << 4) != 0 {
                feats.insert(RiscVFeatures::D);
            }
            if misa & (1 << 8) != 0 {
                feats.insert(RiscVFeatures::C);
            }
            (String::new(), String::new(), feats)
        }
    };

    // Topology
    let cores = unsafe { sysconf(_SC_NPROCESSORS_ONLN) as u32 };
    let threads_per_core = 1;

    RiscVCpuInfo {
        vendor,
        brand,
        features,
        cores,
        threads_per_core,
    }
}

impl fmt::Display for RiscVCpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {} cores, {} threads/core, features: {:?}",
            self.vendor, self.brand, self.cores, self.threads_per_core, self.features
        )
    }
}
