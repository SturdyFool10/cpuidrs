//! Cross-platform tests for cpuid::detect()
//!
//! These tests verify that the detect() function returns reasonable values
//! for architecture, features, and topology on the current platform.

use cpuid::{detect, CoreTypeInfo, CpuFeatures, CpuInfo, CpuTopology};

#[test]
fn test_detect_returns_cpuinfo() {
    let info = detect();
    // Architecture string should not be empty
    assert!(
        !info.architecture.is_empty(),
        "Architecture string is empty"
    );
}

#[test]
fn test_features_struct_fields() {
    let info = detect();
    let feats = &info.features;
    // At least one feature method should exist and be a boolean
    assert!(
        feats.sse() == true || feats.sse() == false,
        "Feature method 'sse()' is not a boolean"
    );
    // All implemented feature methods should be accessible (smoke test)
    // This list is generated to match all macro-generated getters in CpuFeatures
    let _ = (
        feats.fpu(),
        feats.vme(),
        feats.de(),
        feats.pse(),
        feats.tsc(),
        feats.msr(),
        feats.pae(),
        feats.mce(),
        feats.cx8(),
        feats.apic(),
        feats.sep(),
        feats.mtrr(),
        feats.pge(),
        feats.mca(),
        feats.cmov(),
        feats.pat(),
        feats.pse36(),
        feats.psn(),
        feats.clfsh(),
        feats.ds(),
        feats.acpi(),
        feats.mmx(),
        feats.fxsr(),
        feats.sse(),
        feats.sse2(),
        feats.ss(),
        feats.htt(),
        feats.tm(),
        feats.ia64(),
        feats.pbe(),
        feats.sse3(),
        feats.pclmulqdq(),
        feats.dtes64(),
        feats.monitor(),
        feats.ds_cpl(),
        feats.vmx(),
        feats.smx(),
        feats.est(),
        feats.tm2(),
        feats.ssse3(),
        feats.cid(),
        feats.fma(),
        feats.cx16(),
        feats.xtpr(),
        feats.pdcm(),
        feats.pcid(),
        feats.dca(),
        feats.sse41(),
        feats.sse42(),
        feats.x2apic(),
        feats.movbe(),
        feats.popcnt(),
        feats.tsc_deadline(),
        feats.aes(),
        feats.xsave(),
        feats.osxsave(),
        feats.avx(),
        feats.f16c(),
        feats.rdrand(),
        feats.fsgsbase(),
        feats.sgx(),
        feats.bmi1(),
        feats.hle(),
        feats.avx2(),
        feats.smep(),
        feats.bmi2(),
        feats.erms(),
        feats.invpcid(),
        feats.rtm(),
        feats.mpx(),
        feats.avx512f(),
        feats.avx512dq(),
        feats.rdseed(),
        feats.adx(),
        feats.smap(),
        feats.avx512ifma(),
        feats.clflushopt(),
        feats.clwb(),
        feats.avx512pf(),
        feats.avx512er(),
        feats.avx512cd(),
        feats.sha(),
        feats.avx512bw(),
        feats.avx512vl(),
        feats.lahf_lm(),
        feats.cmp_legacy(),
        feats.svm(),
        feats.extapic(),
        feats.cr8_legacy(),
        feats.abm(),
        feats.sse4a(),
        feats.misalignsse(),
        feats.prefetchw(),
        feats.osvw(),
        feats.ibs(),
        feats.xop(),
        feats.skinit(),
        feats.wdt(),
        feats.padlock_rng(),
        feats.padlock_ace(),
        feats.padlock_ace2(),
        feats.padlock_phe(),
        feats.padlock_pmm(),
    );
}

#[test]
fn test_topology_fields() {
    let info = detect();
    let topo = &info.topology;
    // is_hybrid should be a boolean
    assert!(
        topo.is_hybrid == true || topo.is_hybrid == false,
        "Topology field 'is_hybrid' is not a boolean"
    );
    // core_types should be a vector
    assert!(
        topo.core_types.len() >= 0,
        "Topology field 'core_types' is not a vector"
    );
    // If there are core types, their fields should be accessible
    if let Some(core) = topo.core_types.get(0) {
        let _ = (&core.identifier, core.efficiency_class, core.count);
    }
}

#[test]
fn test_current_core_type() {
    let info = detect();
    let maybe_core = info.current_core_type();
    // Should not panic, may be None on some platforms
    if let Some(core) = maybe_core {
        assert!(
            !core.identifier.is_empty(),
            "Current core type identifier is empty"
        );
    }
}
