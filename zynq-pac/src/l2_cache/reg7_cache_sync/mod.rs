use crate::*;
pub struct Reg7CacheSyncRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg7CacheSyncBuilder {
    value: u32,
}
impl FromBits<u32> for Reg7CacheSyncBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg7CacheSyncBuilder, u32> for Reg7CacheSyncRegister {
    fn read(&self) -> Reg7CacheSyncBuilder {
        Reg7CacheSyncBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg7CacheSyncBuilder, u32> for Reg7CacheSyncRegister {
    fn zeroed() -> Reg7CacheSyncBuilder {
        Reg7CacheSyncBuilder::default()
    }
    fn write(&mut self, value: Reg7CacheSyncBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg7CacheSyncBuilder, u32> for Reg7CacheSyncRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg7CacheSyncBuilder) -> Reg7CacheSyncBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg7CacheSyncBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg7CacheSyncBuilder {
    pub fn cache_sync(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_cache_sync(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
}
