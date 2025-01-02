use crate::*;
pub struct DdrPllCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct DdrPllCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for DdrPllCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<DdrPllCtrlBuilder, u32> for DdrPllCtrlRegister {
    fn read(&self) -> DdrPllCtrlBuilder {
        DdrPllCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<DdrPllCtrlBuilder, u32> for DdrPllCtrlRegister {
    fn zeroed() -> DdrPllCtrlBuilder {
        DdrPllCtrlBuilder::default()
    }
    fn write(&mut self, value: DdrPllCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<DdrPllCtrlBuilder, u32> for DdrPllCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(DdrPllCtrlBuilder) -> DdrPllCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for DdrPllCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(3usize, true);
        value.set_bits(12usize..=18usize, 26usize as u32);
        Self { value }
    }
}
impl DdrPllCtrlBuilder {
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
