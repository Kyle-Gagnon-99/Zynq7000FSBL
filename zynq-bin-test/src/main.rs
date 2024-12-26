#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

use zynq_hal::{print, stdio::get_uart, uart};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("Panic: {}\n", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello, world!\n");

    //write!(get_uart(), "Hello, world!\n").unwrap();

    loop {}
}
