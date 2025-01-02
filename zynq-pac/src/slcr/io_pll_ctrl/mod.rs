use crate::*;
pub struct IoPllCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct IoPllCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for IoPllCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<IoPllCtrlBuilder, u32> for IoPllCtrlRegister {
    fn read(&self) -> IoPllCtrlBuilder {
        IoPllCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<IoPllCtrlBuilder, u32> for IoPllCtrlRegister {
    fn zeroed() -> IoPllCtrlBuilder {
        IoPllCtrlBuilder::default()
    }
    fn write(&mut self, value: IoPllCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<IoPllCtrlBuilder, u32> for IoPllCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(IoPllCtrlBuilder) -> IoPllCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for IoPllCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(3usize, true);
        value.set_bits(12usize..=18usize, 26usize as u32);
        Self { value }
    }
}
impl IoPllCtrlBuilder {
    pub fn pll_reset(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_pll_reset(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn pll_pwrdwn(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_pll_pwrdwn(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn pll_bypass_qual(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_pll_bypass_qual(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
    pub fn pll_bypass_force(&self) -> bool {
        self.value.get_bit(4usize)
    }
    pub fn with_pll_bypass_force(mut self, value: bool) -> Self {
        self.value.set_bit(4usize, value);
        self
    }
    pub fn pll_fdiv(&self) -> u8 {
        self.value.get_bits(12usize..=18usize) as u8
    }
    pub fn with_pll_fdiv(mut self, value: u8) -> Self {
        self.value.set_bits(12usize..=18usize, value as u32);
        self
    }
}
