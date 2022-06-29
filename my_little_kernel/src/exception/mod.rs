// Import Dependencies
// Define Vector Table creation Macros

macro_rules! _inner_vector_table {
    ($vector_table_id:ident) => {
        _inner_vector_table_header!($vector_table_id);
        _inner_vector_table_el_group!($vector_table_id, current_el);
        _inner_vector_table_el_group!($vector_table_id, lower_el);
        _inner_vector_table_jat!($vector_table_id);
    };
}

macro_rules! _inner_vector_table_header {
    ($vector_table_id:ident) => {
        core::arch::global_asm!(
            concat!(
                ".section .kernel_vector_table.",
                stringify!($vector_table_id),
                ".handlers",
                ", \"ax\",@progbits"
            ),
            ".balign 0x800",
            concat!("__vt_", stringify!($vector_table_id), " :"),
        );
    };
}

macro_rules! _inner_vector_table_el_group {
    ($vector_table_id:ident, $el_group:ident) => {
        _inner_vector_table_stack_sel!($vector_table_id, $el_group, sp_0);
        _inner_vector_table_stack_sel!($vector_table_id, $el_group, sp_n);
    };
}

macro_rules! _inner_vector_table_stack_sel {
    ($vector_table_id:ident, $el_group:ident, $stack_sel:ident) => {
        _inner_vector_table_exception_handler!($vector_table_id, $el_group, $stack_sel, _0_sync);
        _inner_vector_table_exception_handler!($vector_table_id, $el_group, $stack_sel, _1_irq);
        _inner_vector_table_exception_handler!($vector_table_id, $el_group, $stack_sel, _2_fiq);
        _inner_vector_table_exception_handler!($vector_table_id, $el_group, $stack_sel, _3_serr);
    };
}

macro_rules! _inner_vector_table_exception_handler {
    ($vector_table_id:ident, $el_group:ident, $stack_sel:ident, $exception_type:ident) => {
        paste::paste! {
            #[link_section = concat!(".kernel_vector_table", ".", stringify!($vector_table_id), ".handlers")]
            #[repr(align(0x80))]
            #[no_mangle]
            #[naked]
            pub unsafe extern "C" fn [< __vt_hdlr_ $vector_table_id _ $el_group _ $stack_sel  $exception_type >] () -> ! {
                asm!(
                    // Save context
                    "stp x29, x30, [sp, #-16]!",
                    // Get function address from JAT
                    concat!("ldr x30, ", "__vt_jat_", stringify!($vector_table_id), "_", stringify!($el_group), "_", stringify!($stack_sel), stringify!($exception_type)),
                    "br x30",
                    // Restore context
                    "ldp x29, x30, [sp], #16",
                    // Return from exception
                    "eret",
                    options(noreturn)
                );
            }
        }
    };
}

macro_rules! _inner_vector_table_jat {
    ($vector_table_id:ident) => {
        _inner_vector_table_jat_header!($vector_table_id);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_0, _0_sync);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_0, _1_irq);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_0, _2_fiq);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_0, _3_serr);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_n, _0_sync);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_n, _1_irq);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_n, _2_fiq);
        _inner_vector_table_jat_entry!($vector_table_id, current_el, sp_n, _3_serr);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_0, _0_sync);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_0, _1_irq);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_0, _2_fiq);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_0, _3_serr);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_n, _0_sync);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_n, _1_irq);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_n, _2_fiq);
        _inner_vector_table_jat_entry!($vector_table_id, lower_el, sp_n, _3_serr);
    };
}

macro_rules! _inner_vector_table_jat_header {
    ($vector_table_id:ident) => {
        core::arch::global_asm!(
            concat!(
                ".section .kernel_vector_table.",
                stringify!($vector_table_id),
                ".jat",
                ", \"aw\",@progbits"
            ),
            ".balign 0x800",
            concat!(".global ", "__vt_jat_", stringify!($vector_table_id)),
            concat!("__vt_jat_", stringify!($vector_table_id), " : "),
        );
    };
}

