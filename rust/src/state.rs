use defmt::info;

use crate::{
    consts::MIDI_NOTES_AMOUNT,
    encoder::{EncoderParam, Rotation},
    filter::{Filter, FilterParam},
};

pub struct State {
    pub midi_note: u8,
    pub filter: Filter,
}

impl State {
    pub const fn new() -> Self {
        Self {
            filter: Filter::new(),
            midi_note: 69,
        }
    }
    pub fn adjust(&mut self, param: &EncoderParam, rotation: Rotation) {
        match param {
            EncoderParam::MidiNote => self.udajust_midi_note(rotation),
            EncoderParam::Filter(param) => self.adjust_filter_param(param, rotation),
        }
    }

    fn udajust_midi_note(&mut self, rotation: Rotation) {
        let new = if rotation == Rotation::Right {
            self.midi_note.saturating_add(1)
        } else {
            self.midi_note.saturating_sub(1)
        };
        // new.clamp(0, (MIDI_NOTES_AMOUNT - 1) as u8);
        self.midi_note = new;
        info!("Set MIDI note: {}", self.midi_note);
    }

    fn adjust_filter_param(&mut self, param: &FilterParam, rotation: Rotation) {
        match param {
            FilterParam::Cutoff => {
                let delta = if rotation == Rotation::Right {
                    1.1
                } else {
                    1.0 / 1.1
                };
                self.filter.cutoff = delta * self.filter.cutoff;
                info!("Set filter cutoff: {}", self.filter.cutoff);
            }
            FilterParam::Resonance => {
                if rotation == Rotation::Right {
                    self.filter.resonance += 1.0;
                } else {
                    self.filter.resonance -= 1.0;
                };
                info!("Set filter resonance: {}", self.filter.resonance);
            }
            FilterParam::Gain => {
                if rotation == Rotation::Right {
                    self.filter.gain += 1.0;
                } else {
                    self.filter.gain -= 1.0;
                };
                info!("Set filter gain: {}", self.filter.gain);
            }
        }
    }
}
