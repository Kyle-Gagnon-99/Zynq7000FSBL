use crate::*;
pub struct UartClkCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct UartClkCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for UartClkCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<UartClkCtrlBuilder, u32> for UartClkCtrlRegister {
    fn read(&self) -> UartClkCtrlBuilder {
        UartClkCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<UartClkCtrlBuilder, u32> for UartClkCtrlRegister {
    fn zeroed() -> UartClkCtrlBuilder {
        UartClkCtrlBuilder::default()
    }
    fn write(&mut self, value: UartClkCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<UartClkCtrlBuilder, u32> for UartClkCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(UartClkCtrlBuilder) -> UartClkCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for UartClkCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(0usize, false);
        value.set_bit(1usize, true);
        value.set_bits(4usize..=5usize, UartClockSource::ArmPll as u32);
        value.set_bits(8usize..=13usize, 63usize as u32);
        Self { value }
    }
}
impl UartClkCtrlBuilder {
    pub fn clkact0(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_clkact0(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn clkact1(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_clkact1(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn srcsel(&self) -> Option<UartClockSource> {
        UartClockSource::try_from(self.value.get_bits(4usize..=5usize)).ok()
    }
    pub fn with_srcsel(mut self, value: UartClockSource) -> Self {
        self.value.set_bits(4usize..=5usize, Into::<u32>::into(value));
        self
    }
    pub fn divisor(&self) -> u8 {
        self.value.get_bits(8usize..=13usize) as u8
    }
    pub fn with_divisor(mut self, value: u8) -> Self {
        self.value.set_bits(8usize..=13usize, value as u32);
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UartClockSource {
    ArmPll = 0usize as u8,
    DdrPll = 2usize as u8,
    IoPll = 3usize as u8,
}
impl Default for UartClockSource {
    fn default() -> Self {
        Self::ArmPll
    }
}
impl From<UartClockSource> for u8 {
    fn from(value: UartClockSource) -> Self {
        value as u8
    }
}
impl From<UartClockSource> for u32 {
    fn from(value: UartClockSource) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for UartClockSource {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(UartClockSource::ArmPll),
            2usize => Ok(UartClockSource::DdrPll),
            3usize => Ok(UartClockSource::IoPll),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for UartClockSource {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(UartClockSource::ArmPll),
            2usize => Ok(UartClockSource::DdrPll),
            3usize => Ok(UartClockSource::IoPll),
            _ => Err(()),
        }
    }
}
