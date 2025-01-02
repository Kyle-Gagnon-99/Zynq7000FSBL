use crate::*;
pub struct Fpga2ClkCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Fpga2ClkCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for Fpga2ClkCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Fpga2ClkCtrlBuilder, u32> for Fpga2ClkCtrlRegister {
    fn read(&self) -> Fpga2ClkCtrlBuilder {
        Fpga2ClkCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Fpga2ClkCtrlBuilder, u32> for Fpga2ClkCtrlRegister {
    fn zeroed() -> Fpga2ClkCtrlBuilder {
        Fpga2ClkCtrlBuilder::default()
    }
    fn write(&mut self, value: Fpga2ClkCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Fpga2ClkCtrlBuilder, u32> for Fpga2ClkCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Fpga2ClkCtrlBuilder) -> Fpga2ClkCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Fpga2ClkCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(4usize..=5usize, FpgaClockSource::ArmPll as u32);
        value.set_bits(8usize..=13usize, 24usize as u32);
        value.set_bits(20usize..=25usize, 1usize as u32);
        Self { value }
    }
}
impl Fpga2ClkCtrlBuilder {
    pub fn srcsel(&self) -> Option<FpgaClockSource> {
        FpgaClockSource::try_from(self.value.get_bits(4usize..=5usize)).ok()
    }
    pub fn with_srcsel(mut self, value: FpgaClockSource) -> Self {
        self.value.set_bits(4usize..=5usize, Into::<u32>::into(value));
        self
    }
    pub fn divisor0(&self) -> u8 {
        self.value.get_bits(8usize..=13usize) as u8
    }
    pub fn with_divisor0(mut self, value: u8) -> Self {
        self.value.set_bits(8usize..=13usize, value as u32);
        self
    }
    pub fn divisor1(&self) -> u8 {
        self.value.get_bits(20usize..=25usize) as u8
    }
    pub fn with_divisor1(mut self, value: u8) -> Self {
        self.value.set_bits(20usize..=25usize, value as u32);
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpgaClockSource {
    ArmPll = 0usize as u8,
    DdrPll = 2usize as u8,
    IoPll = 3usize as u8,
}
impl Default for FpgaClockSource {
    fn default() -> Self {
        Self::ArmPll
    }
}
impl From<FpgaClockSource> for u8 {
    fn from(value: FpgaClockSource) -> Self {
        value as u8
    }
}
impl From<FpgaClockSource> for u32 {
    fn from(value: FpgaClockSource) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for FpgaClockSource {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(FpgaClockSource::ArmPll),
            2usize => Ok(FpgaClockSource::DdrPll),
            3usize => Ok(FpgaClockSource::IoPll),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for FpgaClockSource {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(FpgaClockSource::ArmPll),
            2usize => Ok(FpgaClockSource::DdrPll),
            3usize => Ok(FpgaClockSource::IoPll),
            _ => Err(()),
        }
    }
}
