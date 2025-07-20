use crate::{
    adsr::{self, TimeMs},
    consts::SAMPLE_RATE,
    encoder::{EncoderParam, Rotation},
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

    pub fn adjust(&mut self, _param: &EncoderParam, _rotation: Rotation) {
        // match param {
        //     // EncoderParam::Osc(param) => self.oscillator.adjust(param, rotation),
        //     // EncoderParam::Filter(param) => self.filter.adjust(param, rotation),
        // }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.voice_pool.next_sample()
    }

    pub fn is_active(&self) -> bool {
        self.voice_pool.is_active()
    }
}
