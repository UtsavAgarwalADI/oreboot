use crate::i2c_regs::*;
use crate::sc5xx_pac::*;
use embedded_hal::i2c::{self, I2c, Operation, SevenBitAddress};

pub struct adi_twi_i2c;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    BusBusy,
    MemErr,
    Timeout,
}

pub enum msg_flags {
    COMBO, // Combined read/write
    STOP,  // Issue STOP condition
    READ,  // Read operation
}

impl i2c::ErrorType for adi_twi_i2c {
    type Error = Error;
}

impl i2c::Error for Error {
    fn kind(&self) -> i2c::ErrorKind {
        match self {
            Error::BusBusy => i2c::ErrorKind::Bus,
            Error::MemErr => i2c::ErrorKind::Other,
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

        // same internal and 10Mhz ref points
        clock_div = (clock_div << 8) | (clock_div & 0xFF);
        write_16(CLKDIV, clock_div);
        write_16(CLKDIV, clock_div);

        //enable all interrupts
        write_16(INT_STAT, 0);

        if speed > 100_000 {
            write_16(CONTROL, master_ctl::FAST as u16);
        } else {
            write_16(CONTROL, 0);
        }
    }

    #[inline(always)]
    pub fn check_bus_busy() -> bool {
        (read_16(MASTER_STAT) & master_stat::BUSBUSY) != 0
    }

    //taken from uboot spl
    pub fn init() {
        // Initialize adi_twi_i2c controller set up registers, etc.
        let prescale = (((SCLK0 / 1000 / 1000 + 5) / 10) as u16 & clk_mode::PRESCALE) as u16;

        write_16(CONTROL, prescale);
        Self::set_bus_speed(clk_speed::SPEED_MAX);
        write_16(CONTROL, clk_mode::TWI_ENA | prescale);
    }

    pub fn stop() -> Result<(), Error> {
        // turn off adi_twi_i2c controller
        write_16(CONTROL, 0);
        Ok(())
    }
}

impl I2c<SevenBitAddress> for adi_twi_i2c {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        if operations.is_empty() {
            return Ok(());
        }

        Self::init();
        while (Self::check_bus_busy()) {
            // Wait for bus to be free
        }

        write_16(MASTER_ADDR, address as u16);

        //clear fifo
        write_16(FIFO_CTL, fifo_ctl::XMTFLUSH | fifo_ctl::RCVFLUSH);
        write_16(FIFO_CTL, 0);

        //clear stat
        write_16(MASTER_STAT, u16::MAX);
        write_16(INT_STAT, u16::MAX);
        write_16(INT_MASK, 0);

        //enable master
        let mut ctl = read_16(MASTER_CTL);

        // set transfer mode
        ctl = (ctl & master_ctl::FAST);
        // set data count
        ctl |= ((operations.len() as u16) << master_ctl::BITP_DCNT);

        for op in operations.iter_mut() {
            loop {
                let int_stat_val = read_16(INT_STAT);
                match op {
                    Operation::Write(data) => {
                        for &byte in data.iter() {
                            while (Self::check_bus_busy()) {
                                // Wait for bus to be free
                            }
                            write_8(XMT_DATA8, byte);
                        }
                    }

                    Operation::Read(buffer) => {
                        for byte in buffer.iter_mut() {
                            while (Self::check_bus_busy()) {
                                // Wait for bus to be free
                            }
                            *byte = read_8(RCV_DATA8);
                        }
                    }
                }

                // Check for errors
                if (int_stat_val & int_stat::MERR) != 0 {
                    return Err(Error::MemErr);
                }

                if (int_stat_val & int_stat::MCOMP) == 0 {
                    continue; // Wait for completion
                }

                write_16(INT_STAT, int_stat::MCOMP); // Clear completion status
                break;
            }
        }

        Self::stop();
        Ok(())
    }
}
