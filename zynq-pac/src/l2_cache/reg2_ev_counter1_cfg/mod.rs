use crate::*;
pub struct Reg2EvCounter1CfgRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg2EvCounter1CfgBuilder {
    value: u32,
}
impl FromBits<u32> for Reg2EvCounter1CfgBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg2EvCounter1CfgBuilder, u32> for Reg2EvCounter1CfgRegister {
    fn read(&self) -> Reg2EvCounter1CfgBuilder {
        Reg2EvCounter1CfgBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg2EvCounter1CfgBuilder, u32> for Reg2EvCounter1CfgRegister {
    fn zeroed() -> Reg2EvCounter1CfgBuilder {
        Reg2EvCounter1CfgBuilder::default()
    }
    fn write(&mut self, value: Reg2EvCounter1CfgBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg2EvCounter1CfgBuilder, u32> for Reg2EvCounter1CfgRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg2EvCounter1CfgBuilder) -> Reg2EvCounter1CfgBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg2EvCounter1CfgBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg2EvCounter1CfgBuilder {
    pub fn ev_ctr_intr_gen(&self) -> u8 {
        self.value.get_bits(0usize..=1usize) as u8
    }
    pub fn with_ev_ctr_intr_gen(mut self, value: u8) -> Self {
        self.value.set_bits(0usize..=1usize, value as u32);
        self
    }
    pub fn ctr_ev_src(&self) -> u8 {
        self.value.get_bits(2usize..=5usize) as u8
    }
    pub fn with_ctr_ev_src(mut self, value: u8) -> Self {
        self.value.set_bits(2usize..=5usize, value as u32);
        self
    }
}
