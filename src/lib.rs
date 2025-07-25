#![cfg_attr(not(feature = "std"), no_std)]

//! Unified cross-platform CPU feature detection library.
//!
//! This crate provides a unified API for querying CPU features across
//! x86/x86_64, ARM, and RISC-V architectures.

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
/// ARM architecture support module.
pub mod arm;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
/// RISC-V architecture support module.
pub mod riscv;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// x86/x86_64 architecture support module.
pub mod x86;

/// Enum representing supported CPU instruction sets and features across architectures.
#[derive(Debug)]
pub enum InstructionSet {
    // x86/x86_64
    FPU,
    VME,
    DE,
    PSE,
    TSC,
    MSR,
    PAE,
    MCE,
    CX8,
    APIC,
    SEP,
    MTRR,
    PGE,
    MCA,
    CMOV,
    PAT,
    PSE36,
    CLFLUSH,
    MMX,
    FXSR,
    SSE,
    SSE2,
    SSE3,
    PCLMULQDQ,
    MONITOR,
    DsCpl,
    VMX,
    SMX,
    EST,
    TM2,
    SSSE3,
    CnxtId,
    SSE41,
    SSE42,
    MOVBE,
    POPCNT,
    AES,
    XSAVE,
    OSXSAVE,
    AVX,
    F16C,
    RDRAND,
    FSGSBASE,
    BMI1,
    HLE,
    AVX2,
    SMEP,
    BMI2,
    ERMS,
    INVPCID,
    RTM,
    MPX,
    ADX,
    RDSEED,
    SHA,
    CLFLUSHOPT,
    CLWB,
    PREFETCHWT1,
    SMAP,
    AVX512F,
    AVX512DQ,
    AVX512IFMA,
    AVX512CD,
    AVX512BW,
    AVX512VL,
    AVX512VBMI,
    AVX512VBMI2,
    AVX512PKU,
    MOVDIR64B,
    MOVDIRI,
    LZCNT,
    SSE4A,
    MisalignSse,
    PREFETCHW,
    D3DNOWEXT,
    D3DNOW,
    // ARM
    NEON,
    ArmAes,
    PMULL,
    SHA1,
    SHA2,
    CRC32,
    // RISC-V
    RvI,
    RvM,
    RvA,
    RvF,
    RvD,
    RvC,
}
/// Enum representing CPU information for the current architecture.
///
/// Each variant contains architecture-specific CPU info.
#[derive(Debug, Clone)]
pub enum CpuInfo {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    /// x86/x86_64 CPU information.
    X86(x86::X86CpuInfo),
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    /// ARM CPU information.
    Arm(arm::ArmCpuInfo),
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    /// RISC-V CPU information.
    RiscV(riscv::RiscVCpuInfo),
}

