use zynq_pac::{
    slcr::{self, slcr_lock::SlcrLockBuilder, slcr_unlock::SlcrUnlockBuilder},
    RegisterWO,
};

pub struct Slcr {
    pub regs: &'static mut slcr::RegisterBlock,
}

impl Slcr {
    pub fn slcr() -> Self {
        Self {
            regs: &mut *slcr::RegisterBlock::slcr0(),
        }
    }

    /// Allows the sclr register to be modified
    ///
    /// # Arguments
    /// * `f` - A closure that takes a mutable reference to the slcr register block
    pub fn unlocked<F: FnMut(&mut slcr::RegisterBlock)>(mut f: F) {
        let self_ = Self::slcr();
        self_.regs.slcr_unlock.write(SlcrUnlockBuilder::default());
        f(self_.regs);
        self_.regs.slcr_lock.write(SlcrLockBuilder::default());
    }
}
