// Import Dependencies
use core::arch::global_asm;
// Define Internal Macros
macro_rules! vector_table_handler_entry {
    ($el_group:literal, $stack_sel:literal, $exception_type:literal) => {
        concat!(
            ".balign 0x80",
            "\n",
            "__",
            "vbase_handler",
            "_",
            $el_group,
            "_",
            $stack_sel,
            "_",
            $exception_type,
            "__",
            ":",
            "\n",
            "stp x29, x30, [sp, #-16]!",
            "\n",
            "ldr x30, ",
            "__",
            "vbase_jat",
            "_",
            $el_group,
            "_",
            $stack_sel,
            "_",
            $exception_type,
            "__",
            "\n",
            "br x30",
            "\n",
            "ldp x29, x30, [sp], #16",
            "\n",
            "eret",
            "\n",
        )
    };
}
macro_rules! vector_table_jat_entry {
    ($el_group:literal, $stack_sel:literal, $exception_type:literal) => {
        concat!(
            "__",
            "vbase_jat",
            "_",
            $el_group,
            "_",
            $stack_sel,
            "_",
            $exception_type,
            "__",
            ": .dword 0x0"
        )
    };
}
// Define Base Vector Table
global_asm!(
    "
        .section .rodata.vector_table_base
        .balign 0x800
        .global __vector_table_base__
        __vector_table_base__:
    ",
    // Current Exception Level - Sp 0
    vector_table_handler_entry!("ce", "sp0", "sync"),
    vector_table_handler_entry!("ce", "sp0", "irq"),
    vector_table_handler_entry!("ce", "sp0", "fiq"),
    vector_table_handler_entry!("ce", "sp0", "serr"),
    // Current Exception Level - Sp N
    vector_table_handler_entry!("ce", "spn", "sync"),
    vector_table_handler_entry!("ce", "spn", "irq"),
    vector_table_handler_entry!("ce", "spn", "fiq"),
    vector_table_handler_entry!("ce", "spn", "serr"),
    // Lower Exception Level - Sp 0
    vector_table_handler_entry!("le", "sp0", "sync"),
    vector_table_handler_entry!("le", "sp0", "irq"),
    vector_table_handler_entry!("le", "sp0", "fiq"),
    vector_table_handler_entry!("le", "sp0", "serr"),
    // Lower Exception Level - Sp N
    vector_table_handler_entry!("le", "spn", "sync"),
    vector_table_handler_entry!("le", "spn", "irq"),
    vector_table_handler_entry!("le", "spn", "fiq"),
    vector_table_handler_entry!("le", "spn", "serr"),
    // Jump Address Table
    vector_table_jat_entry!("ce", "sp0", "sync"),
    vector_table_jat_entry!("ce", "sp0", "irq"),
    vector_table_jat_entry!("ce", "sp0", "fiq"),
    vector_table_jat_entry!("ce", "sp0", "serr"),
    // Current Exception Level - Sp N
    vector_table_jat_entry!("ce", "spn", "sync"),
    vector_table_jat_entry!("ce", "spn", "irq"),
    vector_table_jat_entry!("ce", "spn", "fiq"),
    vector_table_jat_entry!("ce", "spn", "serr"),
    // Lower Exception Level - Sp 0
    vector_table_jat_entry!("le", "sp0", "sync"),
    vector_table_jat_entry!("le", "sp0", "irq"),
    vector_table_jat_entry!("le", "sp0", "fiq"),
    vector_table_jat_entry!("le", "sp0", "serr"),
    // Lower Exception Level - Sp N
    vector_table_jat_entry!("le", "spn", "sync"),
    vector_table_jat_entry!("le", "spn", "irq"),
    vector_table_jat_entry!("le", "spn", "fiq"),
    vector_table_jat_entry!("le", "spn", "serr"),
);
// Define Strucutre
#[repr(C, align(0x800))]
pub struct VectorTable {
    text: [u8; 0x800],
    handlers: [u64; 16],
}
// Implement Vector Table
impl VectorTable {
    pub fn new() -> Self {
        // Link to skeleton vector table
        extern "C" {
            #[link_name = "__vector_table__base__"]
            static vector_table_base: VectorTable;
        }
        // Instantiate a new VT copying the instructions
        // that jumps to the handler address
        return Self {
            text: unsafe { vector_table_base.text.clone() },
            handlers: [0; 16],
        };
    }
}