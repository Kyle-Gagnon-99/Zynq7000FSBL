use crate::*;
pub struct FpgaRstCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct FpgaRstCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for FpgaRstCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<FpgaRstCtrlBuilder, u32> for FpgaRstCtrlRegister {
    fn read(&self) -> FpgaRstCtrlBuilder {
        FpgaRstCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<FpgaRstCtrlBuilder, u32> for FpgaRstCtrlRegister {
    fn zeroed() -> FpgaRstCtrlBuilder {
        FpgaRstCtrlBuilder::default()
    }
    fn write(&mut self, value: FpgaRstCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<FpgaRstCtrlBuilder, u32> for FpgaRstCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(FpgaRstCtrlBuilder) -> FpgaRstCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for FpgaRstCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl FpgaRstCtrlBuilder {
    pub fn fpga0_out_rst(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_fpga0_out_rst(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn fpga1_out_rst(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_fpga1_out_rst(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn fpga2_out_rst(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn with_fpga2_out_rst(mut self, value: bool) -> Self {
        self.value.set_bit(2usize, value);
        self
    }
    pub fn fpga3_out_rst(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_fpga3_out_rst(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
}
