// Import dependencies
use bitflags::bitflags;
// Define Macros
#[macro_export]
macro_rules! asm_push_context {
    // True for interruptions
    (true) => {
        concat!(
            // Store registers x30-x0 + sp0 (user stack pointer)
            concat!($crate::asm_push_context!(@reg_store), "\n"),
            // Store interrupt specific data
            "
                // Compute and store ELR and SPSR
                mrs x1, ELR_EL1
                mrs x0, SPSR_EL1
                stp x0, x1, [sp, #-16]!
                // Restore x0 and x1 (sp = [*spsr, pc, usp, (x0, x1)])
                ldp x0, x1, [sp, #8 * 3]
            "
        )
    };
    (false) => {
        concat!(
            // Store registers x30-x0 + sp0 (user stack pointer)
            concat!($crate::asm_push_context!(@reg_store), "\n"),
            // Store context switch specific data
            "
                // Alloc space for Flags and PC
                sub sp, sp, #16
                // PC and ELR point to the same instruction
                str x30, [sp, #8]
                // Save auxiliary register (to load system registers)
                str x18, [sp]
                // Compute Flags (NZCV | CurrentEL | SpSel)
                mov x30, xzr
                mrs x18, NZCV
                orr x30, x30, x18
                mrs x18, CurrentEl
                orr x30, x30, x18
                mrs x18, SPSel
                orr x30, x30, x18
                // Restore auxiliary register
                ldr x18, [sp]
                // Save Flags
                str x30, [sp]
                // Restore x30
                ldr x30, [sp, #8]
            "
        )
    };
    () => { $crate::asm_push_context!(false) };
    // Implementation details
    (@reg_store) => {
        "
            // Store x30-x1
            stp x29, x30, [sp, #-16]!
            stp x27, x28, [sp, #-16]!
            stp x25, x26, [sp, #-16]!
            stp x23, x24, [sp, #-16]!
            stp x21, x22, [sp, #-16]!
            stp x19, x20, [sp, #-16]!
            stp x17, x18, [sp, #-16]!
            stp x15, x16, [sp, #-16]!
            stp x13, x14, [sp, #-16]!
            stp x11, x12, [sp, #-16]!
            stp  x9, x10, [sp, #-16]!
            stp  x7,  x8, [sp, #-16]!
            stp  x5,  x6, [sp, #-16]!
            stp  x3,  x4, [sp, #-16]!
            stp  x1,  x2, [sp, #-16]!
            
            // Store x0 and USP
            // (Alloc two, push one, compute, push computed, restore)
            sub  sp,  sp, #16

            str  x0, [sp, #8]
            mrs  x0, SP_EL0
            str  x0, [sp]
            ldr  x0, [sp, #8]
        "
    };
}

#[macro_export]
macro_rules! asm_pop_context {
    // True for interruptions
    (true) => {
        concat!(
            // Load interrupt specific data
            "
                // Load Flags into x0 and PC into x1
                ldp  x0,  x1, [sp], #16
                // Flags into SPSR and PC into ELR
                msr SPSR_EL1, x0
                msr  ELR_EL1, x1
            ",
            // Load registers sp0 (user stack pointer) + x0-x30
            concat!($crate::asm_pop_context!(@reg_load), "\n"),
        )
    };
    (false) => {
        concat!(
            // Load context switch specific data
            "
                // Load Flags into x0 and PC into x1
                ldp  x0,  x1, [sp], #16
                // Flags into SPSR and PC into ELR
                msr SPSR_EL1, x0
                msr  ELR_EL1, x1
            ",
            // Load registers sp0 (user stack pointer) + x0-x30
            concat!($crate::asm_pop_context!(@reg_load), "\n"),
            // // Include symbol to jump after
            // "99: ret"
        )
    };
    () => { $crate::asm_pop_context!(false) };
    // Implementation details
    (@reg_load) => {
        "
            // Load USP and x0 (loads USP in x1, and x0 in x0)
            ldp  x1,  x0, [sp], #16
            msr   SP_EL0, x1 

            // Load x1-x30
            ldp  x1,  x2, [sp], #16
            ldp  x3,  x4, [sp], #16
            ldp  x5,  x6, [sp], #16
            ldp  x7,  x8, [sp], #16
            ldp  x9, x10, [sp], #16
            ldp x11, x12, [sp], #16
            ldp x13, x14, [sp], #16
            ldp x15, x16, [sp], #16
            ldp x17, x18, [sp], #16
            ldp x19, x20, [sp], #16
            ldp x21, x22, [sp], #16
            ldp x23, x24, [sp], #16
            ldp x25, x26, [sp], #16
            ldp x27, x28, [sp], #16
            ldp x29, x30, [sp], #16
        "
    };
}
// Export Macros
pub(crate) use asm_push_context;
pub(crate) use asm_pop_context;
// Define structs
bitflags! {
    pub struct Flags: u64 {
        /// Negative Condition flag
        const N = 1 << 31;
        /// Zero Condition flag
        const Z = 1 << 30;
        /// Carry Condition flag
        const C = 1 << 29;
        /// Overflow Condition flag
        const V = 1 << 28;

        /// Software Step
        const SS = 1 << 21;
        // Illegal Execution state
        const IL = 1 << 20;

        /// Debug exception mask
        const D = 1 << 9;
        /// SError interrupt mask
        const A = 1 << 8;
        /// IRQ interrupt mask
        const I = 1 << 7;
        /// FIQ interrupt mask
        const F = 1 << 6;

        /// Execution state (1 for AArch64)
        const M = 1 << 4;

        // Exeception Levels
        const EL_0 = 0b00 << 2;
        const EL_1 = 0b01 << 2;

        // Stack Selector
        const SP_0 = 0;
        const SP_N = 1;
    }
}
/// Defines a CPU Context that can be on stack.
/// 
/// #\[repr(C)] is used to maintain correct arrangement on memory
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    // OBS: Current stack pointer (sp) is the Context reference itself,
    // so we doesn't need to store this data

    // Lower Address  
    flags: Flags,
    pc: *const u8,
    usp: *const u8,
    x00: usize,
    x01: usize,
    x02: usize,
    x03: usize,
    x04: usize,
    x05: usize,
    x06: usize,
    x07: usize,
    x08: usize,
    x09: usize,
    x10: usize,
    x11: usize,
    x12: usize,
    x14: usize,
    x13: usize,
    x15: usize,
    x16: usize,
    x17: usize,
    x18: usize,
    x19: usize,
    x20: usize,
    x21: usize,
    x22: usize,
    x23: usize,
    x24: usize,
    x25: usize,
    x26: usize,
    x27: usize,
    x28: usize,
    x29: usize,
    /// Link Register
    x30: usize, 
    // Higher address
}