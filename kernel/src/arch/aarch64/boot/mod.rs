// Import dependencies
use core::{arch::asm, slice, ptr};

use super::{cpu::park_non_main_cores, interrupts::{self, setup_interrupts}};
// Link with global labels
extern "C" {
    #[link_name = "__boot_stacks_start__"]
    static boot_stacks_start: u8;
    
    #[link_name = "__boot_stacks_end__"]
    static boot_stacks_end: u8;

    #[link_name = "__bss_start__"]
    static mut bss_start: u8;
    #[link_name = "__bss_end__"]
    static mut bss_end: u8;
}
// Define constants
const CORE_ID_MASK: u8 = 0b11;
const ASSUMED_CORES: usize = 4;
const STACK_ALIGNMENT_MASK: usize = !(0x8);

// Define very initial functions
#[export_name = "_start"]
#[naked]
unsafe extern "C" fn boot_entry() -> ! {
    // This start function only sets up the stack pointer for each core
    asm!(
        "
            // Compute Boot Stacks Size
            adr x0, {boot_stacks_start}
            adr x1, {boot_stacks_end}
            sub x0, x1, x0

            // Compute Single Stack Size
            mov x1, {core_count}
            udiv x0, x0, x1

            // Get Core ID
            mrs x1, MPIDR_EL1
            and x1, x1, {core_id_mask}

            // Compute Stack Location (STACK_PTR = [BOOT_ID * STACK_SIZE] + STACK_SIZE)
            madd x0, x0, x1, x0

            // Align Stack
            and x0, x0, {stack_alignment_mask}

            // Assign Stack Pointer
            mov sp, x0

            // Move to Rust Entrypoint
            b {rust_entrypoint}
        ",
        boot_stacks_start = sym boot_stacks_start,
        boot_stacks_end = sym boot_stacks_end,
        core_id_mask = const CORE_ID_MASK,
        core_count = const ASSUMED_CORES,
        stack_alignment_mask = const STACK_ALIGNMENT_MASK,
        rust_entrypoint = sym start,
        options(noreturn)
    )
}
// Define Rust entrypoint (stack is needed)
unsafe fn start() -> ! {
    // Keep only Core 0 for setup the system
    park_non_main_cores();
    // Initialize BSS
    clear_bss();
    // Setup Interruptions
    setup_interrupts();
    // Loop
    loop {}
}

// Define helpers
#[inline(always)]
unsafe fn clear_bss() {
    // Only clear non zero sized BSS
    if !ptr::eq(&bss_start, &bss_end) {
        // Cast BSS
        let bss = slice::from_mut_ptr_range::<'_, u8>(&mut bss_start..&mut bss_end);
        // Init BSS with zeroes
        bss.fill(0);
    }
}