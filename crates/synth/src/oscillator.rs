use core::f32::consts::TAU;

use defmt::{Format, info};
use libm::sinf;
use midi_parser::parser::Note;

use crate::{consts::SAMPLE_RATE, encoder::Rotation};

#[derive(Format)]
pub enum WaveType {
    Sine,
    SawTooth,
    Square,
    PWM,
}

#[derive(Debug, Format)]
pub enum OscParams {
    NextWave,
    Duty,
}

impl OscParams {
    pub fn init_param() -> Self {
        Self::NextWave
    }

    pub fn next_param(param: &Self) -> Option<Self> {
        use OscParams::*;

        match param {
            NextWave => Some(Duty),
            Duty => None,
        }
    }
}

pub struct Oscillator {
    pub osc_type: WaveType,
    pub note: Note,
    pub phase: f32,
    pub sample_rate: f32,
    pub phase_inc: f32,
    pub duty: f32,
    active: bool,
}

impl Oscillator {
    pub const fn new(note: Note) -> Self {
        let mut this = Self {
            osc_type: WaveType::SawTooth,
            note,
            phase: 0.0,
            sample_rate: SAMPLE_RATE,
            phase_inc: 0.0,
            duty: 0.5,
            active: false,
        };

        this.update_phase_inc();

        this
    }

    pub fn start(&mut self) {
        self.active = true;
        self.phase = 0.0;
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        return self.active;
    }

    pub fn adjust(&mut self, param: &OscParams, rotation: Rotation) {
        use OscParams::*;

        match param {
            NextWave => {
                use WaveType::*;

                self.osc_type = match self.osc_type {
                    Sine => SawTooth,
                    SawTooth => Square,
                    Square => PWM,
                    PWM => Sine,
                };
                info!("Set osc type: {}", self.osc_type);
            }
            Duty => {
                let new = if rotation == Rotation::Right {
                    self.duty + 0.05
                } else {
                    self.duty - 0.05
                };

                self.duty = new.clamp(0.05, 0.95);
                info!("Set duty: {}", self.duty);
            }
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }

        let sample = match self.osc_type {
            WaveType::Sine => sinf(self.phase * TAU),
            WaveType::SawTooth => self.phase * 2.0 - 1.0,
            WaveType::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            WaveType::PWM => {
                if self.phase < self.duty {
                    1.0
                } else {
                    -1.0
                }
            }
        };

        self.phase += self.phase_inc;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }

    const fn update_phase_inc(&mut self) {
        self.phase_inc = self.note.freq / self.sample_rate;
    }
}
