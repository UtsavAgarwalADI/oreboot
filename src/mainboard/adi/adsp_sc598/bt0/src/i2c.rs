use embedded_hal::i2c::{self, SevenBitAddress, I2c, Operation};
use crate::i2c_regs::*;
use crate::sc5xx_pac::*;

pub struct adi_twi_i2c;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    BusBusy,
    Timeout,
}

pub enum msg_flags {
    COMBO, // Combined read/write
    STOP, // Issue STOP condition
    READ,  // Read operation
}

impl i2c::ErrorType for adi_twi_i2c {
    type Error = Error;
}

impl i2c::Error for Error {
    fn kind(&self) -> i2c::ErrorKind {
        match self {
            Error::BusBusy => i2c::ErrorKind::Bus,
            Error::Timeout => i2c::ErrorKind::Other,
        }
    }
}

impl adi_twi_i2c {
    pub fn new() -> Self {
        Self::init();
        adi_twi_i2c
    }

    fn set_bus_speed(speed: u32) {
        let mut clock_div = speed_to_duty_cycle(speed);

        if (clock_div > clk_speed::DUTY_MAX) || (clock_div < clk_speed::DUTY_MIN) {
            panic!("Invalid I2C speed");
        }

        // same internal and 10Mhz ref points
        clock_div = (clock_div << 8) | (clock_div & 0xFF);
        write_16(CLKDIV, clock_div);
        write_16(CLKDIV, clock_div);

        if speed > 100_000 {
            write_16(CONTROL, master_ctl::FAST as u16);
        } else {
            write_16(CONTROL, 0);
        }
         
    }

    //taken from uboot spl
    pub fn init() {
        // Initialize adi_twi_i2c controller set up registers, etc.
        let prescale = (((SCLK0 / 1000 / 1000 + 5) / 10) as u16 & clk_mode::PRESCALE) as u16; 
        
        write_16(CONTROL, prescale);
        Self::set_bus_speed(clk_speed::SPEED_MAX);

        write_16(CONTROL, clk_mode::TWI_ENA | prescale);
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        // turn off adi_twi_i2c controller
        Ok(())
    }
}

impl I2c<SevenBitAddress> for adi_twi_i2c {
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
       Ok(()) 
    }
    
}
