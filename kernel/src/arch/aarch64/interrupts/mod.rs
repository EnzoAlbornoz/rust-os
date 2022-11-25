// Import dependencies
use super::ExecutionLevel;
use super::cpu;
// Define modules
mod vector_table;
// Export structs
pub use vector_table::VectorTable;
pub use vector_table::ExceptionHandler;
// Define iterrupt tables
vector_table::static_vector_table!(VECTOR_TABLE_EL1);
// Define procedures
pub unsafe fn setup_interrupts() {
    // Lock vector tables
    let vt_el1 = VECTOR_TABLE_EL1.lock();
    // Update handlers

    // Update VBARs
    cpu::vbar(ExecutionLevel::El1, &vt_el1);
}