#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{adc::Adc, exti::ExtiInput, gpio::{Input, Pull}};
use embassy_time::Delay;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1, &mut Delay);

    // GPIOs set
    let mut button = ExtiInput::new(Input::new(p.PB4, Pull::Up), p.EXTI4);
    let mut potentiometer = p.PA0;

    loop {
        button.wait_for_falling_edge().await;
        info!("Pressed!");
        let potentiometer_value = adc.read(&mut potentiometer) as f32 / 4095.0;
        info!("potentiometer: {}", potentiometer_value);

        button.wait_for_rising_edge().await;
        info!("Released!");
    }
}
