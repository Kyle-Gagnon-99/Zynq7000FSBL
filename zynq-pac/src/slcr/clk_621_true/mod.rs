use crate::*;
pub struct Clk621TrueRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Clk621TrueBuilder {
    value: u32,
}
impl FromBits<u32> for Clk621TrueBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Clk621TrueBuilder, u32> for Clk621TrueRegister {
    fn read(&self) -> Clk621TrueBuilder {
        Clk621TrueBuilder::from_bits(self.inner.read())
    }
}
impl Default for Clk621TrueBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(0usize, false);
        Self { value }
    }
}
impl Clk621TrueBuilder {
    pub fn clk_621_true(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_clk_621_true(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
}
