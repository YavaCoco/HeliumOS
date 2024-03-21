use core::arch::x86_64::__cpuid;

pub fn id() -> u16 {
  let cpuid = unsafe { __cpuid(1) };
  (cpuid.ebx >> 24) as u16
}