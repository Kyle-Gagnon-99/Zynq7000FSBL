#![no_std]
#![no_main]

extern crate zrbl_support;

use core::panic::PanicInfo;

use zynq_hal::{print, println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("Panic: {}\n", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn main_core0() -> ! {
    println!("  ______  _____  ____  _      ");
    println!(" |___  / |  __ \\|  _ \\| |     ");
    println!("    / /  | |__) | |_) | |     ");
    println!("   / /   |  _  /|  _ <| |     ");
    println!("  / /__  | | \\ \\| |_) | |____ ");
    println!(" /_____| |_|  \\_\\____/|______|");

    loop {}
}
