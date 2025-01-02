use crate::*;
pub struct Reg1ControlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg1ControlBuilder {
    value: u32,
}
impl FromBits<u32> for Reg1ControlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg1ControlBuilder, u32> for Reg1ControlRegister {
    fn read(&self) -> Reg1ControlBuilder {
        Reg1ControlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg1ControlBuilder, u32> for Reg1ControlRegister {
    fn zeroed() -> Reg1ControlBuilder {
        Reg1ControlBuilder::default()
    }
    fn write(&mut self, value: Reg1ControlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg1ControlBuilder, u32> for Reg1ControlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg1ControlBuilder) -> Reg1ControlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg1ControlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg1ControlBuilder {
    pub fn l2_enable(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_l2_enable(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
}
