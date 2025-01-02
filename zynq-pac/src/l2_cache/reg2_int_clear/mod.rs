use crate::*;
pub struct Reg2IntClearRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg2IntClearBuilder {
    value: u32,
}
impl FromBits<u32> for Reg2IntClearBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg2IntClearBuilder, u32> for Reg2IntClearRegister {
    fn read(&self) -> Reg2IntClearBuilder {
        Reg2IntClearBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg2IntClearBuilder, u32> for Reg2IntClearRegister {
    fn zeroed() -> Reg2IntClearBuilder {
        Reg2IntClearBuilder::default()
    }
    fn write(&mut self, value: Reg2IntClearBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg2IntClearBuilder, u32> for Reg2IntClearRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg2IntClearBuilder) -> Reg2IntClearBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg2IntClearBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg2IntClearBuilder {
    pub fn with_ecntr(mut self) -> Self {
        self.value.set_bit(0usize, true);
        self
    }
    pub fn with_parrt(mut self) -> Self {
        self.value.set_bit(1usize, true);
        self
    }
    pub fn with_parrd(mut self) -> Self {
        self.value.set_bit(2usize, true);
        self
    }
    pub fn with_errwt(mut self) -> Self {
        self.value.set_bit(3usize, true);
        self
    }
    pub fn with_errwd(mut self) -> Self {
        self.value.set_bit(4usize, true);
        self
    }
    pub fn with_errrt(mut self) -> Self {
        self.value.set_bit(5usize, true);
        self
    }
    pub fn with_errrd(mut self) -> Self {
        self.value.set_bit(6usize, true);
        self
    }
    pub fn with_slverr(mut self) -> Self {
        self.value.set_bit(7usize, true);
        self
    }
    pub fn with_decerr(mut self) -> Self {
        self.value.set_bit(8usize, true);
        self
    }
}
