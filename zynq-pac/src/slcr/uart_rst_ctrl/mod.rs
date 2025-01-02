use crate::*;
pub struct UartRstCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct UartRstCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for UartRstCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<UartRstCtrlBuilder, u32> for UartRstCtrlRegister {
    fn read(&self) -> UartRstCtrlBuilder {
        UartRstCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<UartRstCtrlBuilder, u32> for UartRstCtrlRegister {
    fn zeroed() -> UartRstCtrlBuilder {
        UartRstCtrlBuilder::default()
    }
    fn write(&mut self, value: UartRstCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<UartRstCtrlBuilder, u32> for UartRstCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(UartRstCtrlBuilder) -> UartRstCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for UartRstCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl UartRstCtrlBuilder {
    pub fn uart0_cpu1_x_rst(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_uart0_cpu1_x_rst(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn uart1_cpu1_x_rst(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_uart1_cpu1_x_rst(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn uart0_ref_rst(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn with_uart0_ref_rst(mut self, value: bool) -> Self {
        self.value.set_bit(2usize, value);
        self
    }
    pub fn uart1_ref_rst(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_uart1_ref_rst(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
}
