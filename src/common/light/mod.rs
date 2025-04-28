mod ws2812;

use embassy_time::Timer;
pub use ws2812::Rgb;
pub use ws2812::Ws2812;

use embassy_nrf::pwm::Instance;
use embassy_nrf::pwm::{self, Prescaler, SequenceLoad, SequencePwm};
use embassy_nrf::Peri;

pub fn build_ws2812<'d, T: Instance, I: embassy_nrf::gpio::Pin, const NBITSADDONE: usize>(
    pwm: Peri<'d, T>,
    ch0: Peri<'d, I>,
) -> Result<Ws2812<'d, T, NBITSADDONE>, embassy_nrf::pwm::Error> {
    let mut config = pwm::Config::default();
    config.sequence_load = SequenceLoad::Common;
    config.prescaler = Prescaler::Div1;
    config.max_duty = 20; // 1.25us (1s / 16Mhz * 20)

    let pwm = SequencePwm::new_1ch(pwm, ch0, config)?;
    Ok(Ws2812::<T, NBITSADDONE>::new(pwm))
}

#[embassy_executor::task]
pub async fn led_task(
    instance: Peri<'static, embassy_nrf::peripherals::PWM0>,
    pin: Peri<'static, embassy_nrf::peripherals::P0_06>,
) -> ! {
    const NUMBER_OF_LEDS: usize = 35;
    const NUMBER_OF_BITS_PER_LED: usize = 24;
    const TOTAL_BITS_PLUS_ONE: usize = NUMBER_OF_LEDS * NUMBER_OF_BITS_PER_LED + 1;

    let mut strip = build_ws2812::<_, _, TOTAL_BITS_PLUS_ONE>(instance, pin).unwrap();

    loop {
        for brightness in (0..=200).step_by(10) {
            let brightness = brightness as u8;
            strip.fill(Rgb(brightness, brightness, brightness));
            strip.flush().await;
            Timer::after_millis(100).await; // still 100ms minimum
        }

        for brightness in (0..=200).rev().step_by(10) {
            let brightness = brightness as u8;
            strip.fill(Rgb(brightness, brightness, brightness));
            strip.flush().await;
            Timer::after_millis(100).await; // 100ms between each dim step
        }
    }
}
