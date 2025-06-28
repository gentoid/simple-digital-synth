use defmt::info;
use midi_parser::parser::{MidiChannel, MidiMessageKind, RunningStatus};

use crate::{
    adsr::{self, TimeMs},
    consts::SAMPLE_RATE,
    encoder::{EncoderParam, Rotation},
    filter::Filter,
    oscillator::Oscillator,
};

pub struct State {
    pub filter: Filter,
    pub oscillator: Oscillator,
    pub parser: RunningStatus,
    pub adsr_envelope: adsr::Envelope,
}

impl State {
    pub const fn new() -> Self {
        let config = adsr::Adsr {
            attack: TimeMs(50),
            decay: TimeMs(20),
            release: TimeMs(200),
            sustain_level: 0.8,
        };

        Self {
            filter: Filter::new(),
            oscillator: Oscillator::new(),
            parser: RunningStatus::new(MidiChannel::Ch1),
            adsr_envelope: adsr::Envelope::new(config, SAMPLE_RATE),
        }
    }

    pub fn adjust(&mut self, param: &EncoderParam, rotation: Rotation) {
        match param {
            EncoderParam::Osc(param) => self.oscillator.adjust(param, rotation),
            EncoderParam::Filter(param) => self.filter.adjust(param, rotation),
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.oscillator.next_sample() * self.adsr_envelope.next()
    }

    pub fn process_midi_byte(&mut self, byte: u8) {
        self.parser.process_midi_byte(byte);
        if self.parser.message_kind().is_none() {
            return;
        }

        if self.parser.in_progress() {
            return;
        }

        use MidiMessageKind::*;

        match self.parser.message_kind().as_ref().unwrap() {
            NoteOn(note, velocity) if velocity.0 > 0 => {
                info!("Start note: {} with velocity: {}", note.0, velocity.0);
                self.oscillator.set_note(note.0);
                self.adsr_envelope.note_on();
            }
            NoteOff(note, velocity) | NoteOn(note, velocity) => {
                info!("Stop note: {} with velocity: {}", note.0, velocity.0);
                self.adsr_envelope.note_off();
            }
            _ => {}
        }
    }
}
