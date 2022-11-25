// Import dependencies
// Define structs

/// Defines a CPU Context that can be on stack.
/// 
/// #\[repr(C)] is used to maintain correct arrangement on memory
#[repr(C)]
pub struct Context {
    x30: usize,
    x29: usize,
    x28: usize,
    x27: usize,
    x26: usize,
    x25: usize,
    x24: usize,
    x23: usize,
    x22: usize,
    x21: usize,
    x20: usize,
    x19: usize,
    x18: usize,
    x17: usize,
    x16: usize,
    x15: usize,
    x14: usize,
    x13: usize,
    x12: usize,
    x11: usize,
    x10: usize,
    x09: usize,
    x08: usize,
    x07: usize,
    x06: usize,
    x05: usize,
    x04: usize,
    x03: usize,
    x02: usize,
    x01: usize,
    x00: usize,
}