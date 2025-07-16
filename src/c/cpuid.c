#include <stdint.h>
#if defined(_MSC_VER)
  #include <intrin.h>
#endif

void cpuid_raw(uint32_t leaf, uint32_t subleaf,
               uint32_t* eax, uint32_t* ebx,
               uint32_t* ecx, uint32_t* edx) {
#if defined(_MSC_VER)
    int regs[4];
    __cpuidex(regs, leaf, subleaf);
    *eax = regs[0]; *ebx = regs[1]; *ecx = regs[2]; *edx = regs[3];
#elif defined(__GNUC__) || defined(__clang__)
    __asm__ volatile("cpuid"
                     : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
                     : "a"(leaf), "c"(subleaf));
#else
    *eax = *ebx = *ecx = *edx = 0;
#endif
}
