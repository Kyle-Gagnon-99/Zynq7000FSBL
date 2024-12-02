use core::arch::asm;

/// No operation
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop", options(nomem, nostack, preserves_flags)) }
}

/// Wait For Event
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe", options(nomem, nostack, preserves_flags)) }
}

/// Send Event
#[inline(always)]
pub fn sev() {
    unsafe { asm!("sev", options(nomem, nostack, preserves_flags)) }
}

/// Wait For Interrupt
#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) }
}

/// Data Memory Barrier
#[inline(always)]
pub fn dmb() {
    unsafe { asm!("dmb", options(nomem, nostack, preserves_flags)) }
}

/// Data Synchronization Barrier
#[inline(always)]
pub fn dsb() {
    unsafe { asm!("dsb", options(nomem, nostack, preserves_flags)) }
}

/// Instruction Synchronization Barrier
#[inline(always)]
pub fn isb() {
    unsafe { asm!("isb", options(nomem, nostack, preserves_flags)) }
}

/// Enable FIQ
#[inline(always)]
pub fn enable_fiq() {
    unsafe { asm!("cpsie f", options(nomem, nostack, preserves_flags)) }
}

/// Enable IRQ
#[inline(always)]
pub fn enable_irq() {
    unsafe { asm!("cpsie i", options(nomem, nostack, preserves_flags)) }
}

/// Disable IRQ, return if IRQ was originally enabled
#[inline(always)]
pub unsafe fn enter_critical() -> bool {
    let mut cpsr: u32;
    asm!("mrs {0}, cpsr", "cpsid i", out(reg) cpsr, options(nomem, nostack, preserves_flags));
    (cpsr & (1 << 7)) == 0
}

#[inline(always)]
pub unsafe fn exit_critical(enable: bool) {
    let mask: u32 = if enable { 1 >> 7 } else { 0 };
    asm!("msr cpsr_c, {0}", in(reg) mask, options(nomem, nostack, preserves_flags));
}

/// Exit IRQ
#[inline(always)]
pub fn exit_irq() {
    unsafe { asm!("subs pc, lr, #4", options(nomem, nostack, preserves_flags)) }
}
