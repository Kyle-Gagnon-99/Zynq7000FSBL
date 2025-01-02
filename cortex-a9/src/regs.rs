use core::arch::asm;

use registers::{RegisterR, RegisterW};

macro_rules! def_reg_r {
    ($name:ident, $type:ty, $asm_instr:tt) => {
        impl RegisterR for $name {
            type R = $type;

            fn read(&self) -> Self::R {
                let r: $type;
                unsafe {
                    asm!(
                        $asm_instr,
                        out(reg) r,
                        options(nomem, nostack, preserves_flags)
                    );
                }
                r
            }
        }
    }
}

macro_rules! def_reg_w {
    ($name:ident, $type:ty, $asm_instr:tt) => {
        impl RegisterW for $name {
            type W = $type;

            fn write(&self, value: Self::W) {
                unsafe {
                    asm!(
                        $asm_instr,
                        in(reg) value,
                        options(nomem, nostack, preserves_flags)
                    );
                }
            }
        }
    }
}

macro_rules! def_reg_rw {
    ($name:ident, $type:ty, $asm_instr:tt) => {
        def_reg_r!($name, $type, $asm_instr);
        def_reg_w!($name, $type, $asm_instr);
    };
}

// Create a macro rule that reads from a register and returns the bit using the get_bit method
// Create a macro rule that writes to a register and sets the bit using the set_bit method
macro_rules! register_bit_r {
    ($reg:ident, $bit_name:ident, $bit_pos:expr) => {
        impl $reg {
            pub fn $bit_name(&self) -> bool {
                self.read() & (1 << $bit_pos) != 0
            }
        }
    };
}

macro_rules! register_bit_w {
    ($reg:ident, $set_bit_name:ident, $bit_pos:expr) => {
        impl $reg {
            pub fn $set_bit_name(&self, value: bool) {
                let mut r = self.read();
                if value {
                    r |= 1 << $bit_pos;
                } else {
                    r &= !(1 << $bit_pos);
                }
                self.write(r);
            }
        }
    };
}

macro_rules! register_bit_rw {
    ($reg:ident, $bit_name:ident, $set_bit_name:ident, $bit_pos:expr) => {
        register_bit_r!($reg, $bit_name, $bit_pos);
        register_bit_w!($reg, $set_bit_name, $bit_pos);
    };
}

macro_rules! register_bits_r {
    ($reg:ident, $bit_name:ident, $type:ty, $bit_start:expr, $bit_end:expr) => {
        impl $reg {
            pub fn $bit_name(&self) -> $type {
                (self.read() >> $bit_start) & ((1 << ($bit_end - $bit_start + 1)) - 1)
            }
        }
    };
}

macro_rules! register_bits_w {
    ($reg:ident, $set_bit_name:ident, $type:ty, $bit_start:expr, $bit_end:expr) => {
        impl $reg {
            pub fn $set_bit_name(&self, value: $type) {
                let mut r = self.read();
                r &= !(((1 << ($bit_end - $bit_start + 1)) - 1) << $bit_start);
                r |= value << $bit_start;
                self.write(r);
            }
        }
    };
}

macro_rules! register_bits_rw {
    ($reg:ident, $bit_name:ident, $set_bit_name:ident, $type:ty, $bit_start:expr, $bit_end:expr) => {
        register_bits_r!($reg, $bit_name, $type, $bit_start, $bit_end);
        register_bits_w!($reg, $set_bit_name, $type, $bit_start, $bit_end);
    };
}

pub struct MPIDR;
def_reg_r!(MPIDR, u32, "mrc p15, 0, {0}, c0, c0, 5");

impl MPIDR {
    pub fn get_cpu_id(&self) -> u32 {
        self.read() & 0x3
    }
}

pub struct SCTLR;
def_reg_rw!(SCTLR, u32, "mrc p15, 0, {0}, c1, c0, 0");
register_bit_rw!(SCTLR, mmu, set_mmu, 0);
register_bit_rw!(SCTLR, alignment_check, set_alignment_check, 1);
register_bit_rw!(SCTLR, data_cache, set_data_cache, 2);
register_bit_rw!(SCTLR, sw, set_sw, 10);
register_bit_rw!(SCTLR, z, set_z, 11);
register_bit_rw!(SCTLR, i, set_i, 12);
register_bit_rw!(SCTLR, v, set_v, 13);
register_bit_rw!(SCTLR, ha, set_ha, 17);
register_bit_rw!(SCTLR, unaligned_access, set_unaligned_access, 22);
register_bit_rw!(SCTLR, ee, set_ee, 25);
register_bit_rw!(SCTLR, nmfi, set_nmfi, 27);
register_bit_rw!(SCTLR, tre, set_tre, 28);
register_bit_rw!(SCTLR, afe, set_afe, 29);
register_bit_rw!(SCTLR, te, set_te, 30);

pub struct ACTLR;
def_reg_rw!(ACTLR, u32, "mrc p15, 0, {0}, c1, c0, 1");
register_bit_rw!(ACTLR, parity_enable, set_parity_enable, 9);
register_bit_rw!(ACTLR, alloc_one_way, set_alloc_one_way, 8);
register_bit_rw!(ACTLR, excl, set_excl, 7);
register_bit_rw!(ACTLR, smp, set_smp, 6);
register_bit_rw!(
    ACTLR,
    write_full_line_of_zeros,
    set_write_full_line_of_zeros,
    3
);
register_bit_rw!(ACTLR, l1_prefetch, set_l1_prefetch, 2);
register_bit_rw!(ACTLR, l2_prefetch, set_l2_prefetch, 1);
register_bit_rw!(ACTLR, fw, set_fw, 0);

pub struct DACR;
def_reg_rw!(DACR, u32, "mrc p15, 0, {0}, c3, c0, 0");

pub struct TTBR0;
def_reg_rw!(TTBR0, u32, "mrc p15, 0, {0}, c2, c0, 0");
register_bits_rw!(TTBR0, table_base, set_table_base, u32, 14, 31);
register_bit_rw!(TTBR0, irgn0, set_irgn0, 6);
register_bits_rw!(TTBR0, rgn, set_rgn, u32, 3, 4);
register_bit_rw!(TTBR0, s, set_s, 1);
register_bit_rw!(TTBR0, irgn1, set_irgn1, 0);

pub struct TTBR1;
def_reg_rw!(TTBR1, u32, "mrc p15, 0, {0}, c2, c0, 1");
register_bits_rw!(TTBR1, table_base, set_table_base, u32, 14, 31);
register_bit_rw!(TTBR1, irgn0, set_irgn0, 6);
register_bits_rw!(TTBR1, rgn, set_rgn, u32, 3, 4);
register_bit_rw!(TTBR1, s, set_s, 1);
register_bit_rw!(TTBR1, irgn1, set_irgn1, 0);
