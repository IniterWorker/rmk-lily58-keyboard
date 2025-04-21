use embassy_nrf::peripherals::PWM0;
use embassy_nrf::pwm::{SequenceConfig, SequencePwm, SingleSequenceMode, SingleSequencer};
use embassy_time::Timer;

// Constants for WS2812 pulse timing
const T1H: u16 = 0x8000 | 13; // 0.8 µs
const T0H: u16 = 0x8000 | 7; // 0.4 µs
const RES: u16 = 0x8000; // Reset pulse (low)

#[derive(Copy, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// `N` = number of LEDs  
/// `N24` = `N * 24`  
/// `LEN` = `N * 24 + 1`
pub struct Ws2812<'a, const N: usize, const N24: usize, const LEN: usize> {
    pwm: SequencePwm<'a, PWM0>,
    buffer: [u16; LEN],
    seq_config: SequenceConfig,
}

impl<'a, const N: usize, const N24: usize, const LEN: usize> Ws2812<'a, N, N24, LEN> {
    pub fn new(pwm: SequencePwm<'a, PWM0>) -> Self {
        let mut seq_config = SequenceConfig::default();
        seq_config.end_delay = 799;

        Self {
            pwm,
            buffer: [RES; LEN],
            seq_config,
        }
    }

    fn encode_rgb(&mut self, colors: &[Rgb]) {
        for (i, Rgb(r, g, b)) in colors.iter().enumerate().take(N) {
            let start = i * 24;
            self.encode_byte(start, *g);
            self.encode_byte(start + 8, *r);
            self.encode_byte(start + 16, *b);
        }

        self.buffer[N24] = RES; // End-of-frame
    }

    fn encode_byte(&mut self, start: usize, byte: u8) {
        for bit in 0..8 {
            let mask = 1 << (7 - bit);
            self.buffer[start + bit] = if byte & mask != 0 { T1H } else { T0H };
        }
    }

    pub async fn show(&mut self, colors: &[Rgb]) {
        self.encode_rgb(colors);

        let sequences = SingleSequencer::new(&mut self.pwm, &self.buffer, self.seq_config.clone());
        sequences.start(SingleSequenceMode::Times(1)).unwrap();

        let total_duration = N as u64 * 30 + 80;
        Timer::after_micros(total_duration).await;
    }
}
