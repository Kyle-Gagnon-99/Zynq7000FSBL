#![no_std]
pub mod asm;
pub mod mutex;

#[inline(always)]
pub fn spin_lock_yield() {
    #[cfg(feature = "power_saving")]
    asm::wfe();
}

#[inline(always)]
pub fn notify_spin_lock() {
    #[cfg(feature = "power_saving")]
    {
        asm::dsb();
        asm::sev();
    }
}
