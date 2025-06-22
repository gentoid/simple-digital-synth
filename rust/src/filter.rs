use defmt::Format;

pub struct Filter {
    pub cutoff: u16,
    pub resonance: u8,
    pub gain: i8,
}

#[derive(Debug, Format)]
pub enum FilterParam {
    Cutoff,
    Resonance,
    Gain,
}


impl FilterParam {
    pub fn init_param() -> Self {
        Self::Cutoff
    }

    pub fn next_param(param: &Self) -> Option<Self> {
        use FilterParam::*;
        
        match param {
            Cutoff => Some(Resonance),
            Resonance => Some(Gain),
            Gain => None,
        }
    }
}
