use core::arch::asm;

pub fn enable_fpu() {
    unsafe {
        asm!("
            mrc p15, 0, r1, c1, c0, 2
            orr r1, r1, (0b1111<<20)
            mcr p15, 0, r1, c1, c0, 2

            vmrs r1, fpexc
            orr r1, r1, (1<<30)
            vmsr fpexc, r1
            ", out("r1") _);
    }
}
