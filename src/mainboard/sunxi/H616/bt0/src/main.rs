#![no_std]
#![no_main]
#![feature(once_cell_get_mut)]

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
};

use util::mmio::{read32, write32};

#[macro_use]
extern crate log;
mod mem_map;
mod uart;

use mem_map::CCU_BASE;

const STACK_SIZE: usize = 1 * 1024; // 1KiB

#[link_section = ".magic_value"]
#[used]
static MAGIC: u32 = 0xEEEE_EEEE;

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

// see "Port controller" in the manual
const GPIO_BASE: usize = 0x0300_B000;
const GPIO_PORTC_CFG1: usize = GPIO_BASE + 0x004C; // PC8-15
const GPIO_PORTC_DATA: usize = GPIO_BASE + 0x0058;
const GPIO_PORTH_CFG0: usize = GPIO_BASE + 0x00FC;
const GPIO_PORTH_PULL: usize = GPIO_BASE + 0x0118;

const PC13_OUT: u32 = 0b001 << 20;
const PC13_HIGH: u32 = 1 << 13;

const APB2_CFG_REG: usize = CCU_BASE + 0x0524;
const UART_BGR_REG: usize = CCU_BASE + 0x090C;

/// Clear stuff and jump to main.
/// Kudos to Azeria \o/
/// https://azeria-labs.com/memory-instructions-load-and-store-part-4/
/// Xn registers are 64-bit, general purpose; X31 aka Xzr is always 0
/// Wn registers are 32-bit and aliases of lower half of Xn
/// https://linux-sunxi.org/Arm64
///
/// # Safety
///
/// Naked function.
#[unsafe(naked)]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    naked_asm!(
        // 2. initialize programming language runtime
        // clear bss segment
        "ldr     w1, sbss",
        "ldr     w2, ebss",
        "2:",
        // jump out of loop once x2 reaches x1
        "sub     w3, w2, w1",
        "cbz     w3, 2f",
        // clear out the respective address in memory
        "str     w0, [x2], #0",
        "sub     w2, w2, 4",
        "bl      2b",
        "2:",
        // does not init data segment as BT0 runs in sram
        // 3. prepare stack
        "ldr     x1, {stack}",
        "mov     sp, x1",
        "ldr     w1, {stack_size}",
        "add     sp, sp, x1",
        // jump to main :)
        "bl   {main}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        main       =   sym main,
    )
}

fn init_logger(s: uart::SunxiSerial) {
    // This is the new method that also compiles in Rust 2024.
    use core::{cell::OnceCell, ptr::addr_of_mut};
    static mut SERIAL: OnceCell<uart::SunxiSerial> = OnceCell::new();
    unsafe {
        log::init((*addr_of_mut!(SERIAL)).get_mut_or_init(|| s));
    }
}


extern "C" fn main() -> ! {
    let mut ini_pc: usize = 0;
    unsafe { asm!("adr {}, .", out(reg) ini_pc) };
    let mut ini_sp: usize = 0;
    unsafe { asm!("mov {}, sp", out(reg) ini_sp) };

    // System init: select APB@24MHz
    let v = read32(APB2_CFG_REG) & !(0b11 << 24);
    write32(APB2_CFG_REG, v);

    // UART0: TX on port H pin 0, RX on port H pin 1
    let v = read32(GPIO_PORTH_CFG0) & 0xffff_ff00;
    write32(GPIO_PORTH_CFG0, v | (0b010 << 4) | (0b010 << 0));
    let v = read32(GPIO_PORTH_PULL) & 0xffff_fff0;
    write32(GPIO_PORTH_PULL, v | (0b01 << 2) | (0b01 << 0));

    const UART0_GATING: u32 = 1 << 16;
    const UART0_RESET: u32 = 1 << 0;
    // deassert reset
    let v = read32(UART_BGR_REG) & !UART0_GATING;
    write32(UART_BGR_REG, v | UART0_GATING);
    // gating pass
    let v = read32(UART_BGR_REG) & !UART0_RESET;
    write32(UART_BGR_REG, v | UART0_RESET);

    let serial = uart::SunxiSerial::new();
    init_logger(serial);
    println!("oreboot 🦀 in aarch64");
    println!("  program counter (PC): {ini_pc:016x}");
    println!("    stack pointer (SP): {ini_sp:016x}");

    loop {
        unsafe {
            asm!("wfe");
        }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    /*
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
    } else {
        println!("panic at unknown location");
    };
    */
    loop {
        core::hint::spin_loop();
    }
}
