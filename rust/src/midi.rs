use crate::tables::midi_to_freq::MIDI_FREQS;

pub const fn midi_note_to_freq(note: u8) -> f32 {
    MIDI_FREQS[note as usize]
}
