#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod arm;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

/// Unified CPU info enum
#[derive(Debug)]
pub enum CpuInfo {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    X86(x86::X86CpuInfo),
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    Arm(arm::ArmCpuInfo),
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    RiscV(riscv::RiscVCpuInfo),
}

/// Get CPU info for the current architecture
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
