use core::f32::consts::TAU;

use defmt::Format;

use crate::consts::SAMPLE_RATE;

pub struct Filter {
    pub cutoff: f32,
    pub resonance: f32,
    pub gain: f32,
    pub z1: f32,
    pub sample_rate: f32,
}

impl Filter {
    pub const fn new() -> Self {
        Self {
            cutoff: 10_000.0,
            gain: 0.0,
            resonance: 0.71,
            sample_rate: SAMPLE_RATE,
            z1: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let dt = 1.0 / self.sample_rate;
        let rc = 1.0 / (TAU * self.cutoff);
        let alpha = dt / (rc + dt);

        self.z1 += alpha * (input - self.z1);
        self.gain * self.z1
    }
}

#[derive(Debug, Format)]
pub enum FilterParam {
    Cutoff,
    Resonance,
    Gain,
}

impl FilterParam {
    pub fn init_param() -> Self {
        Self::Cutoff
    }

    pub fn next_param(param: &Self) -> Option<Self> {
        use FilterParam::*;

        match param {
            Cutoff => Some(Resonance),
            Resonance => Some(Gain),
            Gain => None,
        }
    }
}
