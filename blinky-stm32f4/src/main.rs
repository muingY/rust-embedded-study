#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::Pin,
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = dp.GPIOC.split();
    let button = gpioc.pc13;

    let mut del_var = 7_0000_i32;

    led.set_low();

    loop {
        del_var = loop_delay(del_var, &button);

        led.toggle();
    }
}

fn loop_delay<const P: char, const N: u8>(mut del: i32, but: &Pin<P, N>) -> i32 {
    for _i in 1..del {
        if but.is_low() {
            del = del - 3_0000_i32;
            if del < 1_0000 {
                del = 7_0000_i32;
            }
            return del;
        }
    }
    del
}