pub struct Filter {
    pub cutoff: u16,
    pub resonance: u8,
    pub gain: i8,
}

pub enum FilterParam {
    Cutoff,
    Resonance,
    Gain,
}
