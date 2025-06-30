use defmt::debug;
use heapless::Vec;
use midi_parser::parser::Note;

use crate::{
    adsr::{self, Envelope},
    consts::MAX_TRACKING_VOICES,
    oscillator::Oscillator,
};

struct Voice {
    envelope: adsr::Envelope,
    oscillator: Oscillator,
}

impl Voice {
    fn new(envelope: Envelope, note: Note) -> Self {
        Self {
            envelope,
            oscillator: Oscillator::new(note),
        }
    }

    fn note_on(&mut self) {
        self.oscillator.start();
        self.envelope.note_on();
    }

    fn is_active(&self) -> bool {
        self.envelope.is_active()
    }

    fn note_off(&mut self) {
        self.envelope.note_off();
    }

    fn next_sample(&mut self) -> f32 {
        if !self.envelope.is_active() && self.oscillator.is_active() {
            self.oscillator.stop();
        }

        self.oscillator.next_sample() * self.envelope.next()
    }
}

pub struct VoicePool {
    voices: Vec<Voice, MAX_TRACKING_VOICES>,
    next_voice_index: usize,
    envelope: adsr::Envelope,
}

impl VoicePool {
    pub const fn new(envelope: adsr::Envelope) -> Self {
        Self {
            voices: Vec::new(),
            next_voice_index: 0,
            envelope,
        }
    }

    pub fn is_active(&self) -> bool {
        self.voices.iter().find(|v| v.is_active()).is_some()
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut total: f32 = 0.0;

        for v in self.voices.iter_mut() {
            total += v.next_sample();
        }

        total
    }

    pub fn on_note_on(&mut self, note: Note) {
        let mut voice = Voice::new(self.envelope.clone(), note);
        voice.note_on();

        if self.voices.len() <= self.next_voice_index {
            self.voices.push(voice);
        } else {
            self.voices[self.next_voice_index] = voice;
        }

        self.next_voice_index += 1;

        if self.next_voice_index >= MAX_TRACKING_VOICES {
            self.next_voice_index -= MAX_TRACKING_VOICES;
        }
    }

    pub fn on_note_off(&mut self, note: &Note) {
        for v in self.voices.iter_mut() {
            if v.oscillator.note == *note && v.is_active() {
                v.note_off();
            }
        }
    }
}
