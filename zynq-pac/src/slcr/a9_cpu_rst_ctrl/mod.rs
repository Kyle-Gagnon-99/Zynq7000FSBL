use crate::*;
pub struct A9CpuRstCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct A9CpuRstCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for A9CpuRstCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<A9CpuRstCtrlBuilder, u32> for A9CpuRstCtrlRegister {
    fn read(&self) -> A9CpuRstCtrlBuilder {
        A9CpuRstCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<A9CpuRstCtrlBuilder, u32> for A9CpuRstCtrlRegister {
    fn zeroed() -> A9CpuRstCtrlBuilder {
        A9CpuRstCtrlBuilder::default()
    }
    fn write(&mut self, value: A9CpuRstCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<A9CpuRstCtrlBuilder, u32> for A9CpuRstCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(A9CpuRstCtrlBuilder) -> A9CpuRstCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for A9CpuRstCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl A9CpuRstCtrlBuilder {
    pub fn a9_rst0(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_a9_rst0(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn a9_rst1(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_a9_rst1(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn a9_clkstop0(&self) -> bool {
        self.value.get_bit(4usize)
    }
    pub fn with_a9_clkstop0(mut self, value: bool) -> Self {
        self.value.set_bit(4usize, value);
        self
    }
    pub fn a9_clkstop1(&self) -> bool {
        self.value.get_bit(5usize)
    }
    pub fn with_a9_clkstop1(mut self, value: bool) -> Self {
        self.value.set_bit(5usize, value);
        self
    }
    pub fn peri_rst(&self) -> bool {
        self.value.get_bit(8usize)
    }
    pub fn with_peri_rst(mut self, value: bool) -> Self {
        self.value.set_bit(8usize, value);
        self
    }
}
