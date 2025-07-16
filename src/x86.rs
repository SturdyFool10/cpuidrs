//! src/x86.rs
//!
//! Inline CPUID/XGETBV detection for x86_64 using core::arch intrinsics.

use crate::common::{CoreTypeInfo, CpuFeatures, CpuTopology};
use std::collections::BTreeMap;

/// Execute CPUID with given leaf/subleaf.
#[inline(always)]
fn cpuid_count(leaf: u32, subleaf: u32) -> core::arch::x86_64::CpuidResult {
    unsafe { core::arch::x86_64::__cpuid_count(leaf, subleaf) }
}

/// Read XCR0 via XGETBV.
#[inline(always)]
fn xgetbv(idx: u32) -> u64 {
    let eax: u32;
    let edx: u32;
    unsafe {
        core::arch::asm!(
            "xgetbv",
            in("ecx") idx,
            lateout("eax") eax,
            lateout("edx") edx,
        );
    }
    ((edx as u64) << 32) | eax as u64
}

/// Gather CPU feature flags into the `CpuFeatures` struct.
pub fn get_features() -> CpuFeatures {
    let mut f = CpuFeatures::default();

    // Leaf 1: standard features
    let r1 = cpuid_count(1, 0);
    let ecx1 = r1.ecx;
    let edx1 = r1.edx;

    // EDX features (CPUID 1: EDX)
    f.set_fpu(edx1 & (1 << 0) != 0);
    f.set_vme(edx1 & (1 << 1) != 0);
    f.set_de(edx1 & (1 << 2) != 0);
    f.set_pse(edx1 & (1 << 3) != 0);
    f.set_tsc(edx1 & (1 << 4) != 0);
    f.set_msr(edx1 & (1 << 5) != 0);
    f.set_pae(edx1 & (1 << 6) != 0);
    f.set_mce(edx1 & (1 << 7) != 0);
    f.set_cx8(edx1 & (1 << 8) != 0);
    f.set_apic(edx1 & (1 << 9) != 0);
    f.set_sep(edx1 & (1 << 11) != 0);
    f.set_mtrr(edx1 & (1 << 12) != 0);
    f.set_pge(edx1 & (1 << 13) != 0);
    f.set_mca(edx1 & (1 << 14) != 0);
    f.set_cmov(edx1 & (1 << 15) != 0);
    f.set_pat(edx1 & (1 << 16) != 0);
    f.set_pse36(edx1 & (1 << 17) != 0);
    f.set_psn(edx1 & (1 << 18) != 0);
    f.set_clfsh(edx1 & (1 << 19) != 0);
    f.set_ds(edx1 & (1 << 21) != 0);
    f.set_acpi(edx1 & (1 << 22) != 0);
    f.set_mmx(edx1 & (1 << 23) != 0);
    f.set_fxsr(edx1 & (1 << 24) != 0);
    f.set_sse(edx1 & (1 << 25) != 0);
    f.set_sse2(edx1 & (1 << 26) != 0);
    f.set_ss(edx1 & (1 << 27) != 0);
    f.set_htt(edx1 & (1 << 28) != 0);
    f.set_tm(edx1 & (1 << 29) != 0);
    f.set_ia64(edx1 & (1 << 30) != 0);
    f.set_pbe(edx1 & (1 << 31) != 0);

    // ECX features (CPUID 1: ECX)
    f.set_sse3(ecx1 & (1 << 0) != 0);
    f.set_pclmulqdq(ecx1 & (1 << 1) != 0);
    f.set_dtes64(ecx1 & (1 << 2) != 0);
    f.set_monitor(ecx1 & (1 << 3) != 0);
    f.set_ds_cpl(ecx1 & (1 << 4) != 0);
    f.set_vmx(ecx1 & (1 << 5) != 0);
    f.set_smx(ecx1 & (1 << 6) != 0);
    f.set_est(ecx1 & (1 << 7) != 0);
    f.set_tm2(ecx1 & (1 << 8) != 0);
    f.set_ssse3(ecx1 & (1 << 9) != 0);
    f.set_cid(ecx1 & (1 << 10) != 0);
    f.set_fma(ecx1 & (1 << 12) != 0);
    f.set_cx16(ecx1 & (1 << 13) != 0);
    f.set_xtpr(ecx1 & (1 << 14) != 0);
    f.set_pdcm(ecx1 & (1 << 15) != 0);
    f.set_pcid(ecx1 & (1 << 17) != 0);
    f.set_dca(ecx1 & (1 << 18) != 0);
    f.set_sse41(ecx1 & (1 << 19) != 0);
    f.set_sse42(ecx1 & (1 << 20) != 0);
    f.set_x2apic(ecx1 & (1 << 21) != 0);
    f.set_movbe(ecx1 & (1 << 22) != 0);
    f.set_popcnt(ecx1 & (1 << 23) != 0);
    f.set_tsc_deadline(ecx1 & (1 << 24) != 0);
    f.set_aes(ecx1 & (1 << 25) != 0);
    f.set_xsave(ecx1 & (1 << 26) != 0);
    let osxsave = ecx1 & (1 << 27) != 0;
    f.set_osxsave(osxsave);
    let avx_cpu = ecx1 & (1 << 28) != 0;
    f.set_avx(avx_cpu);
    f.set_f16c(ecx1 & (1 << 29) != 0);
    f.set_rdrand(ecx1 & (1 << 30) != 0);

    // OS support for AVX
    if avx_cpu && osxsave {
        let xcr0 = xgetbv(0);
        if xcr0 & 0b110 == 0b110 {
            f.set_avx(true);
        }
    }

    // Leaf 7, subleaf 0: extended features
    let r7 = cpuid_count(7, 0);
    let ebx7 = r7.ebx;
    f.set_fsgsbase(ebx7 & (1 << 0) != 0);
    f.set_sgx(ebx7 & (1 << 2) != 0);
    f.set_bmi1(ebx7 & (1 << 3) != 0);
    f.set_hle(ebx7 & (1 << 4) != 0);
    f.set_avx2(ebx7 & (1 << 5) != 0);
    f.set_smep(ebx7 & (1 << 7) != 0);
    f.set_bmi2(ebx7 & (1 << 8) != 0);
    f.set_erms(ebx7 & (1 << 9) != 0);
    f.set_invpcid(ebx7 & (1 << 10) != 0);
    f.set_rtm(ebx7 & (1 << 11) != 0);
    f.set_mpx(ebx7 & (1 << 14) != 0);
    let avx512f = ebx7 & (1 << 16) != 0;
    f.set_avx512f(avx512f);
    f.set_avx512dq(ebx7 & (1 << 17) != 0);
    f.set_rdseed(ebx7 & (1 << 18) != 0);
    f.set_adx(ebx7 & (1 << 19) != 0);
    f.set_smap(ebx7 & (1 << 20) != 0);
    f.set_avx512ifma(ebx7 & (1 << 21) != 0);
    f.set_clflushopt(ebx7 & (1 << 23) != 0);
    f.set_clwb(ebx7 & (1 << 24) != 0);
    f.set_avx512pf(ebx7 & (1 << 26) != 0);
    f.set_avx512er(ebx7 & (1 << 27) != 0);
    f.set_avx512cd(ebx7 & (1 << 28) != 0);
    f.set_sha(ebx7 & (1 << 29) != 0);
    f.set_avx512bw(ebx7 & (1 << 30) != 0);
    f.set_avx512vl(ebx7 & (1 << 31) != 0);

    // Leaf 0x80000001: AMD extended
    let re = cpuid_count(0x80000001, 0);
    let ecx_ext = re.ecx;
    f.set_lahf_lm(ecx_ext & (1 << 0) != 0);
    f.set_cmp_legacy(ecx_ext & (1 << 1) != 0);
    f.set_svm(ecx_ext & (1 << 2) != 0);
    f.set_extapic(ecx_ext & (1 << 3) != 0);
    f.set_cr8_legacy(ecx_ext & (1 << 4) != 0);
    f.set_abm(ecx_ext & (1 << 5) != 0);
    f.set_sse4a(ecx_ext & (1 << 6) != 0);
    f.set_misalignsse(ecx_ext & (1 << 7) != 0);
    f.set_prefetchw(ecx_ext & (1 << 8) != 0);
    f.set_osvw(ecx_ext & (1 << 9) != 0);
    f.set_ibs(ecx_ext & (1 << 10) != 0);
    f.set_xop(ecx_ext & (1 << 11) != 0);
    f.set_skinit(ecx_ext & (1 << 12) != 0);
    f.set_wdt(ecx_ext & (1 << 13) != 0);

    // Leaf 0xC0000001: VIA PadLock
    let rc = cpuid_count(0xC0000001, 0);
    let edx_pad = rc.edx;
    f.set_padlock_rng(edx_pad & (1 << 2) != 0);
    f.set_padlock_ace(edx_pad & (1 << 6) != 0);
    f.set_padlock_ace2(edx_pad & (1 << 7) != 0);
    f.set_padlock_phe(edx_pad & (1 << 8) != 0);
    f.set_padlock_pmm(edx_pad & (1 << 9) != 0);

    f
}

