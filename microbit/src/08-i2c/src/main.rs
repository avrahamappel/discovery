#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

//use microbit::hal::prelude::*;
use mma8x5x::Mma8x5x;

#[cfg(feature = "v1")]
use microbit::{hal::twi, pac::twi0::frequency::FREQUENCY_A};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    let sensor = Mma8x5x::new_mma8653(i2c);
    let mut sensor = sensor.into_active().ok().unwrap();
    loop {
        match sensor.read() {
            Ok(data) => {
                rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
            }
            Err(e) => {
                rprintln!("Error: {:?}", e);
            }
        }
    }
}
