// Import Dependencies
use crate::cpu::park_non_main_cores;
use core::arch::asm;
// Link Global Variables
extern "C" {
    static __arm64_phy_dram_start_addr__: usize;
    static __arm64_phy_prog_start_addr__: usize;
    static __boot_stacks_start__: usize;
    static __boot_stacks_end__: usize;
    static mut __bss_start__: usize;
    static __bss_end__: usize;
}
// Defined Global Variables
const CORE_ID_MASK: u8 = 0b11;
const ASSUMED_CORES: usize = 4;
const STACK_ALIGNMENT_MASK: usize = !(0x8);

// Define Very Initial Function
#[export_name = "_start"]
#[naked]
pub extern "C" fn setup_stack() -> ! {
    // This start function only sets up the stack pointer for each core
    unsafe {
        asm!(
            "
                // Compute Boot Stacks Size
                adr x0, __boot_stacks_start__
                adr x1, __boot_stacks_end__
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
            core_id_mask = const CORE_ID_MASK,
            core_count = const ASSUMED_CORES,
            stack_alignment_mask = const STACK_ALIGNMENT_MASK,
            rust_entrypoint = sym start,
            options(noreturn)
        )
    }
}

// Define rust entrypoint
pub unsafe fn start() -> ! {
    // Keep other cores parked (except main core)
    park_non_main_cores();
    // Clear BSS
    clear_bss();
    // Loop
    loop {}
}

// Define Helpers
#[inline(always)]
unsafe fn clear_bss() {
    let bss_start: *mut usize = &mut __bss_start__;
    let bss_end: *const usize = &__bss_end__;

    for bss_ptr in (0..bss_end.offset_from(bss_start)).map(|offset| bss_start.offset(offset)) {
        bss_ptr.as_uninit_mut().and_then(|data| Some(data.write(0)));
    }
}
