use crate::*;
pub struct IoPllCfgRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct IoPllCfgBuilder {
    value: u32,
}
impl FromBits<u32> for IoPllCfgBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<IoPllCfgBuilder, u32> for IoPllCfgRegister {
    fn read(&self) -> IoPllCfgBuilder {
        IoPllCfgBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<IoPllCfgBuilder, u32> for IoPllCfgRegister {
    fn zeroed() -> IoPllCfgBuilder {
        IoPllCfgBuilder::default()
    }
    fn write(&mut self, value: IoPllCfgBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<IoPllCfgBuilder, u32> for IoPllCfgRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(IoPllCfgBuilder) -> IoPllCfgBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for IoPllCfgBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(4usize..=7usize, 10usize as u32);
        value.set_bits(8usize..=11usize, 14usize as u32);
        Self { value }
    }
}
impl IoPllCfgBuilder {
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
