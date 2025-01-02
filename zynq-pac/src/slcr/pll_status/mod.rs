use crate::*;
pub struct PllStatusRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct PllStatusBuilder {
    value: u32,
}
impl FromBits<u32> for PllStatusBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<PllStatusBuilder, u32> for PllStatusRegister {
    fn read(&self) -> PllStatusBuilder {
        PllStatusBuilder::from_bits(self.inner.read())
    }
}
impl Default for PllStatusBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl PllStatusBuilder {
    pub fn arm_pll_lock(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_arm_pll_lock(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn ddr_pll_lock(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_ddr_pll_lock(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn io_pll_lock(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn with_io_pll_lock(mut self, value: bool) -> Self {
        self.value.set_bit(2usize, value);
        self
    }
    pub fn arm_pll_stable(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_arm_pll_stable(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
    pub fn ddr_pll_stable(&self) -> bool {
        self.value.get_bit(4usize)
    }
    pub fn with_ddr_pll_stable(mut self, value: bool) -> Self {
        self.value.set_bit(4usize, value);
        self
    }
    pub fn io_pll_stable(&self) -> bool {
        self.value.get_bit(5usize)
    }
    pub fn with_io_pll_stable(mut self, value: bool) -> Self {
        self.value.set_bit(5usize, value);
        self
    }
}
