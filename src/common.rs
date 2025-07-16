extern crate alloc;

#[cfg(all(
    not(feature = "baremetal"),
    any(target_arch = "arm", target_arch = "aarch64")
))]
use crate::arm::{get_features as arm_features, get_topology as arm_topology};
#[cfg(feature = "baremetal")]
use crate::bare::{get_features as bare_features, get_topology as bare_topology};
#[cfg(all(
    not(feature = "baremetal"),
    any(target_arch = "riscv32", target_arch = "riscv64")
))]
use crate::riscv::{get_features as riscv_features, get_topology as riscv_topology};
#[cfg(all(
    not(feature = "baremetal"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
use crate::x86::{get_features as x86_features, get_topology as x86_topology};

/// CPU information: architecture, supported features, and topology.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CpuInfo {
    /// Architecture string, e.g. "x86_64", "aarch64", "riscv64", or "bare-metal"
    pub architecture: &'static str,
    /// Feature flags
    pub features: CpuFeatures,
    /// Core topology and hybrid grouping info
    pub topology: CpuTopology,
}

/// Flags for instruction set extensions across all supported architectures.
/// Minimal bitfield version for the first 8 flags (expand as needed).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CpuFeatures {
    bits: [u8; 16],
}

macro_rules! bitfield_flag {
    ($get:ident, $set:ident, $byte:expr, $bit:expr) => {
        pub fn $get(&self) -> bool {
            (self.bits[$byte] & ((1u32 << $bit) as u8)) != 0
        }
        pub fn $set(&mut self, val: bool) {
            if val {
                self.bits[$byte] |= (1u32 << $bit) as u8;
            } else {
                self.bits[$byte] &= !((1u32 << $bit) as u8);
            }
        }
    };
}

impl CpuFeatures {
    pub fn new() -> Self {
        Self { bits: [0u8; 16] }
    }
    // EDX (CPUID 1: EDX)
    bitfield_flag!(fpu, set_fpu, 0, 0);
    bitfield_flag!(vme, set_vme, 0, 1);
    bitfield_flag!(de, set_de, 0, 2);
    bitfield_flag!(pse, set_pse, 0, 3);
    bitfield_flag!(tsc, set_tsc, 0, 4);
    bitfield_flag!(msr, set_msr, 0, 5);
    bitfield_flag!(pae, set_pae, 0, 6);
    bitfield_flag!(mce, set_mce, 0, 7);
    bitfield_flag!(cx8, set_cx8, 0, 8);
    bitfield_flag!(apic, set_apic, 0, 9);
    bitfield_flag!(sep, set_sep, 0, 11);
    bitfield_flag!(mtrr, set_mtrr, 0, 12);
    bitfield_flag!(pge, set_pge, 0, 13);
    bitfield_flag!(mca, set_mca, 0, 14);
    bitfield_flag!(cmov, set_cmov, 0, 15);
    bitfield_flag!(pat, set_pat, 0, 16);
    bitfield_flag!(pse36, set_pse36, 0, 17);
    bitfield_flag!(psn, set_psn, 0, 18);
    bitfield_flag!(clfsh, set_clfsh, 0, 19);
    bitfield_flag!(ds, set_ds, 0, 21);
    bitfield_flag!(acpi, set_acpi, 0, 22);
    bitfield_flag!(mmx, set_mmx, 0, 23);
    bitfield_flag!(fxsr, set_fxsr, 0, 24);
    bitfield_flag!(sse, set_sse, 0, 25);
    bitfield_flag!(sse2, set_sse2, 0, 26);
    bitfield_flag!(ss, set_ss, 0, 27);
    bitfield_flag!(htt, set_htt, 0, 28);
    bitfield_flag!(tm, set_tm, 0, 29);
    bitfield_flag!(ia64, set_ia64, 0, 30);
    bitfield_flag!(pbe, set_pbe, 0, 31);

