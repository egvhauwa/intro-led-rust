#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use mk02f12810;
// use mk02f12810::{PIT, PORTA, PTA};

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    hprintln!("Hello, world!").unwrap();

    // let cp = cortex_m::Peripherals::take().unwrap(); // This will work only once!

    let peripherals = mk02f12810::Peripherals::take().unwrap();

    let pta = peripherals.PTA;
    // pta.pddr.write(f)

    // let peri = mk02f12810::Peripherals::take().unwrap();

    loop {
        // your code goes here
    }
}