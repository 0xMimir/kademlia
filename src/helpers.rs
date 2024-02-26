use std::sync::{Mutex, MutexGuard};

pub trait ExpectLock<T: ?Sized> {
    fn expect_lock(&self) -> MutexGuard<'_, T>;
}

impl<T: ?Sized> ExpectLock<T> for Mutex<T> {
    fn expect_lock(&self) -> MutexGuard<'_, T> {
        self.lock().expect("Error locking")
    }
}
