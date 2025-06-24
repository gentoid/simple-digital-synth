use core::f32::consts::TAU;

use defmt::{Format, info};
use libm::sinf;
use midi_parser::{consts::MIDI_NOTES_AMOUNT, parser::midi_note_to_freq};

use crate::{
    consts::{MAX_DAC_VALUE, SAMPLE_RATE},
    encoder::Rotation,
};

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
    MidiNote,
    Duty,
}

impl OscParams {
    pub fn init_param() -> Self {
        Self::NextWave
    }

    pub fn next_param(param: &Self) -> Option<Self> {
        use OscParams::*;

        match param {
            NextWave => Some(MidiNote),
            MidiNote => Some(Duty),
            Duty => None,
        }
    }
}

pub struct Oscillator {
    pub osc_type: WaveType,
    pub freq: f32,
    pub midi_note: u8,
    pub phase: f32,
    pub sample_rate: f32,
    pub phase_inc: f32,
    pub duty: f32,
}

impl Oscillator {
    pub const fn new() -> Self {
        let midi_note = 69;
        let mut this = Self {
            osc_type: WaveType::SawTooth,
            freq: midi_note_to_freq(midi_note),
            midi_note,
            phase: 0.0,
            sample_rate: SAMPLE_RATE,
            phase_inc: 0.0,
            duty: 0.5,
        };

        this.update_phase_inc();

        this
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
            MidiNote => {
                let new = if rotation == Rotation::Right {
                    self.midi_note.saturating_add(1)
                } else {
                    self.midi_note.saturating_sub(1)
                };
                self.set_note(new);
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

    pub fn set_note(&mut self, midi_note: u8) {
        info!("Set MIDI note: {}", midi_note);
        self.midi_note = midi_note.clamp(0, (MIDI_NOTES_AMOUNT - 1) as u8);
        self.freq = midi_note_to_freq(midi_note);
        self.update_phase_inc();
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = match self.osc_type {
            WaveType::Sine => (sinf(self.phase * TAU) + 1.0) / 2.0,
            WaveType::SawTooth => self.phase,
            WaveType::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            WaveType::PWM => {
                if self.phase < self.duty {
                    1.0
                } else {
                    0.0
                }
            }
        };

        self.phase += self.phase_inc;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        MAX_DAC_VALUE as f32 * sample
    }

    const fn update_phase_inc(&mut self) {
        self.phase_inc = self.freq / self.sample_rate;
    }
}
