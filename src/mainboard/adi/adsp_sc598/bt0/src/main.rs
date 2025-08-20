#![no_std]
#![no_main]

mod i2c;
mod i2c_regs;
mod sc5xx_pac;
mod uart;

use crate::sc5xx_pac::*;
use aarch64_cpu::asm;
use embedded_hal_nb::serial::{ErrorType, Read, Write};
use i2c::adi_twi_i2c;
use log::{print, println};
use uart::adi_uart;

fn block_write(s: &mut adi_uart, byte: u8) -> () {
    s.write(byte);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let mut uart = adi_uart::new();
    let i2c = adi_twi_i2c::new();

    adi_twi_i2c::init();

    uart.init(115200);
    uart.write_str("Hello, oreboot!\n").ok();

    loop {
        asm::wfe(); // Wait for event
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
