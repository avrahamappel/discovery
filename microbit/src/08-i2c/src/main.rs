#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use mag3110::Mag3110;
use microbit::{
    hal::{
        prelude::*,
        twi::Twi,
        uart::{Baudrate, Parity},
        Uart,
    },
    pac::{twi0::frequency::FREQUENCY_A, TWI0, UART0},
};
use mma8x5x::Mma8x5x;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

fn print_accel(i2c: Twi<TWI0>, serial: &mut Uart<UART0>) -> Twi<TWI0> {
    let sensor = Mma8x5x::new_mma8653(i2c);
    let mut sensor = sensor.into_active().ok().unwrap();
    match sensor.read() {
        Ok(data) => {
            write!(
                serial,
                "Acceleration: x {} y {} z {}\r\n",
                data.x, data.y, data.z
            )
            .unwrap();
        }
        Err(e) => {
            write!(serial, "Error: {e:?}\r\n").unwrap();
        }
    }
    sensor.destroy()
}

fn print_magn(i2c: Twi<TWI0>, serial: &mut Uart<UART0>) -> Twi<TWI0> {
    let mut sensor = Mag3110::new(i2c).unwrap();
    match sensor.mag() {
        Ok(data) => {
            write!(serial, "Magnet output: {data:?}\r\n").unwrap();
        }
        Err(e) => {
            write!(serial, "Error: {e:?}\r\n").unwrap();
        }
    }
    sensor.destroy()
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100);

    let mut serial = Uart::new(
        board.UART0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    loop {
        let Ok(byte) = nb::block!(serial.read());
        match byte {
            b'a' => {
                i2c = print_accel(i2c, &mut serial);
            }
            b'm' => {
                i2c = print_magn(i2c, &mut serial);
            }
            _ => {}
        }
    }
}
