#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m::iprintln;
use cortex_m_rt::entry;
use f3::{
    hal::{delay::Delay, prelude},
    led::Leds,
};

use f3::hal::{prelude::*, stm32f30x};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    
    let mut itm = cp.ITM;
    iprintln!(&mut itm.stim[0], "Yo bro!!");

    loop {

    }
}