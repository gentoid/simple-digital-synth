use defmt::info;
use midi_parser::parser::{MidiChannel, MidiMessageKind, RunningStatus};

use crate::{
    adsr::{self, TimeMs},
    consts::SAMPLE_RATE,
    encoder::{EncoderParam, Rotation},
    filter::Filter,
    voice::VoicePool,
};

pub struct State {
    pub filter: Filter,
    pub parser: RunningStatus,
    voice_pool: VoicePool,
}

impl State {
    pub const fn new() -> Self {
        let config = adsr::Adsr {
            attack: TimeMs(50),
            decay: TimeMs(20),
            release: TimeMs(200),
            sustain_level: 0.8,
        };

        let envelope = adsr::Envelope::new(config, SAMPLE_RATE);

        Self {
            filter: Filter::new(),
            parser: RunningStatus::new(MidiChannel::Ch1),
            voice_pool: VoicePool::new(envelope),
        }
    }

    pub fn adjust(&mut self, param: &EncoderParam, rotation: Rotation) {
        match param {
            // EncoderParam::Osc(param) => self.oscillator.adjust(param, rotation),
            EncoderParam::Filter(param) => self.filter.adjust(param, rotation),
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.voice_pool.next_sample()
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
                info!("Start note: {} with velocity: {}", note.num, velocity.0);
                self.voice_pool.on_note_on(note.clone());
            }
            NoteOff(note, velocity) | NoteOn(note, velocity) => {
                info!("Stop note: {} with velocity: {}", note.num, velocity.0);
                self.voice_pool.on_note_off(note);
            }
            _ => {}
        }
    }
}
