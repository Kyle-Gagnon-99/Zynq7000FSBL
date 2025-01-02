use core::fmt;

use zynq_pac::{
    slcr::{mio_pin_14, mio_pin_15, uart_clk_ctrl::UartClkCtrlBuilder, uart_rst_ctrl},
    uart::{
        baud_div::BaudDivRegister,
        baud_gen::BaudGenRegister,
        fifo::FifoRegister,
        mode::{ChannelMode, ModeRegister, ParityType},
        RegisterBlock,
    },
    RegisterRO, RegisterRW, RegisterWO,
};

use crate::slcr::Slcr;

pub struct Uart {
    regs: &'static mut RegisterBlock,
}

impl Uart {
    pub fn uart0() -> Self {
        // Route UART 0 RxD/TxD Signals to MIO pins
        // Tx Pin
        Slcr::unlocked(|slcr| {
            // Tx Pin
            slcr.mio_pin_15.write(
                mio_pin_15::MioPin15Builder::default()
                    .with_l3_sel(0b111)
                    .with_io_type(mio_pin_15::IoType::Lvcmos33)
                    .with_pullup(true),
            );

            // Rx Pin
            slcr.mio_pin_14.write(
                mio_pin_14::MioPin14Builder::default()
                    .with_l3_sel(0b111)
                    .with_tri_enable(true)
                    .with_io_type(mio_pin_14::IoType::Lvcmos33)
                    .with_pullup(true),
            );
        });

        Slcr::unlocked(|slcr| {
            slcr.uart_rst_ctrl.write(
                uart_rst_ctrl::UartRstCtrlBuilder::default()
                    .with_uart0_ref_rst(true)
                    .with_uart0_cpu1_x_rst(true),
            );
            slcr.aper_clk_ctrl
                .modify(|builder| builder.with_uart0_cpu_1_xclkact(true));
            slcr.uart_clk_ctrl.write(
                UartClkCtrlBuilder::default()
                    .with_divisor(0x14)
                    .with_srcsel(zynq_pac::slcr::uart_clk_ctrl::UartClockSource::IoPll)
                    .with_clkact0(true),
            );
        });

        let mut self_ = Self {
            regs: RegisterBlock::uart0(),
        };
        self_.configure();
        self_
    }

    /// Checks if the UART TX FIFO is empty
    ///
    /// # Returns
    /// Returns true if the TX FIFO is empty, false otherwise
    pub fn tx_fifo_empty(&self) -> bool {
        self.regs.sr.read().tx_fifo_empty()
    }

    pub fn disable_rx(&mut self) {
        self.regs
            .control
            .modify(|builder| builder.with_rx_en(false).with_rx_dis(true));
    }

    pub fn enable_rx(&mut self) {
        self.regs
            .control
            .modify(|builder| builder.with_rx_en(true).with_rx_dis(false));
    }

    pub fn disable_tx(&mut self) {
        self.regs
            .control
            .modify(|builder| builder.with_tx_en(false).with_tx_dis(true));
    }

    pub fn enable_tx(&mut self) {
        self.regs
            .control
            .modify(|builder| builder.with_tx_en(true).with_tx_dis(false));
    }

    pub fn reset_rx(&mut self) {
        self.regs
            .control
            .modify(|builder| builder.with_sw_rx_rst(true));
    }

    pub fn reset_tx(&mut self) {
        self.regs
            .control
            .modify(|builder| builder.with_sw_tx_rst(true));
    }

    pub fn wait_reset(&self) {
        let mut pending = true;
        while pending {
            let control = self.regs.control.read();
            pending = control.sw_rx_rst() || control.sw_tx_rst();
        }
    }

    pub fn configure(&mut self) {
        // Configure UART character frame
        // * Disable clock-divider
        // * 8-bit
        // * 1 stop bit
        // * No parity
        // * Normal mode
        self.regs.mode.write(
            ModeRegister::zeroed()
                .with_parity(ParityType::None)
                .with_channel_mode(ChannelMode::Normal),
        );

        // Configure baud rate
        self.disable_rx();
        self.disable_tx();

        // Set baud rate to 115200
        // For now, just assume the following numbers
        // Baud Gen CD = 4
        // Baud Rate Divider = 216
        self.regs
            .baud_gen
            .write(BaudGenRegister::zeroed().with_cd(4));
        self.regs
            .baud_div
            .write(BaudDivRegister::zeroed().with_bdiv(216));

        // Reset the RX and TX
        self.reset_rx();
        self.reset_tx();
        self.wait_reset();
        self.enable_rx();
        self.enable_tx();
    }

    pub fn write_byte(&mut self, value: u8) {
        // Wait until TX FIFO is not full
        while self.tx_fifo_empty() {}

        // Write byte to TX FIFO
        self.regs
            .fifo
            .write(FifoRegister::zeroed().with_fifo(value));
    }
}

impl fmt::Write for Uart {
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_byte(c as u8);
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_byte(c as u8);
        }
        Ok(())
    }
}
