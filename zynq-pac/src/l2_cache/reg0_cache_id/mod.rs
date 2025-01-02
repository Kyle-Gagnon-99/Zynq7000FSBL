use crate::*;
pub struct Reg0CacheIdRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg0CacheIdBuilder {
    value: u32,
}
impl FromBits<u32> for Reg0CacheIdBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg0CacheIdBuilder, u32> for Reg0CacheIdRegister {
    fn read(&self) -> Reg0CacheIdBuilder {
        Reg0CacheIdBuilder::from_bits(self.inner.read())
    }
}
impl Default for Reg0CacheIdBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(0usize..=5usize, 8usize as u32);
        value.set_bits(6usize..=9usize, 3usize as u32);
        value.set_bits(10usize..=15usize, 0usize as u32);
        value.set_bits(24usize..=31usize, 65usize as u32);
        Self { value }
    }
}
impl Reg0CacheIdBuilder {
    pub fn rtl_release(&self) -> u8 {
        self.value.get_bits(0usize..=5usize) as u8
    }
    pub fn part_num(&self) -> u8 {
        self.value.get_bits(6usize..=9usize) as u8
    }
    pub fn cache_id(&self) -> u8 {
        self.value.get_bits(10usize..=15usize) as u8
    }
    pub fn implementer(&self) -> u8 {
        self.value.get_bits(24usize..=31usize) as u8
    }
}
