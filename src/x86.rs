// src/x86.rs
#![allow(dead_code)]
use bitflags::bitflags;
use core::fmt;

// FFI binding to our C shim
#[link(name = "cpuid_c")]
extern "C" {
    fn cpuid_raw(
        leaf: u32,
        subleaf: u32,
        eax: *mut u32,
        ebx: *mut u32,
        ecx: *mut u32,
        edx: *mut u32,
    );
}

/// Execute CPUID instruction
unsafe fn cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    cpuid_raw(leaf, subleaf, &mut a, &mut b, &mut c, &mut d);
    (a, b, c, d)
}

bitflags! {
    #[derive(Debug)]
    /// Packed feature flags for x86/x86_64 as a u128
    pub struct X86Features: u128 {
        // CPUID(1).EDX
        const FPU           = 1 << 0;
        const VME           = 1 << 1;
        const DE            = 1 << 2;
        const PSE           = 1 << 3;
        const TSC           = 1 << 4;
        const MSR           = 1 << 5;
        const PAE           = 1 << 6;
        const MCE           = 1 << 7;
        const CX8           = 1 << 8;
        const APIC          = 1 << 9;
        const SEP           = 1 << 10;
        const MTRR          = 1 << 11;
        const PGE           = 1 << 12;
        const MCA           = 1 << 13;
        const CMOV          = 1 << 14;
        const PAT           = 1 << 15;
        const PSE36         = 1 << 16;
        const CLFLUSH       = 1 << 17;
        const MMX           = 1 << 18;
        const FXSR          = 1 << 19;
        const SSE           = 1 << 20;
        const SSE2          = 1 << 21;

        // CPUID(1).ECX
        const SSE3          = 1 << 22;
        const PCLMULQDQ     = 1 << 23;
        const MONITOR       = 1 << 24;
        const DS_CPL        = 1 << 25;
        const VMX           = 1 << 26;
        const SMX           = 1 << 27;
        const EST           = 1 << 28;
        const TM2           = 1 << 29;
        const SSSE3         = 1 << 30;
        const CNXT_ID       = 1 << 31;
        const SSE41         = 1 << 32;
        const SSE42         = 1 << 33;
        const MOVBE         = 1 << 34;
        const POPCNT        = 1 << 35;
        const AES           = 1 << 36;
        const XSAVE         = 1 << 37;
        const OSXSAVE       = 1 << 38;
        const AVX           = 1 << 39;
        const F16C          = 1 << 40;
        const RDRAND        = 1 << 41;

        // CPUID(7,0).EBX
        const FSGSBASE      = 1 << 42;
        const BMI1          = 1 << 43;
        const HLE           = 1 << 44;
        const AVX2          = 1 << 45;
        const SMEP          = 1 << 46;
        const BMI2          = 1 << 47;
        const ERMS          = 1 << 48;
        const INVPCID       = 1 << 49;
        const RTM           = 1 << 50;
        const MPX           = 1 << 51;
        const ADX           = 1 << 52;
        const RDSEED        = 1 << 53;
        const SHA           = 1 << 54;
        const CLFLUSHOPT    = 1 << 55;
        const CLWB          = 1 << 56;
        const PREFETCHWT1   = 1 << 57;
        const SMAP          = 1 << 58;

        // CPUID(7,0).ECX
        const AVX512F       = 1 << 59;
        const AVX512DQ      = 1 << 60;
        const AVX512IFMA    = 1 << 61;
        const AVX512CD      = 1 << 62;
        const AVX512BW      = 1 << 63;
        const AVX512VL      = 1 << 64;
        const AVX512VBMI    = 1 << 65;
        const AVX512VBMI2   = 1 << 66;
        const AVX512PKU     = 1 << 67;
        const MOVDIR64B     = 1 << 68;
        const MOVDIRI       = 1 << 69;

        // AMD extended CPUID (0x80000001)
        const LZCNT         = 1 << 70;
        const SSE4A         = 1 << 71;
        const MISALIGN_SSE  = 1 << 72;
        const PREFETCHW     = 1 << 73;
        const D3DNOWEXT     = 1 << 74;
        const D3DNOW        = 1 << 75;
    }
}

/// Macro to compact repetitive flag checks
macro_rules! cpuid_flags {
    ($flags:ident, $reg:ident, $($bit:expr => $flag:ident),+ $(,)?) => {
        $(
            if $reg & (1u32 << $bit) != 0 {
                $flags.insert(X86Features::$flag);
            }
        )+
    };
}

/// Hybrid core type
#[derive(Debug)]
pub enum CoreType {
    Performance,
    Efficiency,
}

/// x86 CPU information container
#[derive(Debug)]
pub struct X86CpuInfo {
    pub vendor: String,
    pub brand: String,
    pub features: X86Features,
    pub cores: u32,
    pub threads_per_core: u32,
    pub hybrid: bool,
    pub core_type: Option<CoreType>,
}

