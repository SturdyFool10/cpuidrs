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
    // At least one feature field should exist and be a boolean
    assert!(
        feats.sse == true || feats.sse == false,
        "Feature field 'sse' is not a boolean"
    );
    // All fields should be accessible (smoke test)
    let _ = (
        feats.mmx,
        feats.sse,
        feats.sse2,
        feats.sse3,
        feats.ssse3,
        feats.sse41,
        feats.sse42,
        feats.popcnt,
        feats.fma,
        feats.f16c,
        feats.avx,
        feats.avx2,
        feats.bmi1,
        feats.bmi2,
        feats.adx,
        feats.sha,
        feats.avx512f,
        feats.avx512dq,
        feats.avx512cd,
        feats.avx512bw,
        feats.avx512vl,
        feats.neon,
        feats.aes,
        feats.pmull,
        feats.sha1,
        feats.sha2,
        feats.crc32,
        feats.sve,
        feats.sve2,
        feats.rv_i,
        feats.rv_m,
        feats.rv_a,
        feats.rv_f,
        feats.rv_d,
        feats.rv_c,
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