    // ECX (CPUID 1: ECX)
    bitfield_flag!(sse3, set_sse3, 1, 0);
    bitfield_flag!(pclmulqdq, set_pclmulqdq, 1, 1);
    bitfield_flag!(dtes64, set_dtes64, 1, 2);
    bitfield_flag!(monitor, set_monitor, 1, 3);
    bitfield_flag!(ds_cpl, set_ds_cpl, 1, 4);
    bitfield_flag!(vmx, set_vmx, 1, 5);
    bitfield_flag!(smx, set_smx, 1, 6);
    bitfield_flag!(est, set_est, 1, 7);
    bitfield_flag!(tm2, set_tm2, 1, 8);
    bitfield_flag!(ssse3, set_ssse3, 1, 9);
    bitfield_flag!(cid, set_cid, 1, 10);
    bitfield_flag!(fma, set_fma, 1, 12);
    bitfield_flag!(cx16, set_cx16, 1, 13);
    bitfield_flag!(xtpr, set_xtpr, 1, 14);
    bitfield_flag!(pdcm, set_pdcm, 1, 15);
    bitfield_flag!(pcid, set_pcid, 1, 17);
    bitfield_flag!(dca, set_dca, 1, 18);
    bitfield_flag!(sse41, set_sse41, 1, 19);
    bitfield_flag!(sse42, set_sse42, 1, 20);
    bitfield_flag!(x2apic, set_x2apic, 1, 21);
    bitfield_flag!(movbe, set_movbe, 1, 22);
    bitfield_flag!(popcnt, set_popcnt, 1, 23);
    bitfield_flag!(tsc_deadline, set_tsc_deadline, 1, 24);
    bitfield_flag!(aes, set_aes, 1, 25);
    bitfield_flag!(xsave, set_xsave, 1, 26);
    bitfield_flag!(osxsave, set_osxsave, 1, 27);
    bitfield_flag!(avx, set_avx, 1, 28);
    bitfield_flag!(f16c, set_f16c, 1, 29);
    bitfield_flag!(rdrand, set_rdrand, 1, 30);

    // CPUID 7, EBX
    bitfield_flag!(fsgsbase, set_fsgsbase, 2, 0);
    bitfield_flag!(sgx, set_sgx, 2, 2);
    bitfield_flag!(bmi1, set_bmi1, 2, 3);
    bitfield_flag!(hle, set_hle, 2, 4);
    bitfield_flag!(avx2, set_avx2, 2, 5);
    bitfield_flag!(smep, set_smep, 2, 7);
    bitfield_flag!(bmi2, set_bmi2, 2, 8);
    bitfield_flag!(erms, set_erms, 2, 9);
    bitfield_flag!(invpcid, set_invpcid, 2, 10);
    bitfield_flag!(rtm, set_rtm, 2, 11);
    bitfield_flag!(mpx, set_mpx, 2, 14);
    bitfield_flag!(avx512f, set_avx512f, 2, 16);
    bitfield_flag!(avx512dq, set_avx512dq, 2, 17);
    bitfield_flag!(rdseed, set_rdseed, 2, 18);
    bitfield_flag!(adx, set_adx, 2, 19);
    bitfield_flag!(smap, set_smap, 2, 20);
    bitfield_flag!(avx512ifma, set_avx512ifma, 2, 21);
    bitfield_flag!(clflushopt, set_clflushopt, 2, 23);
    bitfield_flag!(clwb, set_clwb, 2, 24);
    bitfield_flag!(avx512pf, set_avx512pf, 2, 26);
    bitfield_flag!(avx512er, set_avx512er, 2, 27);
    bitfield_flag!(avx512cd, set_avx512cd, 2, 28);
    bitfield_flag!(sha, set_sha, 2, 29);
    bitfield_flag!(avx512bw, set_avx512bw, 2, 30);
    bitfield_flag!(avx512vl, set_avx512vl, 2, 31);

    // CPUID 0x80000001, ECX (AMD extended)
    bitfield_flag!(lahf_lm, set_lahf_lm, 3, 0);
    bitfield_flag!(cmp_legacy, set_cmp_legacy, 3, 1);
    bitfield_flag!(svm, set_svm, 3, 2);
    bitfield_flag!(extapic, set_extapic, 3, 3);
    bitfield_flag!(cr8_legacy, set_cr8_legacy, 3, 4);
    bitfield_flag!(abm, set_abm, 3, 5);
    bitfield_flag!(sse4a, set_sse4a, 3, 6);
    bitfield_flag!(misalignsse, set_misalignsse, 3, 7);
    bitfield_flag!(prefetchw, set_prefetchw, 3, 8);
    bitfield_flag!(osvw, set_osvw, 3, 9);
    bitfield_flag!(ibs, set_ibs, 3, 10);
    bitfield_flag!(xop, set_xop, 3, 11);
    bitfield_flag!(skinit, set_skinit, 3, 12);
    bitfield_flag!(wdt, set_wdt, 3, 13);