/// Gather all x86 info at runtime
pub fn gather() -> X86CpuInfo {
    unsafe {
        // Vendor
        let (_m0, ebx, ecx, edx) = cpuid(0, 0);
        let vendor = String::from_utf8_lossy(
            &[ebx.to_le_bytes(), edx.to_le_bytes(), ecx.to_le_bytes()].concat(),
        )
        .trim()
        .to_string();

        // Brand string
        let (max_ext, _, _, _) = cpuid(0x8000_0000, 0);
        let mut brand = String::new();
        if max_ext >= 0x8000_0004 {
            for leaf in 0x8000_0002..=0x8000_0004 {
                let (a, b, c, d) = cpuid(leaf, 0);
                for &reg in &[a, b, c, d] {
                    brand.push_str(&String::from_utf8_lossy(&reg.to_le_bytes()));
                }
            }
            brand = brand.trim_end_matches('\0').trim().to_string();
        }

        // Features
        let mut f = X86Features::empty();
        let (_e1, _, ec1, ed1) = cpuid(1, 0);
        cpuid_flags!(f, ed1,
            0 => FPU, 1 => VME, 2 => DE, 3 => PSE, 4 => TSC, 5 => MSR,
            6 => PAE, 7 => MCE, 8 => CX8, 9 => APIC, 10 => SEP, 11 => MTRR,
            12 => PGE, 13 => MCA, 14 => CMOV, 15 => PAT, 16 => PSE36,
            17 => CLFLUSH, 18 => MMX, 19 => FXSR, 20 => SSE, 21 => SSE2,
        );
        cpuid_flags!(f, ec1,
            0 => SSE3, 1 => PCLMULQDQ, 2 => DS_CPL, 3 => MONITOR, 5 => VMX,
            6 => SMX, 7 => EST, 8 => TM2, 9 => SSSE3, 10 => CNXT_ID,
            19 => SSE41, 20 => SSE42, 22 => MOVBE, 23 => POPCNT, 25 => AES,
            26 => XSAVE, 27 => OSXSAVE, 28 => AVX, 29 => F16C, 30 => RDRAND,
        );
        let (_e7, eb7, ec7, _) = cpuid(7, 0);
        cpuid_flags!(f, eb7,
            0 => FSGSBASE, 3 => BMI1, 4 => HLE, 5 => AVX2, 7 => SMEP,
            8 => BMI2, 9 => ERMS, 10 => INVPCID, 11 => RTM, 14 => MPX,
            18 => RDSEED, 19 => ADX, 21 => SHA, 23 => CLFLUSHOPT, 24 => CLWB,
            20 => SMAP, 16 => AVX512F, 17 => AVX512DQ, 21 => AVX512IFMA,
            28 => AVX512CD, 30 => AVX512BW, 31 => AVX512VL,
        );
        cpuid_flags!(f, ec7,
            1 => AVX512VBMI, 3 => AVX512PKU, 6 => AVX512VBMI2,
            7 => MOVDIR64B, 8 => MOVDIRI,
        );
        let (max_amd, _, _, _) = cpuid(0x8000_0000, 0);
        if max_amd >= 0x8000_0001 {
            let (_ea, _, ec2, ed2) = cpuid(0x8000_0001, 0);
            cpuid_flags!(f, ec2, 5 => LZCNT, 6 => SSE4A, 7 => MISALIGN_SSE);
            cpuid_flags!(f, ed2, 8 => PREFETCHW, 30 => D3DNOWEXT, 31 => D3DNOW);
        }
        let (max_l, _, _, _) = cpuid(0, 0);
        let (tpc, tpp) = if max_l >= 11 {
            let (_, eb0, _, _) = cpuid(11, 0);
            let (_, eb1, _, _) = cpuid(11, 1);
            (eb0, eb1)
        } else {
            let (_, eb, _, _) = cpuid(1, 0);
            (1, (eb >> 16) & 0xff)
        };
        let cores = if tpc > 0 { tpp / tpc } else { 1 };
        let (_, _, _, ed7b) = cpuid(7, 0);
        let hybrid = (ed7b & (1 << 15)) != 0;
        let core_type = if hybrid {
            let (e1a, _, _, _) = cpuid(0x1a, 0);
            let ct = ((e1a >> 24) & 0xff) as u8;
            Some(if ct == 0x20 {
                CoreType::Efficiency
            } else {
                CoreType::Performance
            })
        } else {
            None
        };

        X86CpuInfo {
            vendor,
            brand,
            features: f,
            cores,
            threads_per_core: tpc,
            hybrid,
            core_type,
        }
    }
}

// Display implementation
impl fmt::Display for X86CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {} cores, {} threads/core, features: {:?}",
            self.vendor, self.brand, self.cores, self.threads_per_core, self.features
        )
    }
}
