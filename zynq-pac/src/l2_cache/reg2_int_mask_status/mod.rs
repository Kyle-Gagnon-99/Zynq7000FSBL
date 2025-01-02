use crate::*;
pub struct Reg2IntMaskStatusRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg2IntMaskStatusBuilder {
    value: u32,
}
impl FromBits<u32> for Reg2IntMaskStatusBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg2IntMaskStatusBuilder, u32> for Reg2IntMaskStatusRegister {
    fn read(&self) -> Reg2IntMaskStatusBuilder {
        Reg2IntMaskStatusBuilder::from_bits(self.inner.read())
    }
}
impl Default for Reg2IntMaskStatusBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg2IntMaskStatusBuilder {
    pub fn ecntr(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn parrt(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn parrd(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn errwt(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn errwd(&self) -> bool {
        self.value.get_bit(4usize)
    }
    pub fn errrt(&self) -> bool {
        self.value.get_bit(5usize)
    }
    pub fn errrd(&self) -> bool {
        self.value.get_bit(6usize)
    }
    pub fn slverr(&self) -> bool {
        self.value.get_bit(7usize)
    }
    pub fn decerr(&self) -> bool {
        self.value.get_bit(8usize)
    }
}