    // CPUID 0xC0000001, EDX (VIA PadLock)
    bitfield_flag!(padlock_rng, set_padlock_rng, 4, 2);
    bitfield_flag!(padlock_ace, set_padlock_ace, 4, 6);
    bitfield_flag!(padlock_ace2, set_padlock_ace2, 4, 7);
    bitfield_flag!(padlock_phe, set_padlock_phe, 4, 8);
    bitfield_flag!(padlock_pmm, set_padlock_pmm, 4, 9);
}

impl core::fmt::Debug for CpuFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("CpuFeatures");
        // EDX (CPUID 1: EDX)
        ds.field("fpu", &self.fpu());
        ds.field("vme", &self.vme());
        ds.field("de", &self.de());
        ds.field("pse", &self.pse());
        ds.field("tsc", &self.tsc());
        ds.field("msr", &self.msr());
        ds.field("pae", &self.pae());
        ds.field("mce", &self.mce());
        ds.field("cx8", &self.cx8());
        ds.field("apic", &self.apic());
        ds.field("sep", &self.sep());
        ds.field("mtrr", &self.mtrr());
        ds.field("pge", &self.pge());
        ds.field("mca", &self.mca());
        ds.field("cmov", &self.cmov());
        ds.field("pat", &self.pat());
        ds.field("pse36", &self.pse36());
        ds.field("psn", &self.psn());
        ds.field("clfsh", &self.clfsh());
        ds.field("ds", &self.ds());
        ds.field("acpi", &self.acpi());
        ds.field("mmx", &self.mmx());
        ds.field("fxsr", &self.fxsr());
        ds.field("sse", &self.sse());
        ds.field("sse2", &self.sse2());
        ds.field("ss", &self.ss());
        ds.field("htt", &self.htt());
        ds.field("tm", &self.tm());
        ds.field("ia64", &self.ia64());
        ds.field("pbe", &self.pbe());

        // ECX (CPUID 1: ECX)
        ds.field("sse3", &self.sse3());
        ds.field("pclmulqdq", &self.pclmulqdq());
        ds.field("dtes64", &self.dtes64());
        ds.field("monitor", &self.monitor());
        ds.field("ds_cpl", &self.ds_cpl());
        ds.field("vmx", &self.vmx());
        ds.field("smx", &self.smx());
        ds.field("est", &self.est());
        ds.field("tm2", &self.tm2());
        ds.field("ssse3", &self.ssse3());
        ds.field("cid", &self.cid());
        ds.field("fma", &self.fma());
        ds.field("cx16", &self.cx16());
        ds.field("xtpr", &self.xtpr());
        ds.field("pdcm", &self.pdcm());
        ds.field("pcid", &self.pcid());
        ds.field("dca", &self.dca());
        ds.field("sse41", &self.sse41());
        ds.field("sse42", &self.sse42());
        ds.field("x2apic", &self.x2apic());
        ds.field("movbe", &self.movbe());
        ds.field("popcnt", &self.popcnt());
        ds.field("tsc_deadline", &self.tsc_deadline());
        ds.field("aes", &self.aes());
        ds.field("xsave", &self.xsave());
        ds.field("osxsave", &self.osxsave());
        ds.field("avx", &self.avx());
        ds.field("f16c", &self.f16c());
        ds.field("rdrand", &self.rdrand());

        // CPUID 7, EBX
        ds.field("fsgsbase", &self.fsgsbase());
        ds.field("sgx", &self.sgx());
        ds.field("bmi1", &self.bmi1());
        ds.field("hle", &self.hle());
        ds.field("avx2", &self.avx2());
        ds.field("smep", &self.smep());
        ds.field("bmi2", &self.bmi2());
        ds.field("erms", &self.erms());
        ds.field("invpcid", &self.invpcid());
        ds.field("rtm", &self.rtm());
        ds.field("mpx", &self.mpx());
        ds.field("avx512f", &self.avx512f());
        ds.field("avx512dq", &self.avx512dq());
        ds.field("rdseed", &self.rdseed());
        ds.field("adx", &self.adx());
        ds.field("smap", &self.smap());
        ds.field("avx512ifma", &self.avx512ifma());
        ds.field("clflushopt", &self.clflushopt());
        ds.field("clwb", &self.clwb());
        ds.field("avx512pf", &self.avx512pf());
        ds.field("avx512er", &self.avx512er());
        ds.field("avx512cd", &self.avx512cd());
        ds.field("sha", &self.sha());
        ds.field("avx512bw", &self.avx512bw());
        ds.field("avx512vl", &self.avx512vl());

        // CPUID 0x80000001, ECX (AMD extended)
        ds.field("lahf_lm", &self.lahf_lm());
        ds.field("cmp_legacy", &self.cmp_legacy());
        ds.field("svm", &self.svm());
        ds.field("extapic", &self.extapic());
        ds.field("cr8_legacy", &self.cr8_legacy());
        ds.field("abm", &self.abm());
        ds.field("sse4a", &self.sse4a());
        ds.field("misalignsse", &self.misalignsse());
        ds.field("prefetchw", &self.prefetchw());
        ds.field("osvw", &self.osvw());
        ds.field("ibs", &self.ibs());
        ds.field("xop", &self.xop());
        ds.field("skinit", &self.skinit());
        ds.field("wdt", &self.wdt());

        // CPUID 0xC0000001, EDX (VIA PadLock)
        ds.field("padlock_rng", &self.padlock_rng());
        ds.field("padlock_ace", &self.padlock_ace());
        ds.field("padlock_ace2", &self.padlock_ace2());
        ds.field("padlock_phe", &self.padlock_phe());
        ds.field("padlock_pmm", &self.padlock_pmm());

        ds.finish()
    }
}

