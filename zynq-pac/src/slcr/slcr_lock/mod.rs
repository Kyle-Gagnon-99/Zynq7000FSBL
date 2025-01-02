use crate::*;
pub struct SlcrLockRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct SlcrLockBuilder {
    value: u32,
}
impl FromBits<u32> for SlcrLockBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterWO<SlcrLockBuilder, u32> for SlcrLockRegister {
    fn zeroed() -> SlcrLockBuilder {
        SlcrLockBuilder::default()
    }
    fn write(&mut self, value: SlcrLockBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl Default for SlcrLockBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(0usize..=15usize, 30331usize as u32);
        Self { value }
    }
}
impl SlcrLockBuilder {
    pub fn lock(&self) -> u16 {
        self.value.get_bits(0usize..=15usize) as u16
    }
    pub fn with_lock(mut self, value: u16) -> Self {
        self.value.set_bits(0usize..=15usize, value as u32);
        self
    }
}
