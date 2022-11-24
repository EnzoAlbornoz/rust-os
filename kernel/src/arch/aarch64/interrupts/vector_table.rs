// Define Internal Macros
#[macro_export]
macro_rules! static_vector_table {
    ($vector_table_name:ident) => {        
        extern "C" {
            static $vector_table_name: crate::sync::spin::Spinlock<crate::arch::aarch64::interrupts::vector_table::VectorTable>;
        }

        core::arch::global_asm!(
            // Header
            "
                .section .kernel_vector_table
                .balign 0x800
                .global {0}
                {0}:
            ",
            // Handlers

            // Current Exception Level - Sp 0
            $crate::static_vector_table!(handler "11"),
            $crate::static_vector_table!(handler "12"),
            $crate::static_vector_table!(handler "13"),
            $crate::static_vector_table!(handler "14"),
            // Current Exception Level - Sp N
            $crate::static_vector_table!(handler "21"),
            $crate::static_vector_table!(handler "22"),
            $crate::static_vector_table!(handler "23"),
            $crate::static_vector_table!(handler "24"),
            // Lower Exception Level - Sp 0
            $crate::static_vector_table!(handler "31"),
            $crate::static_vector_table!(handler "32"),
            $crate::static_vector_table!(handler "33"),
            $crate::static_vector_table!(handler "34"),
            // Lower Exception Level - Sp N
            $crate::static_vector_table!(handler "41"),
            $crate::static_vector_table!(handler "42"),
            $crate::static_vector_table!(handler "43"),
            $crate::static_vector_table!(handler "44"),
            // Jump Address Table

            // Current Exception Level - Sp 0
            $crate::static_vector_table!(entry "11"),
            $crate::static_vector_table!(entry "12"),
            $crate::static_vector_table!(entry "13"),
            $crate::static_vector_table!(entry "14"),
            // Current Exception Level - Sp N
            $crate::static_vector_table!(entry "21"),
            $crate::static_vector_table!(entry "22"),
            $crate::static_vector_table!(entry "23"),
            $crate::static_vector_table!(entry "24"),
            // Lower Exception Level - Sp 0
            $crate::static_vector_table!(entry "31"),
            $crate::static_vector_table!(entry "32"),
            $crate::static_vector_table!(entry "33"),
            $crate::static_vector_table!(entry "34"),
            // Lower Exception Level - Sp N
            $crate::static_vector_table!(entry "41"),
            $crate::static_vector_table!(entry "42"),
            $crate::static_vector_table!(entry "43"),
            $crate::static_vector_table!(entry "44"),
            // Spinlock AtomicBool
            ".byte 0x0",
            // Format configurations
            sym $vector_table_name
        );
    };

    (handler $jat_label:literal) => {
        concat!(
            concat!(".balign 0x80", "\n"),
            concat!("stp x29, x30, [sp, #-16]!", "\n"),
            concat!("ldr x30, ", $jat_label, "f", "\n"),
            concat!("br x30", "\n"),
            concat!("ldp x29, x30, [sp], #16", "\n"),
            concat!("eret", "\n")
        )
    };

    (entry $jat_label:literal) => {
        concat!($jat_label, ":", ".dword 0x0", "\n")
    };

}
// Export Macros
pub(crate) use static_vector_table;
// Define Strucutre
#[repr(C, align(0x800))]
pub struct VectorTable {
    text: [u8; 0x800],
    handlers: [extern "C" fn(); 16],
}
// Implement Vector Table
impl VectorTable {
    
}