impl Default for CpuFeatures {
    fn default() -> Self {
        Self::new()
    }
}

/// Grouping info for each core type in a hybrid system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoreTypeInfo {
    /// Identifier (e.g. "Performance", "Efficiency", or capacity string)
    pub identifier: &'static str,
    /// Optional numeric efficiency class (Windows)
    pub efficiency_class: Option<u8>,
    /// Number of logical cores of this type
    pub count: usize,
}

/// Topology, including hybrid detection flag and per-type counts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CpuTopology {
    /// True if multiple core types detected
    pub is_hybrid: bool,
    /// Info for each detected core type
    pub core_types: alloc::vec::Vec<CoreTypeInfo>,
}

impl CpuInfo {
    /// Returns the core type info for the current hardware thread.
    pub fn current_core_type(&self) -> Option<&CoreTypeInfo> {
        current_core_type(&self.topology)
    }
}

/// Dispatch detection to the appropriate module.
pub fn detect() -> CpuInfo {
    #[cfg(feature = "baremetal")]
    {
        let f = bare_features();
        let t = bare_topology();
        CpuInfo {
            architecture: "bare-metal",
            features: f,
            topology: t,
        }
    }
    #[cfg(all(
        not(feature = "baremetal"),
        any(target_arch = "x86", target_arch = "x86_64")
    ))]
    {
        let f = x86_features();
        let t = x86_topology();
        CpuInfo {
            architecture: std::env::consts::ARCH,
            features: f,
            topology: t,
        }
    }
    #[cfg(all(
        not(feature = "baremetal"),
        any(target_arch = "arm", target_arch = "aarch64")
    ))]
    {
        let f = arm_features();
        let t = arm_topology();
        CpuInfo {
            architecture: std::env::consts::ARCH,
            features: f,
            topology: t,
        }
    }
    #[cfg(all(
        not(feature = "baremetal"),
        any(target_arch = "riscv32", target_arch = "riscv64")
    ))]
    {
        let f = riscv_features();
        let t = riscv_topology();
        CpuInfo {
            architecture: std::env::consts::ARCH,
            features: f,
            topology: t,
        }
    }
    #[cfg(not(any(
        feature = "baremetal",
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64"
    )))]
    compile_error!("Unsupported architecture or environment for cpuid crate");
}

/// Map current CPU index to its core type group (Linux/Windows). For bare-metal or unsupported, returns first type.
fn current_core_type(topology: &CpuTopology) -> Option<&CoreTypeInfo> {
    topology.core_types.get(0)
}

/// Returns number of logical CPUs on hosted platforms.
#[cfg(not(feature = "baremetal"))]
pub fn get_num_cpus() -> usize {
    #[cfg(unix)]
    unsafe {
        libc::sysconf(libc::_SC_NPROCESSORS_ONLN).max(1) as usize
    }
    #[cfg(windows)]
    unsafe {
        use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
        let mut sysinfo: SYSTEM_INFO = std::mem::zeroed();
        GetSystemInfo(&mut sysinfo);
        sysinfo.dwNumberOfProcessors as usize
    }
}
