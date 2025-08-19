use core::ptr::{write_volatile, read_volatile};

pub const SCLK0: u32 = 25000000; // Select SCLK0 as the clock source


//TODO read/write can be templated for different types
pub fn write_8 (addr: usize, value: u8) {
    unsafe { write_volatile(addr as *mut u8, value) };
}

pub fn read_8 (addr: usize) -> u8 {
    unsafe { read_volatile(addr as *const u8) }
}

pub fn write_16 (addr: usize, value: u16) {
    unsafe { write_volatile(addr as *mut u16, value) };
}

pub fn read_16 (addr: usize) -> u16 {
    unsafe { read_volatile(addr as *const u16) }
}

pub fn write_32 (addr: usize, value: u32) {
    unsafe { write_volatile(addr as *mut u32, value) };
}

pub fn read_32 (addr: usize) -> u32 {
    unsafe { read_volatile(addr as *const u32) }
}
