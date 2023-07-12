#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32l4xx_hal::{
    prelude::*,
    delay::Delay,
};

#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();

    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr);

    let mut gpio_a = device.GPIOA.split(&mut rcc.ahb2);
    let mut gpio_b = device.GPIOB.split(&mut rcc.ahb2);
    let mut gpio_c = device.GPIOC.split(&mut rcc.ahb2);

    let mut led = gpio_b.pb3.into_push_pull_output(&mut gpio_b.moder, &mut gpio_b.otyper);
    let button = gpio_a.pa0.into_pull_down_input(&mut gpio_a.moder, &mut gpio_a.pupdr);
    // let angle_sensor = gpio_a.pa4.into_floating_input(&mut gpio_a.moder, &mut gpio_a.pupdr);
    // let toogle_switch_up = gpio_a.pa5.into_pull_down_input(&mut gpio_a.moder, &mut gpio_a.pupdr);
    // let toogle_switch_down = gpio_a.pa6.into_pull_down_input(&mut gpio_a.moder, &mut gpio_a.pupdr);
    let toggle_switch_up = gpio_c.pc14.into_pull_up_input(&mut gpio_c.moder, &mut gpio_c.pupdr);
    let toggle_switch_down = gpio_c.pc15.into_pull_up_input(&mut gpio_c.moder, &mut gpio_c.pupdr);

    hprintln!("Debug message test.").unwrap();

    let mut timer = Delay::new(core.SYST, clocks);
    // timer.delay_ms(1000u32);
    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        if toggle_switch_up.is_low().unwrap() && toggle_switch_down.is_high().unwrap() {
            hprintln!("Toggle switch is in Position 1").unwrap();
        } else if toggle_switch_up.is_high().unwrap() && toggle_switch_down.is_low().unwrap() {
            hprintln!("Toggle switch is in Position 2").unwrap();
        } else {
            hprintln!("Toggle switch is in OFF position").unwrap();
        }

        timer.delay_ms(500u32);
    }
}
