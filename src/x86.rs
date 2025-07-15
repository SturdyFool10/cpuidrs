use crate::common::{CoreTypeInfo, CpuFeatures, CpuTopology};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use raw_cpuid::CpuId;

/// Query CPU instruction set extensions using CPUID.
pub fn get_features() -> CpuFeatures {
    let mut feats = CpuFeatures::default();
    let cpuid = CpuId::new();
    if let Some(finfo) = cpuid.get_feature_info() {
        feats.sse = finfo.has_sse();
        feats.sse2 = finfo.has_sse2();
        feats.sse3 = finfo.has_sse3();
        feats.ssse3 = finfo.has_ssse3();
        feats.sse41 = finfo.has_sse41();
        feats.sse42 = finfo.has_sse42();
        feats.avx = finfo.has_avx();
    }
    if let Some(ef) = cpuid.get_extended_feature_info() {
        feats.avx2 = ef.has_avx2();
        feats.avx512f = ef.has_avx512f();
    }
    feats
}

/// Build topology info: detect hybrid and core groups.
pub fn get_topology() -> CpuTopology {
    let mut is_hybrid = false;
    let mut groups = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use winapi::um::sysinfoapi::GetLogicalProcessorInformationEx;
        use winapi::um::winnt::{RelationProcessorCore, SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX};
        // TODO: call GetLogicalProcessorInformationEx and parse EfficiencyClass
    }
    #[cfg(target_os = "linux")]
    {
        // Try reading /sys/devices/system/cpu/cpu*/cpu_capacity
        use std::fs;
        use std::path::Path;
        let mut map = std::collections::BTreeMap::new();
        for entry in fs::read_dir("/sys/devices/system/cpu/").unwrap_or_default() {
            let p = entry.unwrap().path();
            if p.file_name().unwrap().to_string_lossy().starts_with("cpu") {
                if let Ok(cap) = fs::read_to_string(p.join("cpu_capacity")) {
                    if let Ok(cap_val) = cap.trim().parse::<usize>() {
                        *map.entry(cap_val).or_insert(0) += 1;
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
    #[cfg(target_os = "macos")]
    {
        use libc::{CTL_HW, CTL_KERN};
        use std::mem;
        // Query hw.nperflevels
        let mut nlev: i32 = 0;
        let mut len = mem::size_of::<i32>();
        unsafe {
            let mib = [CTL_HW, libc::HW_NCPU];
            libc::sysctl(
                mib.as_ptr(),
                mib.len() as u32,
                &mut nlev as *mut _ as *mut _,
                &mut len,
                std::ptr::null(),
                0,
            );
        }
        // TODO: sysctlbyname("hw.nperflevels") then loop perflevel#
    }

    if !is_hybrid {
        // fallback: single type
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
