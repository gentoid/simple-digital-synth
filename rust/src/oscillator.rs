use core::f32::consts::TAU;

use libm::sinf;

use crate::consts::{MAX_DAC_VALUE, SAMPLE_RATE};

pub struct Oscillator {
    freq: f32,
    phase: f32,
    sample_rate: f32,
    phase_inc: f32,
}

impl Oscillator {
    pub fn new(freq: f32) -> Self {
        let mut this = Self {
            freq,
            phase: 0.0,
            sample_rate: SAMPLE_RATE,
            phase_inc: 0.0,
        };

        this.update_phase_inc();

        this
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq;
        self.update_phase_inc();
    }

    pub fn next_sample(&mut self) -> u16 {
        let sample = MAX_DAC_VALUE as f32 * (sinf(self.phase) + 1.0) / 2.0;
        self.phase += self.phase_inc;

        if self.phase >= TAU {
            self.phase -= TAU;
        }

        sample as u16
    }

    fn update_phase_inc(&mut self) {
        self.phase_inc = self.freq * TAU / self.sample_rate;
    }
}
