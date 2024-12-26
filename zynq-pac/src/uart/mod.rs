pub use volatile_register::{RO, WO, RW};
pub use bit_field::BitField;
pub mod control;
pub mod mode;
pub mod baud_gen;
pub mod sr;
pub mod fifo;
pub mod baud_div;
#[repr(C)]
pub struct RegisterBlock {
    pub control: control::ControlRegister,
    pub mode: mode::ModeRegister,
    pub ier: volatile_register::RW<u32>,
    pub idr: volatile_register::RW<u32>,
    pub imr: volatile_register::RO<u32>,
    pub isr: volatile_register::RO<u32>,
    pub baud_gen: baud_gen::BaudGenRegister,
    pub rxtout: volatile_register::RW<u32>,
    pub rxwm: volatile_register::RW<u32>,
    pub modemcr: volatile_register::RW<u32>,
    pub modemsr: volatile_register::RW<u32>,
    pub sr: sr::SrRegister,
    pub fifo: fifo::FifoRegister,
    pub baud_div: baud_div::BaudDivRegister,
}
impl RegisterBlock {
    #[allow(unused)]
    #[inline(always)]
    pub fn uart0() -> &'static mut Self {
        let addr = 3758100480usize as *mut RegisterBlock;
        unsafe { &mut *addr }
    }
}
