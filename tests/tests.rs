use cpuidrs::{get_cpu_info, CpuInfo, InstructionSet};

#[test]
fn test_get_cpu_info_returns_valid_variant() {
    let info = get_cpu_info();
    match info {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        CpuInfo::X86(_) => {}
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        CpuInfo::Arm(_) => {}
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        CpuInfo::RiscV(_) => {}
        #[allow(unreachable_patterns)]
        _ => panic!("get_cpu_info returned an unexpected variant for this architecture"),
    }
}

#[test]
fn test_has_feature_with_known_feature() {
    let info = get_cpu_info();

    // This test checks that has_feature does not panic and returns a boolean.
    // We use a feature that is likely present on most CPUs for each arch.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let _ = info.has_feature(InstructionSet::SSE);
    }
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    {
        let _ = info.has_feature(InstructionSet::NEON);
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    {
        let _ = info.has_feature(InstructionSet::RvI);
    }
}

#[test]
fn test_has_feature_with_unknown_feature() {
    let info = get_cpu_info();

    // Use a feature from a different architecture to ensure it returns false.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        assert!(!info.has_feature(InstructionSet::NEON));
    }
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    {
        assert!(!info.has_feature(InstructionSet::SSE));
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    {
        assert!(!info.has_feature(InstructionSet::SSE));
    }
}
