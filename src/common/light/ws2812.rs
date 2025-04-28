use embassy_nrf::pwm::{
    Instance, SequenceConfig, SequencePwm, SingleSequenceMode, SingleSequencer,
};
use embassy_time::Timer;

// Constants for WS2812 pulse timing
const T1H: u16 = 0x8000 | 13; // 0.8 µs
const T0H: u16 = 0x8000 | 7; // 0.4 µs
const RES: u16 = 0x8000; // Reset pulse (low)

#[derive(Copy, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

pub struct Ws2812<'a, T: Instance, const N: usize> {
    pwm: SequencePwm<'a, T>,
    buffer: [u16; N],
    seq_config: SequenceConfig,
}

impl<'a, T: Instance, const N: usize> Ws2812<'a, T, N> {
    pub fn new(pwm: SequencePwm<'a, T>) -> Self {
        let mut seq_config = SequenceConfig::default();
        seq_config.end_delay = 799;

        Self {
            pwm,
            buffer: [RES; N],
            seq_config,
        }
    }

    #[allow(dead_code)]
    // Ensure we encode the correct number of LEDs based on `N`
    fn encode_rgb(&mut self, colors: &[Rgb]) {
        let num_leds = (N - 1) / 24; // This calculates the number of LEDs based on the total buffer size.

        // Iterate over all LEDs
        for (i, Rgb(r, g, b)) in colors.iter().enumerate().take(num_leds) {
            let start = i * 24;
            self.encode_byte(start, *g);
            self.encode_byte(start + 8, *r);
            self.encode_byte(start + 16, *b);
        }

        self.buffer[N - 1] = RES; // End-of-frame with reset pulse
    }

    // Encode a single byte (8 bits) for each LED
    fn encode_byte(&mut self, start: usize, byte: u8) {
        for bit in 0..8 {
            let mask = 1 << (7 - bit);
            self.buffer[start + bit] = if byte & mask != 0 { T1H } else { T0H };
        }
    }

    // Fill the buffer with the same color for all LEDs
    pub fn fill(&mut self, color: Rgb) {
        let num_leds = (N - 1) / 24;
        for i in 0..num_leds {
            let start = i * 24;
            self.encode_byte(start, color.1);
            self.encode_byte(start + 8, color.0);
            self.encode_byte(start + 16, color.2);
        }
        self.buffer[N - 1] = RES; // End-of-frame with reset pulse
    }

    pub async fn flush(&mut self) {
        let num_leds = (N - 1) / 24;
        let sequences = SingleSequencer::new(&mut self.pwm, &self.buffer, self.seq_config.clone());
        sequences.start(SingleSequenceMode::Times(1)).unwrap();

        // Total time for all LEDs including reset pulse
        let total_duration = (num_leds as u64 * 30) + 80; // ~30us per LED
        Timer::after_micros(total_duration).await;
    }
}