macro_rules! _inner_vector_table_jat_entry {
    ($vector_table_id:ident, $el_group:ident, $stack_sel:ident, $exception_type:ident) => {
        core::arch::global_asm!(concat!(
            "__vt_jat_",
            stringify!($vector_table_id),
            "_",
            stringify!($el_group),
            "_",
            stringify!($stack_sel),
            stringify!($exception_type),
            " : .dword 0x0"
        ));
    };
}

#[macro_export]
macro_rules! vector_table {
    ($vector_table_id:ident, $vector_table_name:ident) => {
        // Declare vector table
        _inner_vector_table!($vector_table_id);
        // Use it
        extern "C" {
            #[link_name = concat!("__vt_", stringify!($vector_table_id))]
            static mut $vector_table_name: VectorTable;
        }
    };
    ($vector_table_id:ident) => {
        // Declare vector table
        _inner_vector_table!($vector_table_id);
    };
}

#[macro_export]
macro_rules! use_vector_table {
    ($vector_table_id:ident, $vector_table_name:ident) => {
        extern "C" {
            #[link_name = concat!("__vt_", stringify!($vector_table_id))]
            static mut $vector_table_name: VectorTable;
        }
    };
}

// Define "Zero Cost" type for a Vector Table
#[repr(C)]
struct VectorTable {
    execution_levels: VectorTableExecutionLevels,
}

#[repr(C)]
struct VectorTableExecutionLevels {
    current_el: VectorTableStacks,
    lower_el: VectorTableStacks,
}

#[repr(C)]
struct VectorTableStacks {
    sp_0: VectorTableHandlers,
    sp_n: VectorTableHandlers,
}

#[repr(C)]
struct VectorTableHandlers {
    sync: VectorTableExceptionHandler,
    irq: VectorTableExceptionHandler,
    fiq: VectorTableExceptionHandler,
    serror: VectorTableExceptionHandler,
}

#[repr(transparent)]
struct VectorTableExceptionHandler(*const extern "C" fn() -> ());

// Define "High Level" Abstraction functions for the module
enum ExecutionLevel {
    Current,
    Lower,
}

enum StackSelection {
    SP0,
    SPn,
}
enum ExceptionType {
    Sync,
    IRQ,
    FIQ,
    SError,
}

// Implement methods for working with the vector table
impl VectorTableExecutionLevels {
    pub fn get_mut(&mut self, level: ExecutionLevel) -> &mut VectorTableStacks {
        match level {
            ExecutionLevel::Current => &mut self.current_el,
            ExecutionLevel::Lower => &mut self.lower_el
        }
    }
}

impl VectorTableStacks {
    pub fn get_mut(&mut self, stack: StackSelection) -> &mut VectorTableHandlers {
        match stack {
            StackSelection::SP0 => &mut self.sp_0,
            StackSelection::SPn => &mut self.sp_n
        }
    }
}

impl VectorTableHandlers {
    pub fn get_mut(&mut self, exception_type: ExceptionType) -> &mut VectorTableExceptionHandler {
        match exception_type {
            ExceptionType::Sync => &mut self.sync,
            ExceptionType::IRQ => &mut self.irq,
            ExceptionType::FIQ => &mut self.fiq,
            ExceptionType::SError => &mut self.serror,
        }
    }
}


impl VectorTable {
    pub fn set_handler(&mut self, level: ExecutionLevel, stack: StackSelection, exception_type: ExceptionType, handler: *const extern "C" fn() -> ()) {
        self.execution_levels.get_mut(level).get_mut(stack).get_mut(exception_type).0 = handler;
    }

    pub fn get_handler(&mut self, level: ExecutionLevel, stack: StackSelection, exception_type: ExceptionType) -> Option<&extern "C" fn()> {
        unsafe {
            self.execution_levels.get_mut(level).get_mut(stack).get_mut(exception_type).0.as_ref()
        }
    }
}