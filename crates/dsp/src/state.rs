use midi_parser::parser::MidiMessage;

use crate::{
    adsr::{self, TimeMs},
    consts::SAMPLE_RATE,
    filter::Filter,
    voice::VoicePool,
};

pub struct State {
    pub filter: Filter,
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
            voice_pool: VoicePool::new(envelope),
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.voice_pool.next_sample()
    }

    pub fn is_active(&self) -> bool {
        self.voice_pool.is_active()
    }

    pub fn process_midi_msg(&mut self, msg: &MidiMessage) {
        use MidiMessage::*;
        match msg {
            NoteOn(note, _velocity) => self.voice_pool.on_note_on(note), // todo velocity
            NoteOff(note, _velocity) => self.voice_pool.on_note_off(note), // todo velocity
            // CC(num, val) => {
            //     match controller {
            //         74 => {
            //             let cutoff = map_cc_to_cutoff(value);
            //             state.filter.cutoff = cutoff;
            //         }
            //         71 => {
            //             let resonance = map_cc_to_resonance(value);
            //             state.filter.resonance = resonance;
            //         }
            //         _ => {}
            //     }
            // }
            _ => {}
        }
    }
}
