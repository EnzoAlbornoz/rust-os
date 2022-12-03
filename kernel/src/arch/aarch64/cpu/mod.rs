// Import dependencies
use core::arch::asm;
use super::ExceptionLevel;
use super::interrupts::VectorTable;
// Declare modules
pub mod context;
// Define constants
pub const CORE_ID_MASK: u64 = 0b11;
// Define interface functions
#[inline(always)]
pub fn park_non_main_cores() {
    unsafe {
        if core_id() != 0 {
            park_core()
        }
    }
}

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

// Define low-level functions
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
unsafe fn vbar_el1(table: &VectorTable) {
    asm!("msr VBAR_EL1, {}", in(reg) table)
}
#[inline(always)]
unsafe fn vbar_el2(table: &VectorTable) {
    asm!("msr VBAR_EL2, {}", in(reg) table)
}
#[inline(always)]
unsafe fn vbar_el3(table: &VectorTable) {
    asm!("msr VBAR_EL3, {}", in(reg) table)
}

#[inline(always)]
pub unsafe fn vbar(el: ExceptionLevel, table: &VectorTable) {
    match el {
        ExceptionLevel::El1 => vbar_el1(table),
        ExceptionLevel::El2 => vbar_el2(table),
        ExceptionLevel::El3 => vbar_el3(table),
    }
}