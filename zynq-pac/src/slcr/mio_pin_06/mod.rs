use crate::*;
pub struct MioPin06Register {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct MioPin06Builder {
    value: u32,
}
impl FromBits<u32> for MioPin06Builder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<MioPin06Builder, u32> for MioPin06Register {
    fn read(&self) -> MioPin06Builder {
        MioPin06Builder::from_bits(self.inner.read())
    }
}
impl RegisterWO<MioPin06Builder, u32> for MioPin06Register {
    fn zeroed() -> MioPin06Builder {
        MioPin06Builder::default()
    }
    fn write(&mut self, value: MioPin06Builder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<MioPin06Builder, u32> for MioPin06Register {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(MioPin06Builder) -> MioPin06Builder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for MioPin06Builder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(0usize, false);
        value.set_bits(9usize..=11usize, IoType::Lvcmos33 as u32);
        value.set_bit(12usize, true);
        Self { value }
    }
}
impl MioPin06Builder {
    pub fn tri_enable(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_tri_enable(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn l0_sel(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_l0_sel(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn l1_sel(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn with_l1_sel(mut self, value: bool) -> Self {
        self.value.set_bit(2usize, value);
        self
    }
    pub fn l2_sel(&self) -> u8 {
        self.value.get_bits(3usize..=4usize) as u8
    }
    pub fn with_l2_sel(mut self, value: u8) -> Self {
        self.value.set_bits(3usize..=4usize, value as u32);
        self
    }
    pub fn l3_sel(&self) -> u8 {
        self.value.get_bits(5usize..=7usize) as u8
    }
    pub fn with_l3_sel(mut self, value: u8) -> Self {
        self.value.set_bits(5usize..=7usize, value as u32);
        self
    }
    pub fn speed(&self) -> bool {
        self.value.get_bit(8usize)
    }
    pub fn with_speed(mut self, value: bool) -> Self {
        self.value.set_bit(8usize, value);
        self
    }
    pub fn io_type(&self) -> Option<IoType> {
        IoType::try_from(self.value.get_bits(9usize..=11usize)).ok()
    }
    pub fn with_io_type(mut self, value: IoType) -> Self {
        self.value.set_bits(9usize..=11usize, Into::<u32>::into(value));
        self
    }
    pub fn pullup(&self) -> bool {
        self.value.get_bit(12usize)
    }
    pub fn with_pullup(mut self, value: bool) -> Self {
        self.value.set_bit(12usize, value);
        self
    }
    pub fn disable_rcvr(&self) -> bool {
        self.value.get_bit(13usize)
    }
    pub fn with_disable_rcvr(mut self, value: bool) -> Self {
        self.value.set_bit(13usize, value);
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoType {
    Lvcmos18 = 1usize as u8,
    Lvcmos25 = 2usize as u8,
    Lvcmos33 = 3usize as u8,
    Hstl = 4usize as u8,
}
impl Default for IoType {
    fn default() -> Self {
        Self::Lvcmos33
    }
}
impl From<IoType> for u8 {
    fn from(value: IoType) -> Self {
        value as u8
    }
}
impl From<IoType> for u32 {
    fn from(value: IoType) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for IoType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            1usize => Ok(IoType::Lvcmos18),
            2usize => Ok(IoType::Lvcmos25),
            3usize => Ok(IoType::Lvcmos33),
            4usize => Ok(IoType::Hstl),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for IoType {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            1usize => Ok(IoType::Lvcmos18),
            2usize => Ok(IoType::Lvcmos25),
            3usize => Ok(IoType::Lvcmos33),
            4usize => Ok(IoType::Hstl),
            _ => Err(()),
        }
    }
}
