// Declare crate attributes
#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(ptr_to_from_bits)]
#![feature(ptr_as_uninit)]
#![feature(strict_provenance)]
#![feature(fn_align)]
#![feature(const_mut_refs)]
#![feature(slice_from_ptr_range)]
// Load panic package
extern crate armv8a_panic_semihosting;
// Define modules
mod arch;
mod sync;
// mod boot;
// mod cpu;
// mod exception;

// Define kernel init
unsafe fn main() -> ! {
    loop {}
}

