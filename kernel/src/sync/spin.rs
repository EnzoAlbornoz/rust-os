// Import dependencies
use core::{cell::UnsafeCell, sync::atomic::{Ordering, AtomicBool}, hint::spin_loop, ops::{Deref, DerefMut}};

/// Spinslocks should be only used in Low-Level environments,
/// so, it should be FFI-compatible, implementing #[repr(C)]
#[repr(C)]
pub struct Spinlock<T> {
    data: UnsafeCell<T>,
    lock: AtomicBool,
}

pub struct SpinlockGuard<'lock, T> {
    spin: &'lock Spinlock<T>,
}

// Implement structs
impl<T> Spinlock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        // Use weak version, because we already in a loop (non-weak runs on a loop)
        while self.lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // MESI Protocol: Cores should use shared state (read-only)
            // while waiting for the lock to release in order to use less resources.
            while self.lock.load(Ordering::Relaxed) == false {
                spin_loop()
            }
        }
        // We have locked the atomic value (memory aquired)
        SpinlockGuard {
            spin: self
        }
    }
}

impl<'lock, T> Drop for SpinlockGuard<'lock, T> {
    fn drop(&mut self) {
        // Write 0 to the lock status and release the memory atomic value
        self.spin.lock.store(false, Ordering::Release)
    }
}


// Implement transparency for the locked value
// SAFETY: (lifetime garentees the value to exist)

impl<T> Deref for SpinlockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Only this thread/core is accessing this value.
        // Return a reference to the underlying data
        unsafe { &*self.spin.data.get() }
    }
}

impl<T> DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: Only this thread/core is accessing this value.
        // Return a reference to the underlying data
        unsafe { &mut *self.spin.data.get() }
    }
}