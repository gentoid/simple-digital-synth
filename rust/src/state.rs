use crate::{
    encoder::{EncoderParam, Rotation},
    filter::Filter,
    oscillator::Oscillator,
};

pub struct State {
    pub filter: Filter,
    pub oscillator: Oscillator,
}

impl State {
    pub const fn new() -> Self {
        Self {
            filter: Filter::new(),
            oscillator: Oscillator::new(),
        }
    }

    pub fn adjust(&mut self, param: &EncoderParam, rotation: Rotation) {
        match param {
            EncoderParam::Osc(param) => self.oscillator.adjust(param, rotation),
            EncoderParam::Filter(param) => self.filter.adjust(param, rotation),
        }
    }
}
