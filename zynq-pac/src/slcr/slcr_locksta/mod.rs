use crate::*;
pub struct SlcrLockstaRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct SlcrLockstaBuilder {
    value: u32,
}
impl FromBits<u32> for SlcrLockstaBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<SlcrLockstaBuilder, u32> for SlcrLockstaRegister {
    fn read(&self) -> SlcrLockstaBuilder {
        SlcrLockstaBuilder::from_bits(self.inner.read())
    }
}
impl Default for SlcrLockstaBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl SlcrLockstaBuilder {
    pub fn locksta(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_locksta(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
}
