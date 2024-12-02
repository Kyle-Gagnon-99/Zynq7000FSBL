use core::{
    cell::UnsafeCell,
    future::Future,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
    task::Poll,
};

use crate::{
    asm::{self, enter_critical},
    notify_spin_lock, spin_lock_yield,
};

const LOCKED: u32 = 1;
const UNLOCKED: u32 = 0;

pub struct Mutex<T> {
    locked: AtomicU32,
    inner: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Mutex<T> {}
unsafe impl<T: Send> Send for Mutex<T> {}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    irq: bool,
}

struct Fut<'a, T>(&'a Mutex<T>);

impl<'a, T> Future for Fut<'a, T> {
    type Output = MutexGuard<'a, T>;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let irq = unsafe { asm::enter_critical() };
        if self
            .0
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            unsafe { asm::exit_critical(irq) };
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(MutexGuard { mutex: self.0, irq })
        }
    }
}

impl<T> Mutex<T> {
    /// Constructor, const-fn, creates a new Mutex
    pub const fn new(inner: T) -> Self {
        Mutex {
            locked: AtomicU32::new(UNLOCKED),
            inner: UnsafeCell::new(inner),
        }
    }

    /// Locks the mutex, blocking until it is available
    pub fn lock(&self) -> MutexGuard<T> {
        let mut irq = unsafe { asm::enter_critical() };
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            unsafe {
                asm::exit_critical(irq);
                spin_lock_yield();
                irq = enter_critical();
            }
        }
        MutexGuard { mutex: self, irq }
    }

    /// Locks the mutex, returning a future that resolves when the lock is acquired
    pub async fn async_lock(&self) -> MutexGuard<'_, T> {
        Fut(self).await
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        let irq = unsafe { asm::enter_critical() };
        if self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            unsafe { asm::exit_critical(irq) };
            None
        } else {
            Some(MutexGuard { mutex: self, irq })
        }
    }

    fn unlock(&self) {
        self.locked.store(UNLOCKED, Ordering::Release);

        notify_spin_lock();
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.inner.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
        unsafe { asm::exit_critical(self.irq) };
    }
}
