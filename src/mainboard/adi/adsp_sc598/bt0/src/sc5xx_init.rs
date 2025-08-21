use crate::sc5xx_pac::*;

const SMPUS: [usize; 9] = [
    0x31007800, // SMPU0
    0x31083800, // SMPU2
    0x31084800, // SMPU3
    0x31085800, // SMPU4
    0x31086800, // SMPU5
    0x31087800, // SMPU6
    0x310A0800, // SMPU9
    0x310A1800, // SMPU11
    0x31012800, // SMPU12
];

const SPU0: usize = 0x3108BA00;
const SPU0_WP: usize = 0x3108B400;

fn disable_spu() {
    for i in 0..214 {
        write_32((SPU0 + i * 4), 0);
    }
}

fn disable_spu_wp() {
    for i in 0..214 {
        write_32((SPU0_WP + i * 4), 0);
    }
}

fn set_smpus() {
    for smpu in SMPUS {
        write_32(smpu, 0x500);
    }
}

// configures SPU and SMPU to allow peripheral access
pub fn init_sc59x_peripheral_access() {
    disable_spu();
    disable_spu_wp();
    set_smpus();
}

pub fn disable_board_leds() {
    // Set LEDs 7,9 and 10

    let led_config1 = read_32(0x3100411C);
    let led_config2 = read_32(0x3100410C);

    write_32(0x3100411C, led_config1 & !0xE);
    write_32(0x3100410C, led_config2 & !0xE);
}

pub fn enable_board_leds() {
    // Set LEDs 7,9 and 10
    write_32(0x3100411C, 0xE);
    write_32(0x3100410C, 0xE);
}
