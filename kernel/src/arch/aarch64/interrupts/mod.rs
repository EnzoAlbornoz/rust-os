// Import dependencies
use core::arch::asm;
use crate::exception_handler;
use self::vector_table::ExceptionStack;

use super::{cpu, ExceptionLevel};
use super::cpu::context::Context;
use armv8a_semihosting::hprintln;
use vector_table::{ExceptionKind, ExceptionRelativeLevel};
use enum_iterator::all;
// Define modules
mod vector_table;
// Export structs
pub use vector_table::VectorTable;
// Define iterrupt tables
vector_table::static_vector_table!(VECTOR_TABLE_EL1);
// Define procedures
pub unsafe fn setup_interrupts() {
    // Lock vector tables
    let mut vt_el1 = VECTOR_TABLE_EL1.lock();
    // Set default handlers
    vt_el1.set_default_handler(exception_handler!(default_handler));
    // Set specific handlers
    vt_el1.set_kind_handler(ExceptionKind::Sync, exception_handler!(handler_sync));
    // Update VBARs
    cpu::vbar(ExceptionLevel::El1, &vt_el1);
    
}
// Define Interruption Handlers
extern "C" fn default_handler(ctx: Context) {
    unimplemented!()
}

extern "C" fn handler_sync(ctx: Context) {
    hprintln!("Pemba: {:?}", ctx);
}