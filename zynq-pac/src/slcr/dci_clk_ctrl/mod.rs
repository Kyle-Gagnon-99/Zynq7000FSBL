use crate::*;
pub struct DciClkCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct DciClkCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for DciClkCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<DciClkCtrlBuilder, u32> for DciClkCtrlRegister {
    fn read(&self) -> DciClkCtrlBuilder {
        DciClkCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<DciClkCtrlBuilder, u32> for DciClkCtrlRegister {
    fn zeroed() -> DciClkCtrlBuilder {
        DciClkCtrlBuilder::default()
    }
    fn write(&mut self, value: DciClkCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<DciClkCtrlBuilder, u32> for DciClkCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(DciClkCtrlBuilder) -> DciClkCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for DciClkCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(0usize, false);
        value.set_bits(8usize..=13usize, 50usize as u32);
        value.set_bits(20usize..=25usize, 30usize as u32);
        Self { value }
    }
}
impl DciClkCtrlBuilder {
    pub fn clkact(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_clkact(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn divisor0(&self) -> u8 {
        self.value.get_bits(8usize..=13usize) as u8
    }
    pub fn with_divisor0(mut self, value: u8) -> Self {
        self.value.set_bits(8usize..=13usize, value as u32);
        self
    }
    pub fn divisor1(&self) -> u8 {
        self.value.get_bits(20usize..=25usize) as u8
    }
    pub fn with_divisor1(mut self, value: u8) -> Self {
        self.value.set_bits(20usize..=25usize, value as u32);
        self
    }
}
