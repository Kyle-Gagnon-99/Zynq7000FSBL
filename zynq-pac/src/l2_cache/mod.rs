pub use volatile_register::{RO, WO, RW};
pub use bit_field::BitField;
pub mod reg0_cache_id;
pub mod reg0_cache_type;
pub mod reg1_control;
pub mod reg1_aux_control;
pub mod reg1_tag_ram_control;
pub mod reg1_data_ram_control;
pub mod reg2_ev_counter_ctrl;
pub mod reg2_ev_counter1_cfg;
pub mod reg2_ev_counter0_cfg;
pub mod reg2_int_mask;
pub mod reg2_int_mask_status;
pub mod reg2_int_raw_status;
pub mod reg2_int_clear;
pub mod reg7_cache_sync;
pub mod reg7_clean_index;
pub mod reg7_clean_inv_index;
pub mod reg15_prefetch_ctrl;
#[repr(C)]
pub struct RegisterBlock {
    pub reg0_cache_id: reg0_cache_id::Reg0CacheIdRegister,
    pub reg0_cache_type: reg0_cache_type::Reg0CacheTypeRegister,
    pub _reserved2: [u32; 62usize],
    pub reg1_control: reg1_control::Reg1ControlRegister,
    pub reg1_aux_control: reg1_aux_control::Reg1AuxControlRegister,
    pub reg1_tag_ram_control: reg1_tag_ram_control::Reg1TagRamControlRegister,
    pub reg1_data_ram_control: reg1_data_ram_control::Reg1DataRamControlRegister,
    pub _reserved6: [u32; 60usize],
    pub reg2_ev_counter_ctrl: reg2_ev_counter_ctrl::Reg2EvCounterCtrlRegister,
    pub reg2_ev_counter1_cfg: reg2_ev_counter1_cfg::Reg2EvCounter1CfgRegister,
    pub reg2_ev_counter0_cfg: reg2_ev_counter0_cfg::Reg2EvCounter0CfgRegister,
    pub reg2_ev_counter1: volatile_register::RO<u32>,
    pub reg2_ev_counter0: volatile_register::RO<u32>,
    pub reg2_int_mask: reg2_int_mask::Reg2IntMaskRegister,
    pub reg2_int_mask_status: reg2_int_mask_status::Reg2IntMaskStatusRegister,
    pub reg2_int_raw_status: reg2_int_raw_status::Reg2IntRawStatusRegister,
    pub reg2_int_clear: reg2_int_clear::Reg2IntClearRegister,
    pub _reserved15: [u32; 323usize],
    pub reg7_cache_sync: reg7_cache_sync::Reg7CacheSyncRegister,
    pub _reserved16: [u32; 15usize],
    pub reg7_inv_pa: volatile_register::RW<u32>,
    pub _reserved17: [u32; 2usize],
    pub reg7_inv_way: volatile_register::RW<u32>,
    pub _reserved18: [u32; 12usize],
    pub reg7_clean_pa: volatile_register::RW<u32>,
    pub _reserved19: [u32; 1usize],
    pub reg7_clean_index: reg7_clean_index::Reg7CleanIndexRegister,
    pub reg7_clean_way: volatile_register::RW<u32>,
    pub _reserved21: [u32; 12usize],
    pub reg7_clean_inv_pa: volatile_register::RW<u32>,
    pub _reserved22: [u32; 1usize],
    pub reg7_clean_inv_index: reg7_clean_inv_index::Reg7CleanInvIndexRegister,
    pub reg7_clean_inv_way: volatile_register::RW<u32>,
    pub _reserved24: [u32; 472usize],
    pub reg15_prefetch_ctrl: reg15_prefetch_ctrl::Reg15PrefetchCtrlRegister,
}
impl RegisterBlock {
    #[allow(unused)]
    #[inline(always)]
    pub fn l2_cache0() -> &'static mut Self {
        let addr = 4176486400usize as *mut RegisterBlock;
        unsafe { &mut *addr }
    }
}
