use defmt::{info, Format};

use crate::filter::FilterParam;

#[derive(PartialEq)]
pub enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Format)]
pub enum EncoderParam {
    MidiNote,
    Filter(FilterParam),
}

impl EncoderParam {
    pub fn init_param() -> Self {
        EncoderParam::MidiNote
    }
}

pub struct Encoder {
    pub parameter: EncoderParam,
}

impl Encoder {
    pub fn new () -> Self {
        Self { parameter: EncoderParam::init_param() }
    }
    pub fn next_param(&mut self) {
        use EncoderParam::*;

        self.parameter = match &self.parameter {
            MidiNote => Filter(FilterParam::init_param()),
            Filter(param) => FilterParam::next_param(param).map_or(MidiNote, Filter),
        };

        info!("Next parameter is: {:?}", self.parameter);
    }
}

// pub struct Encoder {
//     clk: PA0<Input>,
//     dt: PA1<Input>,
//     sw: PA2<Input>,
//     pub last_clk: bool,
//     pub count: i32,
// }

// impl Encoder {
//     pub fn new(clk: PA0<Input>, dt: PA1<Input>, sw: PA2<Input>) -> Self {
//         let last_clk = clk.is_high().unwrap_or(false);
//         Self { clk, dt, sw, last_clk, count: 0 }
//     }

//     pub fn update(&mut self) {
//         let clk_now = self.clk.is_high().unwrap_or(false);

//         if clk_now != self.last_clk && clk_now {
//             let dt_now = self.dt.is_high().unwrap_or(false);

//             if dt_now != clk_now {
//                 self.count += 1;
//             } else {
//                 self.count -= 1;
//             }
//         }

//         self.last_clk = clk_now;
//     }

//     pub fn is_pressed(&self) -> bool {
//         self.sw.is_low().unwrap_or(false)
//     }

//     pub fn value(&self)-> i32 {
//         self.count
//     }
// }
