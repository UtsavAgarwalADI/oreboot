#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[inline(always)]
pub const fn clk_low(x: u16) -> u16 {
    x & 0x00FF
}

#[inline(always)]
pub const fn clk_hi(y: u16) -> u16 {
    (y & 0x00FF) << 8
}

#[inline(always)]
pub const fn speed_to_duty_cycle(speed: u32) -> u16 {
    (5_000_000 / speed) as u16
}

pub mod clk_speed {
    use super::speed_to_duty_cycle;

    pub const SPEED_MAX: u32 = 400_000;

    pub const DUTY_MAX: u16 = speed_to_duty_cycle(SPEED_MAX) + 1;
    pub const DUTY_MIN: u16 = 0xff;
}

/// Global/clock-and-mode related bits (exact register name depends on the SoC)
pub mod clk_mode {
    /// SCLKs per internal time reference (10 MHz)
    pub const PRESCALE: u16 = 0x007F;
    /// TWI enable
    pub const TWI_ENA: u16 = 0x0080;
    /// SCCB compatibility enable
    pub const SCCB:   u16 = 0x0200;
}

/// Slave control (enable/addressing/behavior)
pub mod slave_ctl {
    /// Slave enable
    pub const SEN:       u16 = 0x0001;
    /// Slave address length
    pub const SADD_LEN:  u16 = 0x0002;
    /// Slave transmit data valid
    pub const STDVAL:    u16 = 0x0004;
    /// NAK generated at conclusion of transfer
    pub const TSC_NAK:   u16 = 0x0008;
    /// General call address matching enabled
    pub const GEN:       u16 = 0x0010;
}

/// Slave status/flags
pub mod slave_stat {
    /// Slave transfer direction
    pub const SDIR:   u16 = 0x0001;
    /// General call indicator
    pub const GCALL:  u16 = 0x0002;
}

/// Master control (enable/addressing/flow)
pub mod master_ctl {
    /// Master mode enable
    pub const MEN:       u16 = 0x0001;
    /// Master address length
    pub const MADD_LEN:  u16 = 0x0002;
    /// Master transmit direction (RX/TX*)
    pub const MDIR:      u16 = 0x0004;
    /// Use Fast-mode timing
    pub const FAST:      u16 = 0x0008;
    /// Issue STOP condition
    pub const STOP:      u16 = 0x0010;
    /// Repeat START or STOP* at end of transfer
    pub const RSTART:    u16 = 0x0020;
    /// Data byte-count 
    pub const BITP_DCNT:      u16 = 0x6;
    /// Data byte-count mask (bytes to transfer)
    pub const MSK_DCNT:      u16 = 0x3FC0;
    /// Serial data override
    pub const SDAOVR:    u16 = 0x4000;
    /// Serial clock override
    pub const SCLOVR:    u16 = 0x8000;
}

/// Master status
pub mod master_stat {
    /// Master transfer in progress
    pub const MPROG:    u16 = 0x0001;
    /// Lost arbitration (transfer aborted)
    pub const LOSTARB:  u16 = 0x0002;
    /// Address not acknowledged
    pub const ANAK:     u16 = 0x0004;
    /// Data not acknowledged
    pub const DNAK:     u16 = 0x0008;
    /// Buffer read error
    pub const BUFRDERR: u16 = 0x0010;
    /// Buffer write error
    pub const BUFWRERR: u16 = 0x0020;
    /// Serial data sense
    pub const SDASEN:   u16 = 0x0040;
    /// Serial clock sense
    pub const SCLSEN:   u16 = 0x0080;
    /// Bus busy indicator
    pub const BUSBUSY:  u16 = 0x0100;
}

/// Interrupt/status flags
pub mod int_stat {
    /// Slave transfer initiated
    pub const SINIT:   u16 = 0x0001;
    /// Slave transfer complete
    pub const SCOMP:   u16 = 0x0002;
    /// Slave transfer error
    pub const SERR:    u16 = 0x0004;
    /// Slave overflow
    pub const SOVF:    u16 = 0x0008;
    /// Master transfer complete
    pub const MCOMP:   u16 = 0x0010;
    /// Master transfer error
    pub const MERR:    u16 = 0x0020;
    /// Transmit FIFO service
    pub const XMTSERV: u16 = 0x0040;
    /// Receive FIFO service
    pub const RCVSERV: u16 = 0x0080;
}

/// FIFO control
pub mod fifo_ctl {
    /// Transmit buffer flush
    pub const XMTFLUSH:  u16 = 0x0001;
    /// Receive buffer flush
    pub const RCVFLUSH:  u16 = 0x0002;
    /// Transmit buffer interrupt length
    pub const XMTINTLEN: u16 = 0x0004;
    /// Receive buffer interrupt length
    pub const RCVINTLEN: u16 = 0x0008;
}

/// FIFO status + enumerations for convenience
pub mod fifo_stat {
    /// Transmit FIFO status mask
    pub const XMTSTAT:   u16 = 0x0003;
    /// Transmit FIFO empty
    pub const XMT_EMPTY: u16 = 0x0000;
    /// Transmit FIFO has 1 byte to write
    pub const XMT_HALF:  u16 = 0x0001;
    /// Transmit FIFO full (2 bytes to write)
    pub const XMT_FULL:  u16 = 0x0003;

    /// Receive FIFO status mask
    pub const RCVSTAT:   u16 = 0x000C;
    /// Receive FIFO empty
    pub const RCV_EMPTY: u16 = 0x0000;
    /// Receive FIFO has 1 byte to read
    pub const RCV_HALF:  u16 = 0x0004;
    /// Receive FIFO full (2 bytes to read)
    pub const RCV_FULL:  u16 = 0x000C;
}

// -------------------------
// Example usage (optional):
// -------------------------
//
// let timing = clk_low(0x30) | clk_hi(0x30); // compose low/high periods
// let en = clk_mode::TWI_ENA | clk_mode::SCCB;
// let mctl = master_ctl::MEN | master_ctl::FAST | master_ctl::STOP;

//i2c2
pub const BASE:        usize = 0x3100_1600; // Base address for the I2C controller

pub const CLKDIV:      usize = BASE + 0x00;
pub const CONTROL:     usize = BASE + 0x04;
pub const SLAVE_CTL:   usize = BASE + 0x08;
pub const SLAVE_STAT:  usize = BASE + 0x0C;
pub const SLAVE_ADDR:  usize = BASE + 0x10;
pub const MASTER_CTL:  usize = BASE + 0x14;
pub const MASTER_STAT: usize = BASE + 0x18;
pub const MASTER_ADDR: usize = BASE + 0x1C;
pub const INT_STAT:    usize = BASE + 0x20;
pub const INT_MASK:    usize = BASE + 0x24;
pub const FIFO_CTL:    usize = BASE + 0x28;
pub const FIFO_STAT:   usize = BASE + 0x2C;

pub const XMT_DATA8:   usize = BASE + 0x80;
pub const XMT_DATA16:  usize = BASE + 0x84;
pub const RCV_DATA8:   usize = BASE + 0x88;
pub const RCV_DATA16:  usize = BASE + 0x8C;





