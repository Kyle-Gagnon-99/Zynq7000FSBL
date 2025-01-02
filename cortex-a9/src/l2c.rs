use zynq_pac::{
    l2_cache::{reg1_aux_control::WaySize, reg7_cache_sync::Reg7CacheSyncBuilder, RegisterBlock},
    RegisterRO, RegisterRW, RegisterWO,
};

use crate::asm::dmb;

pub fn enable_l2_cache(offset: u8) {
    dmb();
    let regs = RegisterBlock::l2_cache0();

    unsafe {
        core::ptr::write_volatile(0xF8000A1Cusize as *mut u32, 0x020202);
    }

    // Disable L2 cache
    regs.reg1_control
        .modify(|builder| builder.with_l2_enable(false));

    regs.reg15_prefetch_ctrl.modify(|builder| {
        builder
            .with_instr_pref_en(true)
            .with_data_pref_en(true)
            .with_double_linefill_en(true)
            .with_incr_double_linefill_en(true)
            .with_pref_drop_en(true)
            .with_prefetch_offset(offset)
    });

    regs.reg1_aux_control.modify(|builder| {
        builder
            .with_early_bresp_en(true)
            .with_instr_prefetch_en(true)
            .with_data_prefetch_en(true)
            .with_cache_replace_policy(true)
            .with_way_size(WaySize::SixtyFourKb)
    });

    // Set up RAM tag and data latency
    regs.reg1_tag_ram_control.modify(|builder| {
        builder
            .with_ram_wr_access_lat(
                zynq_pac::l2_cache::reg1_tag_ram_control::RamWriteAccessLatency::TwoCycles,
            )
            .with_ram_rd_access_lat(
                zynq_pac::l2_cache::reg1_tag_ram_control::RamReadAccessLatency::TwoCycles,
            )
            .with_ram_setup_lat(
                zynq_pac::l2_cache::reg1_tag_ram_control::RamSetupLatency::TwoCycles,
            )
    });

    regs.reg1_data_ram_control.modify(|builder| {
        builder
            .with_ram_wr_access_lat(
                zynq_pac::l2_cache::reg1_data_ram_control::RamWriteAccessLatency::TwoCycles,
            )
            .with_ram_rd_access_lat(
                zynq_pac::l2_cache::reg1_data_ram_control::RamReadAccessLatency::ThreeCycles,
            )
            .with_ram_setup_lat(
                zynq_pac::l2_cache::reg1_data_ram_control::RamSetupLatency::TwoCycles,
            )
    });

    // Invalidate L2 ways
    unsafe {
        regs.reg7_inv_way.write(0xFFFF);
    }

    // Poll for completion
    while regs.reg7_cache_sync.read().cache_sync() {}

    dmb();
}

#[inline(always)]
pub fn l2_cache_invalidate_all() {
    let regs = RegisterBlock::l2_cache0();
    unsafe {
        regs.reg7_inv_way.write(0xFFFF);
    }

    // Poll for completion
    while regs.reg7_cache_sync.read().cache_sync() {}
}

#[inline(always)]
pub fn l2_cache_clean_all() {
    let regs = RegisterBlock::l2_cache0();
    unsafe {
        regs.reg7_clean_way.write(0xFFFF);
    }

    // Poll for completion
    while regs.reg7_cache_sync.read().cache_sync() {}
}

#[inline(always)]
pub fn l2_cache_clean_and_invalidate_all() {
    let regs = RegisterBlock::l2_cache0();
    unsafe {
        regs.reg7_clean_inv_way.write(0xFFFF);
    }

    // Poll for completion
    while regs.reg7_cache_sync.read().cache_sync() {}
}

#[inline(always)]
pub fn l2_cache_sync() {
    let regs = RegisterBlock::l2_cache0();
    regs.reg7_cache_sync
        .write(Reg7CacheSyncBuilder::default().with_cache_sync(false));
}

#[inline(always)]
pub fn l2_cache_clean(addr: usize) {
    let regs = RegisterBlock::l2_cache0();
    unsafe {
        regs.reg7_clean_pa.write(addr as u32);
    }
}

#[inline(always)]
pub fn l2_cache_invalidate(addr: usize) {
    let regs = RegisterBlock::l2_cache0();
    unsafe {
        regs.reg7_inv_pa.write(addr as u32);
    }
}

#[inline(always)]
pub fn l2_cache_clean_invalidate(addr: usize) {
    let regs = RegisterBlock::l2_cache0();
    unsafe {
        regs.reg7_clean_inv_pa.write(addr as u32);
    }
}
