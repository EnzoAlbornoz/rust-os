// Import Dependencies
use core::arch::asm;
// Define Constants
pub const CORE_ID_MASK: u64 = 0b11;
// Define Interface Functions
#[inline(always)]
pub unsafe fn park_core() {
    loop {
        wfi();
    }
}

#[inline(always)]
pub unsafe fn core_id() -> u64 {
    mpidr() & CORE_ID_MASK
}
// Define Functions
#[inline(always)]
pub unsafe fn mpidr() -> u64 {
    let mut mpidr: u64;
    asm!("mrs {mpidr}, mpidr_el1", mpidr = out(reg) mpidr);
    return mpidr;
}

#[inline(always)]
pub unsafe fn wfi() {
    asm!("wfi")
}

#[inline(always)]
pub unsafe fn wfe() {
    asm!("wfe")
}
