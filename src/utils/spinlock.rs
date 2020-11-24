use core::sync::atomic;

pub struct SpinLock {
    lock: atomic::AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            lock: atomic::AtomicBool::new(false)
        }
    }
    pub fn lock(&mut self) {
        while self.lock.swap(true, atomic::Ordering::Acquire) {
            while self.lock.load(atomic::Ordering::Relaxed) {
                atomic::spin_loop_hint();
            }
        }
    }
    pub fn unlock(&mut self) {
        self.lock.store(false, atomic::Ordering::Release);
    }
}

impl Drop for SpinLock {
    fn drop(&mut self) {
        self.unlock();
    }
}