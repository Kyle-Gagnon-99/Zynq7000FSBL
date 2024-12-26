use crate::*;
pub struct BaudDivRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct BaudDivBuilder {
    value: u32,
}
impl FromBits<u32> for BaudDivBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<BaudDivBuilder, u32> for BaudDivRegister {
    fn read(&self) -> BaudDivBuilder {
        BaudDivBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<BaudDivBuilder, u32> for BaudDivRegister {
    fn zeroed() -> BaudDivBuilder {
        BaudDivBuilder::default()
    }
    fn write(&mut self, value: BaudDivBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<BaudDivBuilder, u32> for BaudDivRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(BaudDivBuilder) -> BaudDivBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for BaudDivBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl BaudDivBuilder {
    pub fn bdiv(&self) -> u8 {
        self.value.get_bits(0usize..=7usize) as u8
    }
    pub fn with_bdiv(mut self, value: u8) -> Self {
        self.value.set_bits(0usize..=7usize, value as u32);
        self
    }
}
