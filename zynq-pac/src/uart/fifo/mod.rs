use crate::*;
pub struct FifoRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct FifoBuilder {
    value: u32,
}
impl FromBits<u32> for FifoBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<FifoBuilder, u32> for FifoRegister {
    fn read(&self) -> FifoBuilder {
        FifoBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<FifoBuilder, u32> for FifoRegister {
    fn zeroed() -> FifoBuilder {
        FifoBuilder::default()
    }
    fn write(&mut self, value: FifoBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<FifoBuilder, u32> for FifoRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(FifoBuilder) -> FifoBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for FifoBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl FifoBuilder {
    pub fn fifo(&self) -> u8 {
        self.value.get_bits(0usize..=7usize) as u8
    }
    pub fn with_fifo(mut self, value: u8) -> Self {
        self.value.set_bits(0usize..=7usize, value as u32);
        self
    }
}
