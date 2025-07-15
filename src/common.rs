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
pub struct CpuInfo {
    /// Architecture string, e.g. "x86_64", "aarch64", "riscv64", or "bare-metal"
    pub architecture: &'static str,
    /// Feature flags
    pub features: CpuFeatures,
    /// Core topology and hybrid grouping info
    pub topology: CpuTopology,
}

/// Flags for instruction set extensions across all supported architectures.
#[derive(Debug, Default)]
pub struct CpuFeatures {
    // x86/x86_64
    pub mmx: bool,
    pub sse: bool,
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse41: bool,
    pub sse42: bool,
    pub popcnt: bool,
    pub fma: bool,
    pub f16c: bool,
    pub avx: bool,
    pub avx2: bool,
    pub bmi1: bool,
    pub bmi2: bool,
    pub adx: bool,
    pub sha: bool,
    pub avx512f: bool,
    pub avx512dq: bool,
    pub avx512cd: bool,
    pub avx512bw: bool,
    pub avx512vl: bool,
    // ARM/AArch64 (HWCAP)
    pub neon: bool,
    pub aes: bool,
    pub pmull: bool,
    pub sha1: bool,
    pub sha2: bool,
    pub crc32: bool,
    pub sve: bool,
    pub sve2: bool,
    // RISC-V (misa CSR)
    pub rv_i: bool,
    pub rv_m: bool,
    pub rv_a: bool,
    pub rv_f: bool,
    pub rv_d: bool,
    pub rv_c: bool,
}

/// Grouping info for each core type in a hybrid system.
#[derive(Debug)]
pub struct CoreTypeInfo {
    /// Identifier (e.g. "Performance", "Efficiency", or capacity string)
    pub identifier: &'static str,
    /// Optional numeric efficiency class (Windows)
    pub efficiency_class: Option<u8>,
    /// Number of logical cores of this type
    pub count: usize,
}

/// Topology, including hybrid detection flag and per-type counts.
#[derive(Debug)]
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
