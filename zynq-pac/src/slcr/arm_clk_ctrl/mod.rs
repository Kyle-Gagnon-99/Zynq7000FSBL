use crate::*;
pub struct ArmClkCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct ArmClkCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for ArmClkCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<ArmClkCtrlBuilder, u32> for ArmClkCtrlRegister {
    fn read(&self) -> ArmClkCtrlBuilder {
        ArmClkCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<ArmClkCtrlBuilder, u32> for ArmClkCtrlRegister {
    fn zeroed() -> ArmClkCtrlBuilder {
        ArmClkCtrlBuilder::default()
    }
    fn write(&mut self, value: ArmClkCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<ArmClkCtrlBuilder, u32> for ArmClkCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(ArmClkCtrlBuilder) -> ArmClkCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for ArmClkCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(4usize..=5usize, ArmPllClockSource::ArmPll as u32);
        value.set_bits(8usize..=13usize, 4usize as u32);
        value.set_bit(24usize, true);
        value.set_bit(25usize, true);
        value.set_bit(26usize, true);
        value.set_bit(27usize, true);
        value.set_bit(28usize, true);
        Self { value }
    }
}
impl ArmClkCtrlBuilder {
    pub fn srcsel(&self) -> Option<ArmPllClockSource> {
        ArmPllClockSource::try_from(self.value.get_bits(4usize..=5usize)).ok()
    }
    pub fn with_srcsel(mut self, value: ArmPllClockSource) -> Self {
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
    pub fn cpu_6_or4_xclkact(&self) -> bool {
        self.value.get_bit(24usize)
    }
    pub fn with_cpu_6_or4_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(24usize, value);
        self
    }
    pub fn cpu_3_or2_xclkact(&self) -> bool {
        self.value.get_bit(25usize)
    }
    pub fn with_cpu_3_or2_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(25usize, value);
        self
    }
    pub fn cpu_2_xclkact(&self) -> bool {
        self.value.get_bit(26usize)
    }
    pub fn with_cpu_2_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(26usize, value);
        self
    }
    pub fn cpu_1_xclkact(&self) -> bool {
        self.value.get_bit(27usize)
    }
    pub fn with_cpu_1_xclkact(mut self, value: bool) -> Self {
        self.value.set_bit(27usize, value);
        self
    }
    pub fn cpu_peri_clkact(&self) -> bool {
        self.value.get_bit(28usize)
    }
    pub fn with_cpu_peri_clkact(mut self, value: bool) -> Self {
        self.value.set_bit(28usize, value);
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmPllClockSource {
    ArmPll = 0usize as u8,
    DdrPll = 2usize as u8,
    IoPll = 3usize as u8,
}
impl Default for ArmPllClockSource {
    fn default() -> Self {
        Self::ArmPll
    }
}
impl From<ArmPllClockSource> for u8 {
    fn from(value: ArmPllClockSource) -> Self {
        value as u8
    }
}
impl From<ArmPllClockSource> for u32 {
    fn from(value: ArmPllClockSource) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for ArmPllClockSource {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ArmPllClockSource::ArmPll),
            2usize => Ok(ArmPllClockSource::DdrPll),
            3usize => Ok(ArmPllClockSource::IoPll),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for ArmPllClockSource {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ArmPllClockSource::ArmPll),
            2usize => Ok(ArmPllClockSource::DdrPll),
            3usize => Ok(ArmPllClockSource::IoPll),
            _ => Err(()),
        }
    }
}
