#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use debouncr::{debounce_3, Edge};
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::{Config}
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

    let tx_pin = gpioa.pa2.into_alternate();
    let mut tx = peripheral
        .USART2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(), 
            &clocks,
        ).unwrap();

    let mut del_var = 7_0000_i32;

    led.set_low();

    let mut debouncer = debounce_3(false);

    let mut value: u8 = 0;

    loop {
        for _i in 1..del_var {
            if debouncer.update(button.is_low()) == Some(Edge::Falling) {
                writeln!(tx, "Button Press {:02}\r", value).unwrap();
                value = value.wrapping_add(1);
                del_var = del_var - 3_0000_i32;
                if del_var < 1_0000 {
                    del_var = 7_0000_i32;
                }
                break;
            }
        }

        led.toggle();
    }
}
