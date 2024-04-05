#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32l4xx_hal::{
    adc::ADC, delay::Delay, prelude::*
};

#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr, &mut pwr);
    let mut timer = Delay::new(core.SYST, clocks);

    // ADC set
    let mut adc = ADC::new(device.ADC1, device.ADC_COMMON, &mut rcc.ahb2, &mut rcc.ccipr, &mut timer);

    // GPIOs set
    let mut gpio_a = device.GPIOA.split(&mut rcc.ahb2);
    let mut gpio_b = device.GPIOB.split(&mut rcc.ahb2);
    let mut gpio_c = device.GPIOC.split(&mut rcc.ahb2);

    let mut led = gpio_b.pb3.into_push_pull_output(&mut gpio_b.moder, &mut gpio_b.otyper);
    let button = gpio_b.pb4.into_pull_up_input(&mut gpio_b.moder, &mut gpio_b.pupdr);
    let mut potentiometer = gpio_a.pa0.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);

    // Main loop
    let mut button_state = button.is_high();
    let mut potentiometer_value: f32;
    loop {
        let new_state = button.is_high();
        if new_state != button_state {
            // 디바운싱을 위한 작은 지연
            timer.delay_ms(10u32);

            // 지연 후 상태를 업데이트하여 버튼 상태가 안정적인지 확인
            button_state = button.is_high();
            if new_state != button_state {
                continue; // 지연 동안 상태가 변경된 경우 건너뜀
            }

            if button_state {
                led.set_low();
                hprintln!("Button Released");
            } else {
                led.set_high();
                hprintln!("Button Pressed");
            }
        }

        if !button_state {
            potentiometer_value = (adc.read(&mut potentiometer).unwrap()) as f32 / 4095.0;
            hprintln!("potentiometer: {}", potentiometer_value);
        }
    }
}