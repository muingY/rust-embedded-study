#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
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

    let mut gpio_b = device.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpio_b.pb3.into_push_pull_output(&mut gpio_b.moder, &mut gpio_b.otyper);

    let mut timer = Delay::new(core.SYST, clocks);

    loop {
        timer.delay_ms(1000 as u32);
        led.set_high().unwrap();
        timer.delay_ms(1000 as u32);
        led.set_low().unwrap();
    }
}
