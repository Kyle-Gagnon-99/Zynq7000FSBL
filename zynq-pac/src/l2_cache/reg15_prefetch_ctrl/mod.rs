use crate::*;
pub struct Reg15PrefetchCtrlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg15PrefetchCtrlBuilder {
    value: u32,
}
impl FromBits<u32> for Reg15PrefetchCtrlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<Reg15PrefetchCtrlBuilder, u32> for Reg15PrefetchCtrlRegister {
    fn read(&self) -> Reg15PrefetchCtrlBuilder {
        Reg15PrefetchCtrlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<Reg15PrefetchCtrlBuilder, u32> for Reg15PrefetchCtrlRegister {
    fn zeroed() -> Reg15PrefetchCtrlBuilder {
        Reg15PrefetchCtrlBuilder::default()
    }
    fn write(&mut self, value: Reg15PrefetchCtrlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<Reg15PrefetchCtrlBuilder, u32> for Reg15PrefetchCtrlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Reg15PrefetchCtrlBuilder) -> Reg15PrefetchCtrlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for Reg15PrefetchCtrlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        Self { value }
    }
}
impl Reg15PrefetchCtrlBuilder {
    pub fn prefetch_offset(&self) -> u8 {
        self.value.get_bits(0usize..=4usize) as u8
    }
    pub fn with_prefetch_offset(mut self, value: u8) -> Self {
        self.value.set_bits(0usize..=4usize, value as u32);
        self
    }
    pub fn not_same_id_on_excl_seq_en(&self) -> bool {
        self.value.get_bit(21usize)
    }
    pub fn with_not_same_id_on_excl_seq_en(mut self, value: bool) -> Self {
        self.value.set_bit(21usize, value);
        self
    }
    pub fn incr_double_linefill_en(&self) -> bool {
        self.value.get_bit(23usize)
    }
    pub fn with_incr_double_linefill_en(mut self, value: bool) -> Self {
        self.value.set_bit(23usize, value);
        self
    }
    pub fn pref_drop_en(&self) -> bool {
        self.value.get_bit(24usize)
    }
    pub fn with_pref_drop_en(mut self, value: bool) -> Self {
        self.value.set_bit(24usize, value);
        self
    }
    pub fn double_linefill_on_wrapread_en(&self) -> bool {
        self.value.get_bit(27usize)
    }
    pub fn with_double_linefill_on_wrapread_en(mut self, value: bool) -> Self {
        self.value.set_bit(27usize, value);
        self
    }
    pub fn data_pref_en(&self) -> bool {
        self.value.get_bit(28usize)
    }
    pub fn with_data_pref_en(mut self, value: bool) -> Self {
        self.value.set_bit(28usize, value);
        self
    }
    pub fn instr_pref_en(&self) -> bool {
        self.value.get_bit(29usize)
    }
    pub fn with_instr_pref_en(mut self, value: bool) -> Self {
        self.value.set_bit(29usize, value);
        self
    }
    pub fn double_linefill_en(&self) -> bool {
        self.value.get_bit(30usize)
    }
    pub fn with_double_linefill_en(mut self, value: bool) -> Self {
        self.value.set_bit(30usize, value);
        self
    }
}
