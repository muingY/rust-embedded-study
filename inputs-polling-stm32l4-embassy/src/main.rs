#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{adc::Adc, gpio::{Input, Level, Output, Pull, Speed}};
use embassy_time::{Timer, Delay};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    
    let mut adc = Adc::new(p.ADC1, &mut Delay);

    // GPIOs set
    let mut led = Output::new(p.PB3, Level::Low, Speed::Low);
    let button = Input::new(p.PB4, Pull::Up);
    let mut potentiometer = p.PA0;

    let mut button_state = true;
    let mut potentiometer_value: f32;
    // Loop
    loop {
        let new_state = button.is_high();
        if new_state != button_state {
            // 디바운싱을 위한 작은 지연
            Timer::after_millis(10).await;

            // 지연 후 상태를 업데이트하여 버튼 상태가 안정적인지 확인
            button_state = button.is_high();
            if new_state != button_state {
                continue; // 지연 동안 상태가 변경된 경우 건너뜀
            }

            if button_state {
                led.set_low();
                info!("button released");
            } else {
                led.set_high();
                info!("button pressed");
            }
        }
        if !button_state {
            potentiometer_value = adc.read(&mut potentiometer) as f32 / 4095.0;
            info!("potentiometer: {}", potentiometer_value);
        }
    }
}