use crate::*;
pub struct Reg0CacheTypeRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg0CacheTypeBuilder {
    value: u32,
}
impl FromBits<u32> for Reg0CacheTypeBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg0CacheTypeBuilder, u32> for Reg0CacheTypeRegister {
    fn read(&self) -> Reg0CacheTypeBuilder {
        Reg0CacheTypeBuilder::from_bits(self.inner.read())
    }
}
impl Default for Reg0CacheTypeBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg0CacheTypeBuilder {
    pub fn l2_cache_line_len_l(&self) -> u8 {
        self.value.get_bits(0usize..=1usize) as u8
    }
    pub fn l2_assoc_l(&self) -> bool {
        self.value.get_bit(6usize)
    }
    pub fn lsize_mid(&self) -> u8 {
        self.value.get_bits(8usize..=10usize) as u8
    }
    pub fn l2_cache_line_len_d(&self) -> u8 {
        self.value.get_bits(12usize..=13usize) as u8
    }
    pub fn l2_assoc_d(&self) -> bool {
        self.value.get_bit(18usize)
    }
    pub fn dsize_mid(&self) -> u8 {
        self.value.get_bits(20usize..=22usize) as u8
    }
    pub fn ctype(&self) -> u8 {
        self.value.get_bits(25usize..=28usize) as u8
    }
    pub fn data_banking(&self) -> bool {
        self.value.get_bit(31usize)
    }
}
