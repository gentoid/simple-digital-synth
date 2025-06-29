use core::f32::consts::TAU;

use defmt::{Format, info};

use crate::{consts::SAMPLE_RATE, encoder::Rotation};

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
            gain: 1.0,
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

    pub fn adjust(&mut self, param: &FilterParam, rotation: Rotation) {
        match param {
            FilterParam::Cutoff => {
                let delta = if rotation == Rotation::Right {
                    1.01
                } else {
                    1.0 / 1.01
                };
                self.cutoff = delta * self.cutoff;
                info!("Set filter cutoff: {}", self.cutoff);
            }
            FilterParam::Resonance => {
                if rotation == Rotation::Right {
                    self.resonance += 0.05;
                } else {
                    self.resonance -= 0.05;
                };
                info!("Set filter resonance: {}", self.resonance);
            }
            FilterParam::Gain => {
                if rotation == Rotation::Right {
                    self.gain += 0.05;
                } else {
                    self.gain -= 0.05;
                };
                info!("Set filter gain: {}", self.gain);
            }
        }
    }
}

#[derive(Debug, Format)]
pub enum FilterParam {
    Cutoff,
    Resonance,
    Gain,
}

impl FilterParam {
    pub const fn init_param() -> Self {
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
