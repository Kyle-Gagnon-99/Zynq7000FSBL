use crate::*;
pub struct DdrPllCfgRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct DdrPllCfgBuilder {
    value: u32,
}
impl FromBits<u32> for DdrPllCfgBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<DdrPllCfgBuilder, u32> for DdrPllCfgRegister {
    fn read(&self) -> DdrPllCfgBuilder {
        DdrPllCfgBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<DdrPllCfgBuilder, u32> for DdrPllCfgRegister {
    fn zeroed() -> DdrPllCfgBuilder {
        DdrPllCfgBuilder::default()
    }
    fn write(&mut self, value: DdrPllCfgBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<DdrPllCfgBuilder, u32> for DdrPllCfgRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(DdrPllCfgBuilder) -> DdrPllCfgBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for DdrPllCfgBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(4usize..=7usize, 10usize as u32);
        value.set_bits(8usize..=11usize, 14usize as u32);
        Self { value }
    }
}
impl DdrPllCfgBuilder {
    pub fn pll_res(&self) -> u8 {
        self.value.get_bits(4usize..=7usize) as u8
    }
    pub fn with_pll_res(mut self, value: u8) -> Self {
        self.value.set_bits(4usize..=7usize, value as u32);
        self
    }
    pub fn pll_cp(&self) -> u8 {
        self.value.get_bits(8usize..=11usize) as u8
    }
    pub fn with_pll_cp(mut self, value: u8) -> Self {
        self.value.set_bits(8usize..=11usize, value as u32);
        self
    }
    pub fn lock_cnt(&self) -> u16 {
        self.value.get_bits(12usize..=21usize) as u16
    }
    pub fn with_lock_cnt(mut self, value: u16) -> Self {
        self.value.set_bits(12usize..=21usize, value as u32);
        self
    }
}
