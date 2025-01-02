use crate::*;
pub struct SclRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct SclBuilder {
    value: u32,
}
impl FromBits<u32> for SclBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<SclBuilder, u32> for SclRegister {
    fn read(&self) -> SclBuilder {
        SclBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<SclBuilder, u32> for SclRegister {
    fn zeroed() -> SclBuilder {
        SclBuilder::default()
    }
    fn write(&mut self, value: SclBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<SclBuilder, u32> for SclRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(SclBuilder) -> SclBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for SclBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl SclBuilder {
    pub fn lock(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_lock(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
}
