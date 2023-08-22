#![no_main]
#![no_std]

pub mod serial_setup;

use core::fmt::Write;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

use serial_setup::UartePort;

// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     let board = microbit::Board::take().unwrap();
//
//     let mut serial = {
//         let serial = uarte::Uarte::new(
//             board.UARTE0,
//             board.uart.into(),
//             Parity::EXCLUDED,
//             Baudrate::BAUD115200,
//         );
//         UartePort::new(serial)
//     };
//
//     writeln!(serial, "This is a test 🤙!").unwrap();
//     nb::block!(serial.flush()).unwrap();
//
//     loop {}
// }

// Receiving a byte
// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     let board = microbit::Board::take().unwrap();
//
//     let mut serial = {
//         let serial = uarte::Uarte::new(
//             board.UARTE0,
//             board.uart.into(),
//             Parity::EXCLUDED,
//             Baudrate::BAUD115200,
//         );
//         UartePort::new(serial)
//     };
//
//     loop {
//         let byte = nb::block!(serial.read()).unwrap();
//         rprintln!("{}", byte as char);
//     }
// }

// Echo server
// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     let board = microbit::Board::take().unwrap();
//
//     let mut serial = {
//         let serial = uarte::Uarte::new(
//             board.UARTE0,
//             board.uart.into(),
//             Parity::EXCLUDED,
//             Baudrate::BAUD115200,
//         );
//         UartePort::new(serial)
//     };
//
//     loop {
//         let byte = nb::block!(serial.read()).unwrap();
//         write!(serial, "Client sent: {}", byte).unwrap();
//         nb::block!(serial.flush()).unwrap();
//     }
// }

// Alright, next let's make the server more interesting by having it respond to the client with the reverse of the text that they sent.
// The server will respond to the client every time they press the ENTER key. Each server response will be in a new line.
#[entry]
fn main() -> ! {
    const ENTER_KEY: u8 = 13;
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut buffer: heapless::Vec<u8, 1024> = heapless::Vec::new();
    loop {
        let byte = nb::block!(serial.read()).unwrap();
        if byte != ENTER_KEY {
            buffer.push(byte).unwrap();
        } else {
            write!(serial, "Reversed client message: ").unwrap();
            for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                nb::block!(serial.write(*byte)).unwrap();
            }
            nb::block!(serial.flush()).unwrap();
            buffer.clear();
        }
    }
}
