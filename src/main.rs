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

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    hprintln!("Hello, world!").unwrap();

    let p = mk02f12810::Peripherals::take().unwrap();

    let sim = p.SIM;
    let pta = p.PTA;
    let porta = p.PORTA;

    // Enable PORTA clock
    sim.scgc5.write(|w| w.porta().set_bit());

    // Set pin 1 of PORTA to output
    pta.pddr.write(|w| unsafe { w.bits(1_u32 << 1) });

    // Set pin 1 of PORTA mux to GPIO
    porta.pcr[1].modify(|r, w| unsafe { w.bits((r.bits() & !0xF00) | (1_u32 << 8)) });

    // Toggle led
    pta.pdor
        .modify(|r, w| unsafe { w.bits(r.bits() ^ (1_u32 << 1)) });

    loop {
        // Put code here
    }
}
