use core::ops::{Deref, DerefMut};

use cortex_a9::mutex::{Mutex, MutexGuard};

use crate::uart::Uart;

static mut UART: Mutex<LazyUart> = Mutex::new(LazyUart::Unitialized);

pub fn get_uart<'a>() -> MutexGuard<'a, LazyUart> {
    unsafe { UART.lock() }
}

pub enum LazyUart {
    Unitialized,
    Initialized(Uart),
}

impl Deref for LazyUart {
    type Target = Uart;

    fn deref(&self) -> &Self::Target {
        match self {
            LazyUart::Unitialized => panic!("UART not initialized"),
            LazyUart::Initialized(uart) => uart,
        }
    }
}

impl DerefMut for LazyUart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            LazyUart::Unitialized => {
                let uart = Uart::uart0();
                *self = LazyUart::Initialized(uart);
                self
            }
            LazyUart::Initialized(uart) => uart,
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut uart = $crate::stdio::get_uart();
        let _ = write!(uart, $($arg)*);
    })
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut uart = $crate::stdio::get_uart();
        let _ = write!(uart, $($arg)*);
        let _ = write!(uart, "\n");
        // flush after the newline
        while !*uart.tx_idle() {}
    })
}
