use crate::*;
pub struct Reg2IntMaskRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg2IntMaskBuilder {
    value: u32,
}
impl FromBits<u32> for Reg2IntMaskBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg2IntMaskBuilder, u32> for Reg2IntMaskRegister {
    fn read(&self) -> Reg2IntMaskBuilder {
        Reg2IntMaskBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg2IntMaskBuilder, u32> for Reg2IntMaskRegister {
    fn zeroed() -> Reg2IntMaskBuilder {
        Reg2IntMaskBuilder::default()
    }
    fn write(&mut self, value: Reg2IntMaskBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg2IntMaskBuilder, u32> for Reg2IntMaskRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg2IntMaskBuilder) -> Reg2IntMaskBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg2IntMaskBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg2IntMaskBuilder {
    pub fn ecntr(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_ecntr(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn parrt(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_parrt(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn parrd(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn with_parrd(mut self, value: bool) -> Self {
        self.value.set_bit(2usize, value);
        self
    }
    pub fn errwt(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_errwt(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
    pub fn errwd(&self) -> bool {
        self.value.get_bit(4usize)
    }
    pub fn with_errwd(mut self, value: bool) -> Self {
        self.value.set_bit(4usize, value);
        self
    }
    pub fn errrt(&self) -> bool {
        self.value.get_bit(5usize)
    }
    pub fn with_errrt(mut self, value: bool) -> Self {
        self.value.set_bit(5usize, value);
        self
    }
    pub fn errrd(&self) -> bool {
        self.value.get_bit(6usize)
    }
    pub fn with_errrd(mut self, value: bool) -> Self {
        self.value.set_bit(6usize, value);
        self
    }
    pub fn slverr(&self) -> bool {
        self.value.get_bit(7usize)
    }
    pub fn with_slverr(mut self, value: bool) -> Self {
        self.value.set_bit(7usize, value);
        self
    }
    pub fn decerr(&self) -> bool {
        self.value.get_bit(8usize)
    }
    pub fn with_decerr(mut self, value: bool) -> Self {
        self.value.set_bit(8usize, value);
        self
    }
}
