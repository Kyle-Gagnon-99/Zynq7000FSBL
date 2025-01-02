use crate::*;
pub struct Reg2EvCounterCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg2EvCounterCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for Reg2EvCounterCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg2EvCounterCtrlBuilder, u32> for Reg2EvCounterCtrlRegister {
    fn read(&self) -> Reg2EvCounterCtrlBuilder {
        Reg2EvCounterCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg2EvCounterCtrlBuilder, u32> for Reg2EvCounterCtrlRegister {
    fn zeroed() -> Reg2EvCounterCtrlBuilder {
        Reg2EvCounterCtrlBuilder::default()
    }
    fn write(&mut self, value: Reg2EvCounterCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg2EvCounterCtrlBuilder, u32> for Reg2EvCounterCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg2EvCounterCtrlBuilder) -> Reg2EvCounterCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg2EvCounterCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg2EvCounterCtrlBuilder {
    pub fn ev_ctr_en(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_ev_ctr_en(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn counter_reset(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_counter_reset(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
}
