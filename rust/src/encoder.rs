use crate::filter::FilterParam;


pub enum  EncoderParam {
    MidiNote,
    Filter(FilterParam),
}

pub struct Encoder {
    parameter: EncoderParam,
}

#[derive(PartialEq)]
pub enum Rotation {
    Left,
    Right,
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
