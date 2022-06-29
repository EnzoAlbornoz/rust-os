// Declare Crate Attributes
#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(asm_sym)]
#![feature(ptr_to_from_bits)]
#![feature(ptr_as_uninit)]
#![feature(strict_provenance)]
#![feature(fn_align)]
// Load Panic Package
extern crate armv8a_panic_semihosting;
// Define Modules
mod arch;
mod boot;
mod cpu;
mod exception;
// Define Kernel Init
unsafe fn main() -> ! {
    loop {}
}
