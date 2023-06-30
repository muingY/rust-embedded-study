#![no_std]
#![no_main]

use cortex_m_rt::entry;
use fugit::{Duration, ExtU32};
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripheral = pac::Peripherals::take().unwrap();

    let rcc = peripheral.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let gpioa = peripheral.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = peripheral.GPIOC.split();
    let button = gpioc.pc13;

    let mut del_var: Duration<u32, 1, 1000> = 2001.millis();

    led.set_low();

    let mut counter = peripheral.TIM1.counter_ms(&clocks);

    loop {
        counter.start(del_var).unwrap();

        while counter.now().duration_since_epoch() < del_var - 1.millis() {
            if button.is_low() {
                del_var -= 500.millis();

                if del_var.to_millis() < 500_u32 {
                    del_var = 2001.millis();
                }

                break;
            }
        }
        led.toggle();
    }
}
