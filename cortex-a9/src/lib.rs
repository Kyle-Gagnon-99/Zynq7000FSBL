#![no_std]
#![feature(naked_functions)]
#![feature(never_type)]
pub mod asm;
pub mod cache;
pub mod fpu;
pub mod l2c;
pub mod mmu;
pub mod mutex;
pub mod regs;

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

#[macro_export]
macro_rules! interrupt_handler {
    ($name:ident, $name2:ident, $stack0:ident, $stack1:ident, $body:block) => {
        use core::arch::asm;

        #[link_section = ".text.boot"]
        #[no_mangle]
        #[naked]
        pub unsafe extern "C" fn $name() -> ! {
            asm!(
                // Setup the stack pointer, depending on CPU 0 or 1
                // and preserve the registers
                "sub lr, lr, #4",
                "stmfd sp!, {{r0-r12, lr}}",
                "mrc p15, #0, r0, c0, c0, #5",
                concat!("movw r1, :lower16:", stringify!($stack0)),
                concat!("movt r1, :upper16:", stringify!($stack0)),
                "tst r0, #3",
                concat!("movwne r1, :lower16:", stringify!($stack1)),
                concat!("movtne r1, :upper16:", stringify!($stack1)),
                "mov r0, sp",
                "mov sp, r1",
                "push {{r0, r1}}",                                      // 2 registers are pushed to maintain 8 byte stack alignment
                concat!("bl ", stringify!($name2)),
                "pop {{r0, r1}}",
                "mov sp, r0",
                "ldmfd sp!, {{r0-r12, pc}}^",                           // caret ^ : copy SPSR to the CPSR
                options(noreturn)
            );
        }

        #[no_mangle]
        pub unsafe extern "C" fn $name2() $body
    };
}
