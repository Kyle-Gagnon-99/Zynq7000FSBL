use core::ptr::addr_of_mut;

// Define the available linker symbols and the entry function to hand off after booting
use cortex_a9::{
    cache::{bpiall, dciall_l1, iciallu, tlbiall},
    fpu::enable_fpu,
    interrupt_handler,
    regs::MPIDR,
    spin_lock_yield,
};
use r0::zero_bss;
use vcell::VolatileCell;
extern "C" {
    static mut __bss_start: u32;
    static mut __bss_end: u32;
    static __stack0_start: u32;
    static __stack1_start: u32;
    fn main_core0();
    #[allow(dead_code)]
    fn main_core1();
}

static mut CORE1_ENABLED: VolatileCell<bool> = VolatileCell::new(false);

interrupt_handler!(Reset, reset_irq, __stack0_start, __stack1_start, {
    match MPIDR.get_cpu_id() {
        0 => {
            boot_core0();
        }
        1 => {
            unsafe {
                while !CORE1_ENABLED.get() {
                    spin_lock_yield();
                }
            }
            boot_core1();
        }
        _ => panic!("Invalid CPU ID"),
    }
});

#[inline(never)]
unsafe extern "C" fn boot_core0() -> ! {
    // Enable the L1 cache
    l1_cache_init();

    // Enable the FPU
    enable_fpu();

    // Zero out the .bss section
    zero_bss(addr_of_mut!(__bss_start), addr_of_mut!(__bss_end));

    main_core0();
    panic!("main_core0 returned");
}

#[inline(never)]
pub extern "C" fn boot_core1() -> ! {
    panic!("Core 1 should not have been started");
}

fn l1_cache_init() {
    // Invalidate TLBs
    tlbiall();

    // Invalidate I-cache (instruction cache)
    iciallu();

    // Invalidate branch predictor array
    bpiall();

    // Invalidate D-Cache
    dciall_l1();
}
