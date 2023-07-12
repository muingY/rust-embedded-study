#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::{Config},
    adc::{Adc, config::AdcConfig, config::SampleTime},
};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let mut adc = Adc::adc1(device.ADC1, true, AdcConfig::default());
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    let mut timer = device.TIM1.delay_ms(&clocks);

    let gpio_a = device.GPIOA.split();
    let gpio_b = device.GPIOB.split();
    let gpio_c = device.GPIOC.split();

    let tx_pin = gpio_a.pa2.into_alternate();
    let mut tx = device
        .USART2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        ).unwrap();

    let mut led = gpio_a.pa5.into_push_pull_output();
    let toggle_switch_up = gpio_b.pb3.into_pull_up_input();
    let toggle_switch_down = gpio_b.pb5.into_pull_up_input();
    let angle_sensor = gpio_c.pc0.into_analog(); // AN25-Analog

    led.set_low();

    loop {
        if toggle_switch_up.is_low() && toggle_switch_down.is_high() {
            writeln!(tx, "Toggle switch is in Position 1").unwrap();
            led.set_high();
        } else if toggle_switch_up.is_high() && toggle_switch_down.is_low() {
            writeln!(tx, "Toggle switch is in Position 2").unwrap();
            led.set_high();
        } else {
            writeln!(tx, "Toggle switch is in OFF position").unwrap();
            led.set_low();
        }

        writeln!(tx, "Angle sensor result: {}", adc.convert(&angle_sensor, SampleTime::Cycles_84)).unwrap();

        // Remove later.
        timer.delay_ms(200u32);
    }
}
