# CPUIDrs
---
your single stop shop for all CPUID related information

## Features
- CPUID instruction decoding
- Simple API for querying CPUID information
- Support for multiple CPU architectures
- Easy to use and integrate into your projects

## Installation
You can use CPUIDrs by adding it to your project dependencies. For example, if you are using Cargo, add the following to your `Cargo.toml`:

```toml
[dependencies]
cpuidrs = "0.1.0"
```
## Usage
Here's a simple example of how to use CPUIDrs in your Rust project:
```rust
use cpuidrs::CpuInfo;
fn main() {
    let cpuinf: CpuInfo = cpuidrs::get_cpu_info(); // This gets information about the current core and some minor info about the whole CPU such as whether you're on a hybrid cpu or not.
    if cpuinf.has_feature(cpuidrs::InstructionSet::AVX2) { //note that we use this enum, this is a list of all the instruction sets that CPUIDrs supports and the method we are calling will return true if the instruction set is supported by the current core of the CPU.
        println!("This Core supports AVX2!");
    } else {
        println!("This Core does not support AVX2.");
    }
}
```

## Purpose
This library is designed to allow developers to know whether instruction extensions are supported BEFORE running them so you can get the speed up of using them without causing incompatibility problems with CPUs that do not support them. This approach also would allow a developer to create a message saying the program is not supported on this CPU if it does not support the required instruction set.


## Legal
This project is licensed under both the Apache 2.0 and MIT licenses. You can choose which license to use for your project. See the licenses in ./licenses/ for more details.
