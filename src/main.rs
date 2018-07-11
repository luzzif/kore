#![no_std]
#![no_main]
#![feature(panic_implementation)]

extern crate volatile;

mod vga;

use core::panic::PanicInfo;
use vga::writer::VgaWriter;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = VgaWriter::new();
    writer.write("Hello world!");

    loop {}
}

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