impl CpuInfo {
    /// Checks if the CPU supports the given feature.
    ///
    /// # Arguments
    ///
    /// * `feature` - The instruction set feature to check.
    ///
    /// # Returns
    ///
    /// `true` if the feature is supported, `false` otherwise.
    pub fn has_feature(&self, feature: InstructionSet) -> bool {
        match self {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            CpuInfo::X86(info) => {
                use x86::X86Features;
                match feature {
                    InstructionSet::FPU => info.features.contains(X86Features::FPU),
                    InstructionSet::VME => info.features.contains(X86Features::VME),
                    InstructionSet::DE => info.features.contains(X86Features::DE),
                    InstructionSet::PSE => info.features.contains(X86Features::PSE),
                    InstructionSet::TSC => info.features.contains(X86Features::TSC),
                    InstructionSet::MSR => info.features.contains(X86Features::MSR),
                    InstructionSet::PAE => info.features.contains(X86Features::PAE),
                    InstructionSet::MCE => info.features.contains(X86Features::MCE),
                    InstructionSet::CX8 => info.features.contains(X86Features::CX8),
                    InstructionSet::APIC => info.features.contains(X86Features::APIC),
                    InstructionSet::SEP => info.features.contains(X86Features::SEP),
                    InstructionSet::MTRR => info.features.contains(X86Features::MTRR),
                    InstructionSet::PGE => info.features.contains(X86Features::PGE),
                    InstructionSet::MCA => info.features.contains(X86Features::MCA),
                    InstructionSet::CMOV => info.features.contains(X86Features::CMOV),
                    InstructionSet::PAT => info.features.contains(X86Features::PAT),
                    InstructionSet::PSE36 => info.features.contains(X86Features::PSE36),
                    InstructionSet::CLFLUSH => info.features.contains(X86Features::CLFLUSH),
                    InstructionSet::MMX => info.features.contains(X86Features::MMX),
                    InstructionSet::FXSR => info.features.contains(X86Features::FXSR),
                    InstructionSet::SSE => info.features.contains(X86Features::SSE),
                    InstructionSet::SSE2 => info.features.contains(X86Features::SSE2),
                    InstructionSet::SSE3 => info.features.contains(X86Features::SSE3),
                    InstructionSet::PCLMULQDQ => info.features.contains(X86Features::PCLMULQDQ),
                    InstructionSet::MONITOR => info.features.contains(X86Features::MONITOR),
                    InstructionSet::DsCpl => info.features.contains(X86Features::DS_CPL),
                    InstructionSet::VMX => info.features.contains(X86Features::VMX),
                    InstructionSet::SMX => info.features.contains(X86Features::SMX),
                    InstructionSet::EST => info.features.contains(X86Features::EST),
                    InstructionSet::TM2 => info.features.contains(X86Features::TM2),
                    InstructionSet::SSSE3 => info.features.contains(X86Features::SSSE3),
                    InstructionSet::CnxtId => info.features.contains(X86Features::CNXT_ID),
                    InstructionSet::SSE41 => info.features.contains(X86Features::SSE41),
                    InstructionSet::SSE42 => info.features.contains(X86Features::SSE42),
                    InstructionSet::MOVBE => info.features.contains(X86Features::MOVBE),
                    InstructionSet::POPCNT => info.features.contains(X86Features::POPCNT),
                    InstructionSet::AES => info.features.contains(X86Features::AES),
                    InstructionSet::XSAVE => info.features.contains(X86Features::XSAVE),
                    InstructionSet::OSXSAVE => info.features.contains(X86Features::OSXSAVE),
                    InstructionSet::AVX => info.features.contains(X86Features::AVX),
                    InstructionSet::F16C => info.features.contains(X86Features::F16C),
                    InstructionSet::RDRAND => info.features.contains(X86Features::RDRAND),
                    InstructionSet::FSGSBASE => info.features.contains(X86Features::FSGSBASE),
                    InstructionSet::BMI1 => info.features.contains(X86Features::BMI1),
                    InstructionSet::HLE => info.features.contains(X86Features::HLE),
                    InstructionSet::AVX2 => info.features.contains(X86Features::AVX2),
                    InstructionSet::SMEP => info.features.contains(X86Features::SMEP),
                    InstructionSet::BMI2 => info.features.contains(X86Features::BMI2),
                    InstructionSet::ERMS => info.features.contains(X86Features::ERMS),
                    InstructionSet::INVPCID => info.features.contains(X86Features::INVPCID),
                    InstructionSet::RTM => info.features.contains(X86Features::RTM),
                    InstructionSet::MPX => info.features.contains(X86Features::MPX),
                    InstructionSet::ADX => info.features.contains(X86Features::ADX),
                    InstructionSet::RDSEED => info.features.contains(X86Features::RDSEED),
                    InstructionSet::SHA => info.features.contains(X86Features::SHA),
                    InstructionSet::CLFLUSHOPT => info.features.contains(X86Features::CLFLUSHOPT),
                    InstructionSet::CLWB => info.features.contains(X86Features::CLWB),
                    InstructionSet::PREFETCHWT1 => info.features.contains(X86Features::PREFETCHWT1),
                    InstructionSet::SMAP => info.features.contains(X86Features::SMAP),
                    InstructionSet::AVX512F => info.features.contains(X86Features::AVX512F),
                    InstructionSet::AVX512DQ => info.features.contains(X86Features::AVX512DQ),
                    InstructionSet::AVX512IFMA => info.features.contains(X86Features::AVX512IFMA),
                    InstructionSet::AVX512CD => info.features.contains(X86Features::AVX512CD),
                    InstructionSet::AVX512BW => info.features.contains(X86Features::AVX512BW),
                    InstructionSet::AVX512VL => info.features.contains(X86Features::AVX512VL),
                    InstructionSet::AVX512VBMI => info.features.contains(X86Features::AVX512VBMI),
                    InstructionSet::AVX512VBMI2 => info.features.contains(X86Features::AVX512VBMI2),
                    InstructionSet::AVX512PKU => info.features.contains(X86Features::AVX512PKU),
                    InstructionSet::MOVDIR64B => info.features.contains(X86Features::MOVDIR64B),
                    InstructionSet::MOVDIRI => info.features.contains(X86Features::MOVDIRI),
                    InstructionSet::LZCNT => info.features.contains(X86Features::LZCNT),
                    InstructionSet::SSE4A => info.features.contains(X86Features::SSE4A),
                    InstructionSet::MisalignSse => {
                        info.features.contains(X86Features::MISALIGN_SSE)
                    }
                    InstructionSet::PREFETCHW => info.features.contains(X86Features::PREFETCHW),
                    InstructionSet::D3DNOWEXT => info.features.contains(X86Features::D3DNOWEXT),
                    InstructionSet::D3DNOW => info.features.contains(X86Features::D3DNOW),
                    _ => false,
                }
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            CpuInfo::Arm(info) => {
                use arm::ArmFeatures;
                match feature {
                    InstructionSet::NEON => info.features.contains(ArmFeatures::NEON),
                    InstructionSet::ArmAes => info.features.contains(ArmFeatures::AES),
                    InstructionSet::PMULL => info.features.contains(ArmFeatures::PMULL),
                    InstructionSet::SHA1 => info.features.contains(ArmFeatures::SHA1),
                    InstructionSet::SHA2 => info.features.contains(ArmFeatures::SHA2),
                    InstructionSet::CRC32 => info.features.contains(ArmFeatures::CRC32),
                    _ => false,
                }
            }
            #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
            CpuInfo::RiscV(info) => {
                use riscv::RiscVFeatures;
                match feature {
                    InstructionSet::RvI => info.features.contains(RiscVFeatures::I),
                    InstructionSet::RvM => info.features.contains(RiscVFeatures::M),
                    InstructionSet::RvA => info.features.contains(RiscVFeatures::A),
                    InstructionSet::RvF => info.features.contains(RiscVFeatures::F),
                    InstructionSet::RvD => info.features.contains(RiscVFeatures::D),
                    InstructionSet::RvC => info.features.contains(RiscVFeatures::C),
                    _ => false,
                }
            }
        }
    }
}

/// Gathers CPU information for the current architecture.
///
/// # Returns
///
/// A [`CpuInfo`] enum containing architecture-specific CPU details.
pub fn get_cpu_info() -> CpuInfo {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        CpuInfo::X86(x86::gather())
    }
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    {
        CpuInfo::Arm(arm::gather())
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    {
        CpuInfo::RiscV(riscv::gather())
    }
}
