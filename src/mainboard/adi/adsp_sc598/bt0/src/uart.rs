use embedded_hal_nb::nb;
use embedded_hal_nb::nb::block;
use log::Serial;
use embedded_hal_nb::serial::{Error as _, ErrorType, Write, Read};
use core::ptr::{write_volatile, read_volatile};
use core::fmt;

//TODO: Probably move the baremetal talking to a PAC

/*
 * This is based off of earlycon support. We need to enable 
 * baud rate control to setup the serial port correctly.
 * */

const SCLK0: u32 = 25000000; // Select SCLK0 as the clock source

const THRE: u8 = 1 << 5; // Transmit Holding Register Empty
const DR: u8 = 1 << 0; // Data Ready

const UEN: u32 = 0x01; // UART Enable Register
const UMOD_UART: u32 = 0x0 << 4; // UART Enable Register
const WLS_8: u32 = 0x3 << 8; // Word Length Select (8 bits)

const UART_BASE: usize = 0x31003000;

const REVID:      usize = UART_BASE + 0x00;
const CONTROL:    usize = UART_BASE + 0x04;
const STATUS:     usize = UART_BASE + 0x08;
const SCR:        usize = UART_BASE + 0x0C;
const CLOCK:      usize = UART_BASE + 0x10;
const EMASK:      usize = UART_BASE + 0x14;
const EMASKST:    usize = UART_BASE + 0x18;
const EMASKCL:    usize = UART_BASE + 0x1C;
const RBR:        usize = UART_BASE + 0x20;
const THR:        usize = UART_BASE + 0x24;
const TAIP:       usize = UART_BASE + 0x28;
const TSR:        usize = UART_BASE + 0x2C;
const RSR:        usize = UART_BASE + 0x30;
const TXDIV_CNT:  usize = UART_BASE + 0x34;
const RXDIV_CNT:  usize = UART_BASE + 0x38;

pub struct adi_uart {}

fn write_8 (addr: usize, value: u8) {
    unsafe { write_volatile(addr as *mut u8, value) };
}

fn read_8 (addr: usize) -> u8 {
    unsafe { read_volatile(addr as *const u8) }
}

fn write_32 (addr: usize, value: u32) {
    unsafe { write_volatile(addr as *mut u32, value) };
}

fn read_32 (addr: usize) -> u32 {
    unsafe { read_volatile(addr as *const u32) }
}

impl adi_uart {
    pub fn new() -> Self {
        adi_uart {}
    }

    pub fn init(&self, baud_rate: u32) {
        let mut divisor: u32 = 0; // Placeholder for actual divisor calculation based on baud rate
                         
        divisor = ((SCLK0 / (baud_rate/2)) / baud_rate) & 0xFFFF;

        write_32(CONTROL, UEN| UMOD_UART | WLS_8);
        write_32(STATUS, u32::MAX); 
        write_32(CLOCK, divisor); //set clock

        //set baud rate
        ;

    }

    pub fn ready(&self, tx: bool) -> bool {
        
        if tx {
            read_8(STATUS) & THRE != 0
        } else {
            read_8(STATUS) & DR != 0
        }
    }

    pub fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for &byte in s.as_bytes() {
            // Inject a carriage return before a newline
            if byte == b'\n' {
                block!(self.write(b'\r')).ok();
            }
            block!(self.write(byte)).ok();
        }
        block!(self.flush()).ok();
        Ok(())
    }

}

impl Serial for adi_uart {
}


impl ErrorType for adi_uart {
    type Error = log::Error;
}

impl Write for adi_uart {

    #[unsafe(no_mangle)]
    fn write(&mut self, c: u8) -> nb::Result<(), Self::Error> {
        if !self.ready(true) { 
            return Err(nb::Error::WouldBlock);
        };

        write_8(THR, c);
        Ok(())
    }
    
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
       
        let tfe_empty = true;
        if tfe_empty {
            return Ok(());
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

