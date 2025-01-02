use crate::*;
pub struct AperClkCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct AperClkCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for AperClkCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<AperClkCtrlBuilder, u32> for AperClkCtrlRegister {
    fn read(&self) -> AperClkCtrlBuilder {
        AperClkCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<AperClkCtrlBuilder, u32> for AperClkCtrlRegister {
    fn zeroed() -> AperClkCtrlBuilder {
        AperClkCtrlBuilder::default()
    }
    fn write(&mut self, value: AperClkCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<AperClkCtrlBuilder, u32> for AperClkCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(AperClkCtrlBuilder) -> AperClkCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for AperClkCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(0usize, false);
        value.set_bit(2usize, true);
        value.set_bit(3usize, true);
        value.set_bit(6usize, true);
        value.set_bit(7usize, true);
        value.set_bit(10usize, true);
        value.set_bit(11usize, true);
        value.set_bit(14usize, true);
        value.set_bit(15usize, true);
        value.set_bit(16usize, true);
        value.set_bit(17usize, true);
        value.set_bit(18usize, true);
        value.set_bit(19usize, true);
        value.set_bit(20usize, true);
        value.set_bit(21usize, true);
        value.set_bit(22usize, true);
        value.set_bit(23usize, true);
        value.set_bit(24usize, true);
        Self { value }
    }
}
impl AperClkCtrlBuilder {
    pub fn dma_cpu_2_xclkact(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_dma_cpu_2_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn usb0_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn with_usb0_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(2usize, value);
        self
    }
    pub fn usb1_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_usb1_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
    pub fn gem0_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(6usize)
    }
    pub fn with_gem0_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(6usize, value);
        self
    }
    pub fn gem1_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(7usize)
    }
    pub fn with_gem1_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(7usize, value);
        self
    }
    pub fn sdio0_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(10usize)
    }
    pub fn with_sdio0_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(10usize, value);
        self
    }
    pub fn sdio1_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(11usize)
    }
    pub fn with_sdio1_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(11usize, value);
        self
    }
    pub fn spi0_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(14usize)
    }
    pub fn with_spi0_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(14usize, value);
        self
    }
    pub fn spi1_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(15usize)
    }
    pub fn with_spi1_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(15usize, value);
        self
    }
    pub fn can0_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(16usize)
    }
    pub fn with_can0_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(16usize, value);
        self
    }
    pub fn can1_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(17usize)
    }
    pub fn with_can1_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(17usize, value);
        self
    }
    pub fn i2_c0_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(18usize)
    }
    pub fn with_i2_c0_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(18usize, value);
        self
    }
    pub fn i2_c1_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(19usize)
    }
    pub fn with_i2_c1_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(19usize, value);
        self
    }
    pub fn uart0_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(20usize)
    }
    pub fn with_uart0_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(20usize, value);
        self
    }
    pub fn uart1_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(21usize)
    }
    pub fn with_uart1_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(21usize, value);
        self
    }
    pub fn gpio_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(22usize)
    }
    pub fn with_gpio_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(22usize, value);
        self
    }
    pub fn lqspi_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(23usize)
    }
    pub fn with_lqspi_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(23usize, value);
        self
    }
    pub fn smc_cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(24usize)
    }
    pub fn with_smc_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(24usize, value);
        self
    }
}
