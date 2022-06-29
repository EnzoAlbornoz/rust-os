// Import Dependencies
use crate::arch::{core_id, park_core};
// Define Functions
#[inline(always)]
pub fn park_non_main_cores() {
    unsafe {
        if core_id() != 0 {
            park_core()
        }
    }
}
