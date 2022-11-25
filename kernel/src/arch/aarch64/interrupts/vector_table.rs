// Define internal macros
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
            // Current exception level - Sp 0
            $crate::static_vector_table!(handler "11"),
            $crate::static_vector_table!(handler "12"),
            $crate::static_vector_table!(handler "13"),
            $crate::static_vector_table!(handler "14"),
            // Current exception level - Sp N
            $crate::static_vector_table!(handler "21"),
            $crate::static_vector_table!(handler "22"),
            $crate::static_vector_table!(handler "23"),
            $crate::static_vector_table!(handler "24"),
            // Lower exception level - Sp 0
            $crate::static_vector_table!(handler "31"),
            $crate::static_vector_table!(handler "32"),
            $crate::static_vector_table!(handler "33"),
            $crate::static_vector_table!(handler "34"),
            // Lower exception level - Sp N
            $crate::static_vector_table!(handler "41"),
            $crate::static_vector_table!(handler "42"),
            $crate::static_vector_table!(handler "43"),
            $crate::static_vector_table!(handler "44"),

            // Jump address table
            // Current exception level - Sp 0
            $crate::static_vector_table!(entry "11"),
            $crate::static_vector_table!(entry "12"),
            $crate::static_vector_table!(entry "13"),
            $crate::static_vector_table!(entry "14"),
            // Current exception level - Sp N
            $crate::static_vector_table!(entry "21"),
            $crate::static_vector_table!(entry "22"),
            $crate::static_vector_table!(entry "23"),
            $crate::static_vector_table!(entry "24"),
            // Lower exception level - Sp 0
            $crate::static_vector_table!(entry "31"),
            $crate::static_vector_table!(entry "32"),
            $crate::static_vector_table!(entry "33"),
            $crate::static_vector_table!(entry "34"),
            // Lower exception level - Sp N
            $crate::static_vector_table!(entry "41"),
            $crate::static_vector_table!(entry "42"),
            $crate::static_vector_table!(entry "43"),
            $crate::static_vector_table!(entry "44"),

            // Spinlock - Atomic Bool
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



// Export macros
pub(crate) use static_vector_table;
// Define strucutres
#[repr(C, align(0x800))]
pub struct VectorTable {
    text: [u8; 0x800],
    handlers: [extern "C" fn(); 16],
}

pub struct ExceptionHandler();
// Implement vector table
impl VectorTable {
    // pub fn set_handler() {
        
    // }
}
