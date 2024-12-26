use crate::*;
pub struct BaudGenRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct BaudGenBuilder {
    value: u32,
}
impl FromBits<u32> for BaudGenBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<BaudGenBuilder, u32> for BaudGenRegister {
    fn read(&self) -> BaudGenBuilder {
        BaudGenBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<BaudGenBuilder, u32> for BaudGenRegister {
    fn zeroed() -> BaudGenBuilder {
        BaudGenBuilder::default()
    }
    fn write(&mut self, value: BaudGenBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<BaudGenBuilder, u32> for BaudGenRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(BaudGenBuilder) -> BaudGenBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for BaudGenBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl BaudGenBuilder {
    pub fn cd(&self) -> u16 {
        self.value.get_bits(0usize..=15usize) as u16
    }
    pub fn with_cd(mut self, value: u16) -> Self {
        self.value.set_bits(0usize..=15usize, value as u32);
        self
    }
}