/// Return (current_core, features, topology).
pub fn get_cpu_info() -> (Option<usize>, CpuFeatures, CpuTopology) {
    #[cfg(target_os = "linux")]
    let core = Some(unsafe { libc::sched_getcpu() as usize });
    #[cfg(target_os = "windows")]
    let core = Some(unsafe {
        // Use winapi instead of windows_sys for compatibility
        use winapi::um::processthreadsapi::GetCurrentProcessorNumber;
        GetCurrentProcessorNumber() as usize
    });
    #[cfg(target_os = "macos")]
    let core = None;
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    let core = None;

    (core, get_features(), get_topology())
}

/// Detect CPU topology: hybrid groups or logical count
#[cfg(target_os = "linux")]
pub fn get_topology() -> CpuTopology {
    let mut topo = CpuTopology {
        is_hybrid: false,
        core_types: Vec::new(),
    };
    let mut map: BTreeMap<u8, usize> = BTreeMap::new();
    for entry in std::fs::read_dir("/sys/devices/system/cpu/").unwrap_or_default() {
        if let Ok(p) = entry.map(|e| e.path()) {
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if name.starts_with("cpu") && name[3..].chars().all(char::is_numeric) {
                    if let Ok(cap) = std::fs::read_to_string(p.join("cpu_capacity")) {
                        if let Ok(c) = cap.trim().parse::<u8>() {
                            *map.entry(c).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    if map.len() > 1 {
        topo.is_hybrid = true;
        for (eff, cnt) in map {
            topo.core_types.push(CoreTypeInfo {
                identifier: format!("eff_{}", eff),
                efficiency_class: Some(eff),
                count: cnt,
            });
        }
    } else if let Ok(n) = std::thread::available_parallelism() {
        topo.core_types.push(CoreTypeInfo {
            identifier: "logical".into(),
            efficiency_class: None,
            count: n.get(),
        });
    }
    topo
}

#[cfg(not(target_os = "linux"))]
pub fn get_topology() -> CpuTopology {
    let mut topo = CpuTopology {
        is_hybrid: false,
        core_types: Vec::new(),
    };
    if let Ok(n) = std::thread::available_parallelism() {
        topo.core_types.push(CoreTypeInfo {
            identifier: "logical".into(),
            efficiency_class: None,
            count: n.get(),
        });
    }
    topo
}
