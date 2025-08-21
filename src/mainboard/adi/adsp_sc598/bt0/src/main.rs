#![no_std]
#![no_main]

mod i2c;
mod i2c_regs;
mod sc5xx_init;
mod sc5xx_pac;
mod uart;

use crate::sc5xx_pac::*;
use aarch64_cpu::asm;
use core::cell::RefCell;
use embedded_hal_bus::i2c as i2c_bus;
use embedded_hal_nb::serial::{ErrorType, Read, Write};
use i2c::adi_twi_i2c;
use log::{print, println};
use mcp23017_tp::prelude::*;
use uart::adi_uart;

fn block_write(s: &mut adi_uart, byte: u8) -> () {
    s.write(byte);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    sc5xx_init::enable_board_leds();
    sc5xx_init::init_sc59x_peripheral_access();
    sc5xx_init::disable_board_leds();

    let mut uart = adi_uart::new();
    let i2c = adi_twi_i2c::new();

    sc5xx_init::enable_board_leds();
    let i2c_ref_cell = RefCell::new(i2c);
    let address = 0x22;

    let mut pina1 = mcp23017_tp::Pina1::new(i2c_bus::RefCellDevice::new(&i2c_ref_cell), address)
        .set_as_output();

    let mut pinb3 = mcp23017_tp::Pinb3::new(i2c_bus::RefCellDevice::new(&i2c_ref_cell), address)
        .set_as_output();

    sc5xx_init::disable_board_leds();

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
