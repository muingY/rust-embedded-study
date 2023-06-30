#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    // gpio::Pin,
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripheral = pac::Peripherals::take().unwrap();

    let gpioa = peripheral.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = peripheral.GPIOC.split();
    let button = gpioc.pc13;

    led.set_low();

    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
