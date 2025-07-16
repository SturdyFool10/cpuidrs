fn main() {
    cc::Build::new()
        .file("src/c/cpuid.c")
        .include("src/c")
        .compile("cpuid_c");
}
