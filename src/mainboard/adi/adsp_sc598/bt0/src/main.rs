#![no_std]
#![no_main]

mod uart;
mod i2c;
mod i2c_regs;
mod sc5xx_pac;

use aarch64_cpu::asm;
use embedded_hal_nb::serial::{Read, Write, ErrorType};
use log::{print, println};
use uart::adi_uart;

fn init_logger(s: adi_uart) {
        static mut SERIAL: Option<adi_uart> = None;
        SERIAL.replace(s);
        log::init(SERIAL.as_mut().unwrap());
    }
}

fn block_write(s: &mut adi_uart, byte: u8) -> () {
    s.write(byte);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let mut uart = adi_uart::new();
    
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
