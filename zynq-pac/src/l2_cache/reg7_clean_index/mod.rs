use crate::*;
pub struct Reg7CleanIndexRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg7CleanIndexBuilder {
    value: u32,
}
impl FromBits<u32> for Reg7CleanIndexBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg7CleanIndexBuilder, u32> for Reg7CleanIndexRegister {
    fn read(&self) -> Reg7CleanIndexBuilder {
        Reg7CleanIndexBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg7CleanIndexBuilder, u32> for Reg7CleanIndexRegister {
    fn zeroed() -> Reg7CleanIndexBuilder {
        Reg7CleanIndexBuilder::default()
    }
    fn write(&mut self, value: Reg7CleanIndexBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg7CleanIndexBuilder, u32> for Reg7CleanIndexRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg7CleanIndexBuilder) -> Reg7CleanIndexBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg7CleanIndexBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg7CleanIndexBuilder {
    pub fn cache(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_cache(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn index(&self) -> u8 {
        self.value.get_bits(5usize..=11usize) as u8
    }
    pub fn with_index(mut self, value: u8) -> Self {
        self.value.set_bits(5usize..=11usize, value as u32);
        self
    }
    pub fn way(&self) -> u8 {
        self.value.get_bits(28usize..=30usize) as u8
    }
    pub fn with_way(mut self, value: u8) -> Self {
        self.value.set_bits(28usize..=30usize, value as u32);
        self
    }
}
