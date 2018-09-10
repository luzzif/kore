#![cfg_attr(not(test), no_main)]
#![no_std]
#![feature(panic_handler)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;

mod vga;

use core::fmt::Write;
use vga::writer::VGA_WRITER;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(VGA_WRITER.lock(), "Hello world!");
    loop {}
}

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    write!(VGA_WRITER.lock(), "{}", info).unwrap();
    loop {}
}
