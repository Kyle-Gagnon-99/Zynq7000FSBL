pub use volatile_register::{RO, WO, RW};
pub use bit_field::BitField;
pub mod scl;
pub mod slcr_lock;
pub mod slcr_unlock;
pub mod slcr_locksta;
pub mod arm_pll_ctrl;
pub mod ddr_pll_ctrl;
pub mod io_pll_ctrl;
pub mod pll_status;
pub mod arm_pll_cfg;
pub mod ddr_pll_cfg;
pub mod io_pll_cfg;
pub mod arm_clk_ctrl;
pub mod ddr_clk_ctrl;
pub mod dci_clk_ctrl;
pub mod aper_clk_ctrl;
pub mod uart_clk_ctrl;
pub mod fpga0_clk_ctrl;
pub mod fpga1_clk_ctrl;
pub mod fpga2_clk_ctrl;
pub mod fpga3_clk_ctrl;
pub mod clk_621_true;
pub mod pss_rst_ctrl;
pub mod uart_rst_ctrl;
pub mod fpga_rst_ctrl;
pub mod a9_cpu_rst_ctrl;
pub mod boot_mode;
pub mod mio_pin_00;
pub mod mio_pin_01;
pub mod mio_pin_02;
pub mod mio_pin_03;
pub mod mio_pin_04;
pub mod mio_pin_05;
pub mod mio_pin_06;
pub mod mio_pin_07;
pub mod mio_pin_08;
pub mod mio_pin_09;
pub mod mio_pin_10;
pub mod mio_pin_11;
pub mod mio_pin_12;
pub mod mio_pin_13;
pub mod mio_pin_14;
pub mod mio_pin_15;
#[repr(C)]
pub struct RegisterBlock {
    pub scl: scl::SclRegister,
    pub slcr_lock: slcr_lock::SlcrLockRegister,
    pub slcr_unlock: slcr_unlock::SlcrUnlockRegister,
    pub slcr_locksta: slcr_locksta::SlcrLockstaRegister,
    pub _reserved4: [u32; 60usize],
    pub arm_pll_ctrl: arm_pll_ctrl::ArmPllCtrlRegister,
    pub ddr_pll_ctrl: ddr_pll_ctrl::DdrPllCtrlRegister,
    pub io_pll_ctrl: io_pll_ctrl::IoPllCtrlRegister,
    pub pll_status: pll_status::PllStatusRegister,
    pub arm_pll_cfg: arm_pll_cfg::ArmPllCfgRegister,
    pub ddr_pll_cfg: ddr_pll_cfg::DdrPllCfgRegister,
    pub io_pll_cfg: io_pll_cfg::IoPllCfgRegister,
    pub _reserved11: [u32; 1usize],
    pub arm_clk_ctrl: arm_clk_ctrl::ArmClkCtrlRegister,
    pub ddr_clk_ctrl: ddr_clk_ctrl::DdrClkCtrlRegister,
    pub dci_clk_ctrl: dci_clk_ctrl::DciClkCtrlRegister,
    pub aper_clk_ctrl: aper_clk_ctrl::AperClkCtrlRegister,
    pub usb0_clk_ctrl: volatile_register::RW<u32>,
    pub usb1_clk_ctrl: volatile_register::RW<u32>,
    pub gem0_rclk_ctrl: volatile_register::RW<u32>,
    pub gem1_rclk_ctrl: volatile_register::RW<u32>,
    pub gem0_clk_ctrl: volatile_register::RW<u32>,
    pub gem1_clk_ctrl: volatile_register::RW<u32>,
    pub smc_clk_ctrl: volatile_register::RW<u32>,
    pub lqspi_clk_ctrl: volatile_register::RW<u32>,
    pub sdio_clk_ctrl: volatile_register::RW<u32>,
    pub uart_clk_ctrl: uart_clk_ctrl::UartClkCtrlRegister,
    pub spi_clk_ctrl: volatile_register::RW<u32>,
    pub can_clk_ctrl: volatile_register::RW<u32>,
    pub can_mioclk_ctrl: volatile_register::RW<u32>,
    pub dbg_clk_ctrl: volatile_register::RW<u32>,
    pub pcap_clk_ctrl: volatile_register::RW<u32>,
    pub topsw_clk_ctrl: volatile_register::RW<u32>,
    pub fpga0_clk_ctrl: fpga0_clk_ctrl::Fpga0ClkCtrlRegister,
    pub fpga0_thr_ctrl: volatile_register::RW<u32>,
    pub fpga0_thr_cnt: volatile_register::RW<u32>,
    pub fpga0_thr_sta: volatile_register::RO<u32>,
    pub fpga1_clk_ctrl: fpga1_clk_ctrl::Fpga1ClkCtrlRegister,
    pub fpga1_thr_ctrl: volatile_register::RW<u32>,
    pub fpga1_thr_cnt: volatile_register::RW<u32>,
    pub fpga1_thr_sta: volatile_register::RO<u32>,
    pub fpga2_clk_ctrl: fpga2_clk_ctrl::Fpga2ClkCtrlRegister,
    pub fpga2_thr_ctrl: volatile_register::RW<u32>,
    pub fpga2_thr_cnt: volatile_register::RW<u32>,
    pub fpga2_thr_sta: volatile_register::RO<u32>,
    pub fpga3_clk_ctrl: fpga3_clk_ctrl::Fpga3ClkCtrlRegister,
    pub fpga3_thr_ctrl: volatile_register::RW<u32>,
    pub fpga3_thr_cnt: volatile_register::RW<u32>,
    pub fpga3_thr_sta: volatile_register::RO<u32>,
    pub _reserved47: [u32; 5usize],
    pub clk_621_true: clk_621_true::Clk621TrueRegister,
    pub _reserved48: [u32; 14usize],
    pub pss_rst_ctrl: pss_rst_ctrl::PssRstCtrlRegister,
    pub ddr_rst_ctrl: volatile_register::RW<u32>,
    pub topsw_rst_ctrl: volatile_register::RW<u32>,
    pub dmac_rst_ctrl: volatile_register::RW<u32>,
    pub usb_rst_ctrl: volatile_register::RW<u32>,
    pub gem_rst_ctrl: volatile_register::RW<u32>,
    pub sdio_rst_ctrl: volatile_register::RW<u32>,
    pub spi_rst_ctrl: volatile_register::RW<u32>,
    pub can_rst_ctrl: volatile_register::RW<u32>,
    pub i2c_rst_ctrl: volatile_register::RW<u32>,
    pub uart_rst_ctrl: uart_rst_ctrl::UartRstCtrlRegister,
    pub gpio_rst_ctrl: volatile_register::RW<u32>,
    pub lqspi_rst_ctrl: volatile_register::RW<u32>,
    pub smc_rst_ctrl: volatile_register::RW<u32>,
    pub ocm_rst_ctrl: volatile_register::RW<u32>,
    pub _reserved63: [u32; 1usize],
    pub fpga_rst_ctrl: fpga_rst_ctrl::FpgaRstCtrlRegister,
    pub a9_cpu_rst_ctrl: a9_cpu_rst_ctrl::A9CpuRstCtrlRegister,
    pub _reserved65: [u32; 1usize],
    pub rs_awdt_ctrl: volatile_register::RW<u32>,
    pub _reserved66: [u32; 2usize],
    pub reboot_status: volatile_register::RO<u32>,
    pub boot_mode: boot_mode::BootModeRegister,
    pub _reserved68: [u32; 40usize],
    pub apu_ctrl: volatile_register::RW<u32>,
    pub wdt_clk_sel: volatile_register::RW<u32>,
    pub _reserved70: [u32; 78usize],
    pub tz_dma_ns: volatile_register::RW<u32>,
    pub tz_dma_irq_ns: volatile_register::RW<u32>,
    pub tz_dma_periph_ns: volatile_register::RW<u32>,
    pub _reserved73: [u32; 57usize],
    pub pss_idcode: volatile_register::RO<u32>,
    pub _reserved74: [u32; 51usize],
    pub ddr_urgent: volatile_register::RW<u32>,
    pub _reserved75: [u32; 2usize],
    pub ddr_cal_start: volatile_register::RW<u32>,
    pub _reserved76: [u32; 1usize],
    pub ddr_ref_start: volatile_register::RW<u32>,
    pub ddr_cmd_sta: volatile_register::RO<u32>,
    pub ddr_urgent_sel: volatile_register::RW<u32>,
    pub ddr_dfi_status: volatile_register::RO<u32>,
    pub _reserved80: [u32; 55usize],
    pub mio_pin_00: mio_pin_00::MioPin00Register,
    pub mio_pin_01: mio_pin_01::MioPin01Register,
    pub mio_pin_02: mio_pin_02::MioPin02Register,
    pub mio_pin_03: mio_pin_03::MioPin03Register,
    pub mio_pin_04: mio_pin_04::MioPin04Register,
    pub mio_pin_05: mio_pin_05::MioPin05Register,
    pub mio_pin_06: mio_pin_06::MioPin06Register,
    pub mio_pin_07: mio_pin_07::MioPin07Register,
    pub mio_pin_08: mio_pin_08::MioPin08Register,
    pub mio_pin_09: mio_pin_09::MioPin09Register,
    pub mio_pin_10: mio_pin_10::MioPin10Register,
    pub mio_pin_11: mio_pin_11::MioPin11Register,
    pub mio_pin_12: mio_pin_12::MioPin12Register,
    pub mio_pin_13: mio_pin_13::MioPin13Register,
    pub mio_pin_14: mio_pin_14::MioPin14Register,
    pub mio_pin_15: mio_pin_15::MioPin15Register,
}
impl RegisterBlock {
    #[allow(unused)]
    #[inline(always)]
    pub fn slcr0() -> &'static mut Self {
        let addr = 4160749568usize as *mut RegisterBlock;
        unsafe { &mut *addr }
    }
}
