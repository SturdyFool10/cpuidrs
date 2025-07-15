#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use crate::common::{CoreTypeInfo, CpuFeatures, CpuTopology};
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use libc::getauxval;

/// Detect ARM CPU features via HWCAP flags.
pub fn get_features() -> CpuFeatures {
    let mut feats = CpuFeatures::default();
    #[cfg(target_os = "linux")]
    unsafe {
        const AT_HWCAP: u64 = libc::AT_HWCAP as u64;
        let caps = getauxval(AT_HWCAP);
        const HWCAP_NEON: u64 = 1 << 12;
        feats.neon = (caps & HWCAP_NEON) != 0;
        // map more HWCAP bits to features as needed
    }
    feats
}

/// Detect core types on ARM (big.LITTLE) via cpu_capacity.
pub fn get_topology() -> CpuTopology {
    let mut is_hybrid = false;
    let mut groups = Vec::new();

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let mut map = std::collections::BTreeMap::new();
        if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu/") {
            for ent in entries.flatten() {
                let p = ent.path();
                if p.file_name().unwrap().to_string_lossy().starts_with("cpu") {
                    if let Ok(cap) = fs::read_to_string(p.join("cpu_capacity")) {
                        if let Ok(val) = cap.trim().parse::<usize>() {
                            *map.entry(val).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        if map.len() > 1 {
            is_hybrid = true;
            for (cap, cnt) in map {
                groups.push(CoreTypeInfo {
                    identifier: format!("capacity_{}", cap),
                    efficiency_class: Some(cap as u8),
                    count: cnt,
                });
            }
        }
    }
    if !is_hybrid {
        groups.push(CoreTypeInfo {
            identifier: "standard".into(),
            efficiency_class: None,
            count: num_cpus::get(),
        });
    }

    CpuTopology {
        is_hybrid,
        core_types: groups,
    }
}
