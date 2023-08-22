// #![deny(unsafe_code)C
#![no_main]
#![no_std]

mod serial_setup;

use cortex_m_rt::entry;
// use panic_rtt_target as _;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::hal::prelude::*;

use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

const ACCELEROMETER_ADDR: u8 = 0x19;
const MAGNETOMETER_ADDR: u8 = 0x1E;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main() -> ! {
    // rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut acc = [0];
    let mut mag = [0];
    // First write the address + register onto the bus, then read the chip's responses
    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc)
        .unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag)
        .unwrap();

    // rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    // rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);

    loop {}
}
