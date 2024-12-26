use crate::*;
pub struct SrRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct SrBuilder {
    value: u32,
}
impl FromBits<u32> for SrBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<SrBuilder, u32> for SrRegister {
    fn read(&self) -> SrBuilder {
        SrBuilder::from_bits(self.inner.read())
    }
}
impl Default for SrBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl SrBuilder {
    pub fn rx_fifo_trigger(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn rx_fifo_empty(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn rx_fifo_full(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn tx_fifo_empty(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn tx_fifo_full(&self) -> bool {
        self.value.get_bit(4usize)
    }
}
