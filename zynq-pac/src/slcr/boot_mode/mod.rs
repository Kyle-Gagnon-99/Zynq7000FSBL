use crate::*;
pub struct BootModeRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct BootModeBuilder {
    value: u32,
}
impl FromBits<u32> for BootModeBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<BootModeBuilder, u32> for BootModeRegister {
    fn read(&self) -> BootModeBuilder {
        BootModeBuilder::from_bits(self.inner.read())
    }
}
impl Default for BootModeBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(0usize..=3usize, BootMode::Jtag as u32);
        Self { value }
    }
}
impl BootModeBuilder {
    pub fn boot_mode(&self) -> Option<BootMode> {
        BootMode::try_from(self.value.get_bits(0usize..=3usize)).ok()
    }
    pub fn with_boot_mode(mut self, value: BootMode) -> Self {
        self.value.set_bits(0usize..=3usize, Into::<u32>::into(value));
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootMode {
    Jtag = 0usize as u8,
    Nor = 2usize as u8,
    Nand = 4usize as u8,
    QuadSpi = 1usize as u8,
    Sd = 5usize as u8,
}
impl Default for BootMode {
    fn default() -> Self {
        Self::Jtag
    }
}
impl From<BootMode> for u8 {
    fn from(value: BootMode) -> Self {
        value as u8
    }
}
impl From<BootMode> for u32 {
    fn from(value: BootMode) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for BootMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(BootMode::Jtag),
            2usize => Ok(BootMode::Nor),
            4usize => Ok(BootMode::Nand),
            1usize => Ok(BootMode::QuadSpi),
            5usize => Ok(BootMode::Sd),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for BootMode {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(BootMode::Jtag),
            2usize => Ok(BootMode::Nor),
            4usize => Ok(BootMode::Nand),
            1usize => Ok(BootMode::QuadSpi),
            5usize => Ok(BootMode::Sd),
            _ => Err(()),
        }
    }
}
