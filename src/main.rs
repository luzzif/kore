#![cfg_attr(not(test), no_main)]
#![no_std]
#![feature(panic_handler)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;
mod vga;
use core::panic::PanicInfo;
use core::fmt::Write;
use vga::writer::VGA_WRITER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(VGA_WRITER.lock(), "Hello world!");
    loop {}
}


#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    write!(VGA_WRITER.lock(), "{}", info).unwrap();
    loop {}
}
