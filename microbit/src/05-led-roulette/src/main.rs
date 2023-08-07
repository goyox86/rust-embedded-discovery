#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit as _;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;
    let a = 1;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
