use crate::*;
pub struct SlcrUnlockRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct SlcrUnlockBuilder {
    value: u32,
}
impl FromBits<u32> for SlcrUnlockBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterWO<SlcrUnlockBuilder, u32> for SlcrUnlockRegister {
    fn zeroed() -> SlcrUnlockBuilder {
        SlcrUnlockBuilder::default()
    }
    fn write(&mut self, value: SlcrUnlockBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl Default for SlcrUnlockBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(0usize..=15usize, 57101usize as u32);
        Self { value }
    }
}
impl SlcrUnlockBuilder {
    pub fn unlock(&self) -> u16 {
        self.value.get_bits(0usize..=15usize) as u16
    }
    pub fn with_unlock(mut self, value: u16) -> Self {
        self.value.set_bits(0usize..=15usize, value as u32);
        self
    }
}
