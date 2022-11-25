// Define modules
mod boot;
mod cpu;
mod interrupts;
// Define shared structs and constants
pub enum ExecutionLevel {
    El1,
    El2,
    El3
}