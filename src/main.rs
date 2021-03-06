#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

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

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    let half_period = 50_u16;

    loop {
        for led in 0..8 {
            leds[led].on();
            delay.delay_ms(half_period);
        }

        for led in 0..8 {
            leds[led].off();
            delay.delay_ms(half_period);
        }
    }
}