use crate::*;
pub struct Reg1TagRamControlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg1TagRamControlBuilder {
    value: u32,
}
impl FromBits<u32> for Reg1TagRamControlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg1TagRamControlBuilder, u32> for Reg1TagRamControlRegister {
    fn read(&self) -> Reg1TagRamControlBuilder {
        Reg1TagRamControlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg1TagRamControlBuilder, u32> for Reg1TagRamControlRegister {
    fn zeroed() -> Reg1TagRamControlBuilder {
        Reg1TagRamControlBuilder::default()
    }
    fn write(&mut self, value: Reg1TagRamControlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg1TagRamControlBuilder, u32> for Reg1TagRamControlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg1TagRamControlBuilder) -> Reg1TagRamControlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg1TagRamControlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(0usize..=2usize, 7usize as u32);
        value.set_bits(4usize..=6usize, 7usize as u32);
        value.set_bits(8usize..=10usize, 7usize as u32);
        Self { value }
    }
}
impl Reg1TagRamControlBuilder {
    pub fn ram_setup_lat(&self) -> Option<RamSetupLatency> {
        RamSetupLatency::try_from(self.value.get_bits(0usize..=2usize)).ok()
    }
    pub fn with_ram_setup_lat(mut self, value: RamSetupLatency) -> Self {
        self.value.set_bits(0usize..=2usize, Into::<u32>::into(value));
        self
    }
    pub fn ram_rd_access_lat(&self) -> Option<RamReadAccessLatency> {
        RamReadAccessLatency::try_from(self.value.get_bits(4usize..=6usize)).ok()
    }
    pub fn with_ram_rd_access_lat(mut self, value: RamReadAccessLatency) -> Self {
        self.value.set_bits(4usize..=6usize, Into::<u32>::into(value));
        self
    }
    pub fn ram_wr_access_lat(&self) -> Option<RamWriteAccessLatency> {
        RamWriteAccessLatency::try_from(self.value.get_bits(8usize..=10usize)).ok()
    }
    pub fn with_ram_wr_access_lat(mut self, value: RamWriteAccessLatency) -> Self {
        self.value.set_bits(8usize..=10usize, Into::<u32>::into(value));
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RamSetupLatency {
    OneCycle = 0usize as u8,
    TwoCycles = 1usize as u8,
    ThreeCycles = 2usize as u8,
    FourCycles = 3usize as u8,
    FiveCycles = 4usize as u8,
    SixCycles = 5usize as u8,
    SevenCycles = 6usize as u8,
    EightCycles = 7usize as u8,
}
impl Default for RamSetupLatency {
    fn default() -> Self {
        Self::EightCycles
    }
}
impl From<RamSetupLatency> for u8 {
    fn from(value: RamSetupLatency) -> Self {
        value as u8
    }
}
impl From<RamSetupLatency> for u32 {
    fn from(value: RamSetupLatency) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for RamSetupLatency {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(RamSetupLatency::OneCycle),
            1usize => Ok(RamSetupLatency::TwoCycles),
            2usize => Ok(RamSetupLatency::ThreeCycles),
            3usize => Ok(RamSetupLatency::FourCycles),
            4usize => Ok(RamSetupLatency::FiveCycles),
            5usize => Ok(RamSetupLatency::SixCycles),
            6usize => Ok(RamSetupLatency::SevenCycles),
            7usize => Ok(RamSetupLatency::EightCycles),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for RamSetupLatency {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(RamSetupLatency::OneCycle),
            1usize => Ok(RamSetupLatency::TwoCycles),
            2usize => Ok(RamSetupLatency::ThreeCycles),
            3usize => Ok(RamSetupLatency::FourCycles),
            4usize => Ok(RamSetupLatency::FiveCycles),
            5usize => Ok(RamSetupLatency::SixCycles),
            6usize => Ok(RamSetupLatency::SevenCycles),
            7usize => Ok(RamSetupLatency::EightCycles),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RamReadAccessLatency {
    OneCycle = 0usize as u8,
    TwoCycles = 1usize as u8,
    ThreeCycles = 2usize as u8,
    FourCycles = 3usize as u8,
    FiveCycles = 4usize as u8,
    SixCycles = 5usize as u8,
    SevenCycles = 6usize as u8,
    EightCycles = 7usize as u8,
}
impl Default for RamReadAccessLatency {
    fn default() -> Self {
        Self::EightCycles
    }
}
impl From<RamReadAccessLatency> for u8 {
    fn from(value: RamReadAccessLatency) -> Self {
        value as u8
    }
}
impl From<RamReadAccessLatency> for u32 {
    fn from(value: RamReadAccessLatency) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for RamReadAccessLatency {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(RamReadAccessLatency::OneCycle),
            1usize => Ok(RamReadAccessLatency::TwoCycles),
            2usize => Ok(RamReadAccessLatency::ThreeCycles),
            3usize => Ok(RamReadAccessLatency::FourCycles),
            4usize => Ok(RamReadAccessLatency::FiveCycles),
            5usize => Ok(RamReadAccessLatency::SixCycles),
            6usize => Ok(RamReadAccessLatency::SevenCycles),
            7usize => Ok(RamReadAccessLatency::EightCycles),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for RamReadAccessLatency {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(RamReadAccessLatency::OneCycle),
            1usize => Ok(RamReadAccessLatency::TwoCycles),
            2usize => Ok(RamReadAccessLatency::ThreeCycles),
            3usize => Ok(RamReadAccessLatency::FourCycles),
            4usize => Ok(RamReadAccessLatency::FiveCycles),
            5usize => Ok(RamReadAccessLatency::SixCycles),
            6usize => Ok(RamReadAccessLatency::SevenCycles),
            7usize => Ok(RamReadAccessLatency::EightCycles),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RamWriteAccessLatency {
    OneCycle = 0usize as u8,
    TwoCycles = 1usize as u8,
    ThreeCycles = 2usize as u8,
    FourCycles = 3usize as u8,
    FiveCycles = 4usize as u8,
    SixCycles = 5usize as u8,
    SevenCycles = 6usize as u8,
    EightCycles = 7usize as u8,
}
impl Default for RamWriteAccessLatency {
    fn default() -> Self {
        Self::EightCycles
    }
}
impl From<RamWriteAccessLatency> for u8 {
    fn from(value: RamWriteAccessLatency) -> Self {
        value as u8
    }
}
impl From<RamWriteAccessLatency> for u32 {
    fn from(value: RamWriteAccessLatency) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for RamWriteAccessLatency {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(RamWriteAccessLatency::OneCycle),
            1usize => Ok(RamWriteAccessLatency::TwoCycles),
            2usize => Ok(RamWriteAccessLatency::ThreeCycles),
            3usize => Ok(RamWriteAccessLatency::FourCycles),
            4usize => Ok(RamWriteAccessLatency::FiveCycles),
            5usize => Ok(RamWriteAccessLatency::SixCycles),
            6usize => Ok(RamWriteAccessLatency::SevenCycles),
            7usize => Ok(RamWriteAccessLatency::EightCycles),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for RamWriteAccessLatency {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(RamWriteAccessLatency::OneCycle),
            1usize => Ok(RamWriteAccessLatency::TwoCycles),
            2usize => Ok(RamWriteAccessLatency::ThreeCycles),
            3usize => Ok(RamWriteAccessLatency::FourCycles),
            4usize => Ok(RamWriteAccessLatency::FiveCycles),
            5usize => Ok(RamWriteAccessLatency::SixCycles),
            6usize => Ok(RamWriteAccessLatency::SevenCycles),
            7usize => Ok(RamWriteAccessLatency::EightCycles),
            _ => Err(()),
        }
    }
}
