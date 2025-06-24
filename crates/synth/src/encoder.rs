use defmt::{Format, info};

use crate::{filter::FilterParam, oscillator::OscParams};

#[derive(PartialEq)]
pub enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Format)]
pub enum EncoderParam {
    Osc(OscParams),
    Filter(FilterParam),
}

impl EncoderParam {
    pub const fn init_param() -> Self {
        EncoderParam::Osc(OscParams::NextWave)
    }
}

pub struct Encoder {
    pub parameter: EncoderParam,
}

impl Encoder {
    pub const fn new() -> Self {
        Self {
            parameter: EncoderParam::init_param(),
        }
    }
    pub fn next_param(&mut self) {
        use EncoderParam::*;

        self.parameter =
            match &self.parameter {
                Osc(param) => OscParams::next_param(param)
                    .map_or_else(|| Filter(FilterParam::init_param()), Osc),
                Filter(param) => FilterParam::next_param(param)
                    .map_or_else(|| Osc(OscParams::init_param()), Filter),
            };

        info!("Next parameter is: {:?}", self.parameter);
    }
}
