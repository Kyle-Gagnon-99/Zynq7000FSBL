use crate::*;
pub struct Reg1AuxControlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg1AuxControlBuilder {
    value: u32,
}
impl FromBits<u32> for Reg1AuxControlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg1AuxControlBuilder, u32> for Reg1AuxControlRegister {
    fn read(&self) -> Reg1AuxControlBuilder {
        Reg1AuxControlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg1AuxControlBuilder, u32> for Reg1AuxControlRegister {
    fn zeroed() -> Reg1AuxControlBuilder {
        Reg1AuxControlBuilder::default()
    }
    fn write(&mut self, value: Reg1AuxControlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg1AuxControlBuilder, u32> for Reg1AuxControlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg1AuxControlBuilder) -> Reg1AuxControlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg1AuxControlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(17usize..=19usize, 3usize as u32);
        value.set_bits(23usize..=24usize, 0usize as u32);
        value.set_bit(25usize, true);
        Self { value }
    }
}
impl Reg1AuxControlBuilder {
    pub fn full_line_zero_enable(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_full_line_zero_enable(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn hig_pr_so_dev_rd_en(&self) -> bool {
        self.value.get_bit(10usize)
    }
    pub fn with_hig_pr_so_dev_rd_en(mut self, value: bool) -> Self {
        self.value.set_bit(10usize, value);
        self
    }
    pub fn store_buff_dev_lim_en(&self) -> bool {
        self.value.get_bit(11usize)
    }
    pub fn with_store_buff_dev_lim_en(mut self, value: bool) -> Self {
        self.value.set_bit(11usize, value);
        self
    }
    pub fn ex_cache_config(&self) -> bool {
        self.value.get_bit(12usize)
    }
    pub fn with_ex_cache_config(mut self, value: bool) -> Self {
        self.value.set_bit(12usize, value);
        self
    }
    pub fn shared_attr_inva_en(&self) -> bool {
        self.value.get_bit(13usize)
    }
    pub fn with_shared_attr_inva_en(mut self, value: bool) -> Self {
        self.value.set_bit(13usize, value);
        self
    }
    pub fn associativity(&self) -> bool {
        self.value.get_bit(16usize)
    }
    pub fn with_associativity(mut self, value: bool) -> Self {
        self.value.set_bit(16usize, value);
        self
    }
    pub fn way_size(&self) -> Option<WaySize> {
        WaySize::try_from(self.value.get_bits(17usize..=19usize)).ok()
    }
    pub fn with_way_size(mut self, value: WaySize) -> Self {
        self.value.set_bits(17usize..=19usize, Into::<u32>::into(value));
        self
    }
    pub fn event_mon_bus_en(&self) -> bool {
        self.value.get_bit(20usize)
    }
    pub fn with_event_mon_bus_en(mut self, value: bool) -> Self {
        self.value.set_bit(20usize, value);
        self
    }
    pub fn parity_en(&self) -> bool {
        self.value.get_bit(21usize)
    }
    pub fn with_parity_en(mut self, value: bool) -> Self {
        self.value.set_bit(21usize, value);
        self
    }
    pub fn shared_attr_override_en(&self) -> bool {
        self.value.get_bit(22usize)
    }
    pub fn with_shared_attr_override_en(mut self, value: bool) -> Self {
        self.value.set_bit(22usize, value);
        self
    }
    pub fn force_write_alloc(&self) -> Option<ForceWriteAllocate> {
        ForceWriteAllocate::try_from(self.value.get_bits(23usize..=24usize)).ok()
    }
    pub fn with_force_write_alloc(mut self, value: ForceWriteAllocate) -> Self {
        self.value.set_bits(23usize..=24usize, Into::<u32>::into(value));
        self
    }
    pub fn cache_replace_policy(&self) -> bool {
        self.value.get_bit(25usize)
    }
    pub fn with_cache_replace_policy(mut self, value: bool) -> Self {
        self.value.set_bit(25usize, value);
        self
    }
    pub fn nonsec_lockdown_en(&self) -> bool {
        self.value.get_bit(26usize)
    }
    pub fn with_nonsec_lockdown_en(mut self, value: bool) -> Self {
        self.value.set_bit(26usize, value);
        self
    }
    pub fn nonsec_inte_access_ctrl(&self) -> bool {
        self.value.get_bit(27usize)
    }
    pub fn with_nonsec_inte_access_ctrl(mut self, value: bool) -> Self {
        self.value.set_bit(27usize, value);
        self
    }
    pub fn data_prefetch_en(&self) -> bool {
        self.value.get_bit(28usize)
    }
    pub fn with_data_prefetch_en(mut self, value: bool) -> Self {
        self.value.set_bit(28usize, value);
        self
    }
    pub fn instr_prefetch_en(&self) -> bool {
        self.value.get_bit(29usize)
    }
    pub fn with_instr_prefetch_en(mut self, value: bool) -> Self {
        self.value.set_bit(29usize, value);
        self
    }
    pub fn early_bresp_en(&self) -> bool {
        self.value.get_bit(30usize)
    }
    pub fn with_early_bresp_en(mut self, value: bool) -> Self {
        self.value.set_bit(30usize, value);
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaySize {
    SixteenKb = 1usize as u8,
    ThirtyTwoKb = 2usize as u8,
    SixtyFourKb = 3usize as u8,
    OneTwentyEightKb = 4usize as u8,
    TwoFiftySixKb = 5usize as u8,
    FiveTwelveKb = 6usize as u8,
}
impl Default for WaySize {
    fn default() -> Self {
        Self::SixtyFourKb
    }
}
impl From<WaySize> for u8 {
    fn from(value: WaySize) -> Self {
        value as u8
    }
}
impl From<WaySize> for u32 {
    fn from(value: WaySize) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for WaySize {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            1usize => Ok(WaySize::SixteenKb),
            2usize => Ok(WaySize::ThirtyTwoKb),
            3usize => Ok(WaySize::SixtyFourKb),
            4usize => Ok(WaySize::OneTwentyEightKb),
            5usize => Ok(WaySize::TwoFiftySixKb),
            6usize => Ok(WaySize::FiveTwelveKb),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for WaySize {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            1usize => Ok(WaySize::SixteenKb),
            2usize => Ok(WaySize::ThirtyTwoKb),
            3usize => Ok(WaySize::SixtyFourKb),
            4usize => Ok(WaySize::OneTwentyEightKb),
            5usize => Ok(WaySize::TwoFiftySixKb),
            6usize => Ok(WaySize::FiveTwelveKb),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForceWriteAllocate {
    UseAwcache = 0usize as u8,
    ForceNoAllocate = 1usize as u8,
    OverrideAwcache = 2usize as u8,
}
impl Default for ForceWriteAllocate {
    fn default() -> Self {
        Self::UseAwcache
    }
}
impl From<ForceWriteAllocate> for u8 {
    fn from(value: ForceWriteAllocate) -> Self {
        value as u8
    }
}
impl From<ForceWriteAllocate> for u32 {
    fn from(value: ForceWriteAllocate) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for ForceWriteAllocate {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ForceWriteAllocate::UseAwcache),
            1usize => Ok(ForceWriteAllocate::ForceNoAllocate),
            2usize => Ok(ForceWriteAllocate::OverrideAwcache),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for ForceWriteAllocate {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ForceWriteAllocate::UseAwcache),
            1usize => Ok(ForceWriteAllocate::ForceNoAllocate),
            2usize => Ok(ForceWriteAllocate::OverrideAwcache),
            _ => Err(()),
        }
    }
}
