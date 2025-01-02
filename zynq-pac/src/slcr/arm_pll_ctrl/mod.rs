use crate::*;
pub struct ArmPllCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct ArmPllCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for ArmPllCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<ArmPllCtrlBuilder, u32> for ArmPllCtrlRegister {
    fn read(&self) -> ArmPllCtrlBuilder {
        ArmPllCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<ArmPllCtrlBuilder, u32> for ArmPllCtrlRegister {
    fn zeroed() -> ArmPllCtrlBuilder {
        ArmPllCtrlBuilder::default()
    }
    fn write(&mut self, value: ArmPllCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<ArmPllCtrlBuilder, u32> for ArmPllCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(ArmPllCtrlBuilder) -> ArmPllCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for ArmPllCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(3usize, true);
        value.set_bits(12usize..=18usize, 26usize as u32);
        Self { value }
    }
}
impl ArmPllCtrlBuilder {
    pub fn pll_reset(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_pll_reset(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn pll_pwrdwn(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_pll_pwrdwn(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn pll_bypass_qual(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_pll_bypass_qual(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
    pub fn pll_bypass_force(&self) -> bool {
        self.value.get_bit(4usize)
    }
    pub fn with_pll_bypass_force(mut self, value: bool) -> Self {
        self.value.set_bit(4usize, value);
        self
    }
    pub fn pll_fdiv(&self) -> u8 {
        self.value.get_bits(12usize..=18usize) as u8
    }
    pub fn with_pll_fdiv(mut self, value: u8) -> Self {
        self.value.set_bits(12usize..=18usize, value as u32);
        self
    }
}
