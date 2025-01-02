use crate::*;
pub struct DdrClkCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct DdrClkCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for DdrClkCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<DdrClkCtrlBuilder, u32> for DdrClkCtrlRegister {
    fn read(&self) -> DdrClkCtrlBuilder {
        DdrClkCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<DdrClkCtrlBuilder, u32> for DdrClkCtrlRegister {
    fn zeroed() -> DdrClkCtrlBuilder {
        DdrClkCtrlBuilder::default()
    }
    fn write(&mut self, value: DdrClkCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<DdrClkCtrlBuilder, u32> for DdrClkCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(DdrClkCtrlBuilder) -> DdrClkCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for DdrClkCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(0usize, false);
        value.set_bit(1usize, true);
        value.set_bits(20usize..=25usize, 4usize as u32);
        value.set_bits(26usize..=31usize, 6usize as u32);
        Self { value }
    }
}
impl DdrClkCtrlBuilder {
    pub fn ddr_3_xclkact(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_ddr_3_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn ddr_2_xclkact(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_ddr_2_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn ddr_3_xclk_divisor(&self) -> u8 {
        self.value.get_bits(20usize..=25usize) as u8
    }
    pub fn with_ddr_3_xclk_divisor(mut self, value: u8) -> Self {
        self.value.set_bits(20usize..=25usize, value as u32);
        self
    }
    pub fn ddr_2_xclk_divisor(&self) -> u8 {
        self.value.get_bits(26usize..=31usize) as u8
    }
    pub fn with_ddr_2_xclk_divisor(mut self, value: u8) -> Self {
        self.value.set_bits(26usize..=31usize, value as u32);
        self
    }
}
