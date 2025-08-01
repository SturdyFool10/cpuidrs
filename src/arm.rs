#![allow(dead_code)]
use core::fmt;
#[cfg(target_os = "macos")]
use libc::{c_void, sysctlbyname};
#[cfg(target_os = "linux")]
use libc::{
    getauxval, sysconf, AT_HWCAP, AT_HWCAP2, HWCAP_AES, HWCAP_CRC32, HWCAP_NEON, HWCAP_PMULL,
    HWCAP_SHA1, HWCAP_SHA2, _SC_NPROCESSORS_ONLN,
};
#[cfg(windows)]
use winapi::um::sysinfoapi::GetNativeSystemInfo;

bitflags! {
    #[derive(Debug)]
    /// Packed feature flags for ARM/ARM64.
    /// Each flag represents a CPU feature detected at runtime.
    pub struct ArmFeatures: u64 {
        /// NEON SIMD instructions
        const NEON  = 1 << 0;
        /// AES instructions
        const AES   = 1 << 1;
        /// Polynomial multiplication (PMULL)
        const PMULL = 1 << 2;
        /// SHA1 instructions
        const SHA1  = 1 << 3;
        /// SHA2 instructions
        const SHA2  = 1 << 4;
        /// CRC32 instructions
        const CRC32 = 1 << 5;
    }
}

/// Stores information about a single logical ARM/ARM64 CPU.
/// Includes vendor, brand string, feature flags, core/thread counts.
#[derive(Debug)]
pub struct ArmCpuInfo {
    /// CPU vendor string (e.g., "ARM", "Apple")
    pub vendor: String,
    /// CPU brand string (e.g., "Apple M1", "Cortex-A72")
    pub brand: String,
    /// Feature flags detected at runtime
    pub features: ArmFeatures,
    /// Number of physical cores
    pub cores: u32,
    /// Number of threads per core (usually 1 for ARM)
    pub threads_per_core: u32,
}

/// Gathers ARM/ARM64 CPU information for the current system.
/// Detects vendor, brand, features, and topology (core/thread count).
/// Uses OS-specific APIs and `/proc/cpuinfo` where available.
pub fn gather() -> ArmCpuInfo {
    // Vendor & brand
    let (vendor, brand) = {
        #[cfg(target_os = "linux")]
        {
            let info = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
            let mut vendor = String::new();
            let mut brand = String::new();
            for line in info.lines() {
                if line.starts_with("Hardware") {
                    if let Some(val) = line.split(':').nth(1) {
                        vendor = val.trim().to_string();
                    }
                } else if line.starts_with("model name") || line.starts_with("Processor") {
                    if let Some(val) = line.split(':').nth(1) {
                        brand = val.trim().to_string();
                    }
                }
            }
            (vendor, brand)
        }
        #[cfg(target_os = "macos")]
        {
            let mut buf = [0u8; 64];
            let mut len = buf.len();
            unsafe {
                sysctlbyname(
                    b"machdep.cpu.brand_string\0".as_ptr() as *const i8,
                    buf.as_mut_ptr() as *mut c_void,
                    &mut len,
                    std::ptr::null_mut(),
                    0,
                );
            }
            let vendor = "Apple".to_string();
            let brand = String::from_utf8_lossy(&buf[..len]).trim().to_string();
            (vendor, brand)
        }
        #[cfg(windows)]
        {
            ("Unknown".to_string(), "ARM CPU".to_string())
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
        {
            ("Unknown".to_string(), "ARM CPU".to_string())
        }
    };

    // Features
    let mut f = ArmFeatures::empty();
    #[cfg(target_os = "linux")]
    unsafe {
        let caps = getauxval(AT_HWCAP) as u64;
        let caps2 = getauxval(AT_HWCAP2) as u64;
        if caps & (HWCAP_NEON as u64) != 0 {
            f.insert(ArmFeatures::NEON);
        }
        if caps & (HWCAP_AES as u64) != 0 {
            f.insert(ArmFeatures::AES);
        }
        if caps & (HWCAP_PMULL as u64) != 0 {
            f.insert(ArmFeatures::PMULL);
        }
        if caps & (HWCAP_SHA1 as u64) != 0 {
            f.insert(ArmFeatures::SHA1);
        }
        if caps & (HWCAP_SHA2 as u64) != 0 {
            f.insert(ArmFeatures::SHA2);
        }
        if caps & (HWCAP_CRC32 as u64) != 0 {
            f.insert(ArmFeatures::CRC32);
        }
    }
    #[cfg(target_os = "macos")]
    {
        // macOS feature detection omitted for brevity
    }
    #[cfg(windows)]
    unsafe {
        // Windows ARM feature checks omitted
    }

    // Topology
    let cores = unsafe {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            sysconf(_SC_NPROCESSORS_ONLN) as u32
        }
        #[cfg(windows)]
        {
            let mut info = std::mem::zeroed();
            GetNativeSystemInfo(&mut info);
            info.dwNumberOfProcessors
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
        {
            1
        }
    };
    let threads_per_core = 1;

    ArmCpuInfo {
        vendor,
        brand,
        features: f,
        cores,
        threads_per_core,
    }
}

impl fmt::Display for ArmCpuInfo {
    /// Formats the ARM CPU info for pretty-printing.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {} cores, {} threads/core, features: {:?}",
            self.vendor, self.brand, self.cores, self.threads_per_core, self.features
        )
    }
}
