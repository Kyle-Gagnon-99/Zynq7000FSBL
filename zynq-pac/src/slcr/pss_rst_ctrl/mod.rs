use crate::*;
pub struct PssRstCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct PssRstCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for PssRstCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<PssRstCtrlBuilder, u32> for PssRstCtrlRegister {
    fn read(&self) -> PssRstCtrlBuilder {
        PssRstCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<PssRstCtrlBuilder, u32> for PssRstCtrlRegister {
    fn zeroed() -> PssRstCtrlBuilder {
        PssRstCtrlBuilder::default()
    }
    fn write(&mut self, value: PssRstCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<PssRstCtrlBuilder, u32> for PssRstCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(PssRstCtrlBuilder) -> PssRstCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for PssRstCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl PssRstCtrlBuilder {
    pub fn soft_rst(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_soft_rst(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
}
