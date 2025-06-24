use crate::tables::MIDI_FREQS;

#[derive(Debug, PartialEq)]
pub struct Note(u8);

#[derive(Debug, PartialEq)]
pub struct Velocity(u8);

#[derive(Debug, PartialEq)]
pub struct ControlNum(u8);

#[derive(Debug, PartialEq)]
pub struct ControlVal(u8);

#[derive(Debug, PartialEq)]
pub struct ProgramNumber(u8);

#[derive(Debug, PartialEq)]
pub struct PitchBendValue(u16);

#[derive(Debug, PartialEq)]
pub enum MidiMessageKind {
    NoteOff(Note, Velocity),
    NoteOn(Note, Velocity),
    PolyphonicAT(Note, Velocity),
    CC(ControlNum, ControlVal),
    ProgramChange(ProgramNumber),
    ChannelAT(Velocity),
    PithBend(PitchBendValue),
    SysEx,
}

impl MidiMessageKind {
    pub fn from_byte(byte: &u8) -> Self {
        use MidiMessageKind::*;

        match byte & 0xF0 {
            0x80 => NoteOff(Note(0), Velocity(0)),
            0x90 => NoteOn(Note(0), Velocity(0)),
            0xA0 => PolyphonicAT(Note(0), Velocity(0)),
            0xB0 => CC(ControlNum(0), ControlVal(0)),
            0xC0 => ProgramChange(ProgramNumber(0)),
            0xD0 => ChannelAT(Velocity(0)),
            0xE0 => PithBend(PitchBendValue(0)),
            0xF0 => SysEx,
            _ => unreachable!("All kind of MIDI messages processed"),
        }
    }

    pub fn process_data_byte(&mut self, byte: u8, bytes_counter: &u32) {
        use MidiMessageKind::*;

        match self {
            NoteOff(note, velocity) | NoteOn(note, velocity) | PolyphonicAT(note, velocity) => {
                match bytes_counter % 2 {
                    0 => *note = Note(byte),
                    1 => *velocity = Velocity(byte),
                    _ => unreachable!("'value % 2' cannot have more than 2 values"),
                }
            }
            CC(cc_number, cc_value) => match bytes_counter % 2 {
                0 => *cc_number = ControlNum(byte),
                1 => *cc_value = ControlVal(byte),
                _ => unreachable!("'value % 2' cannot have more than 2 values"),
            },
            ProgramChange(program) => *program = ProgramNumber(byte),
            ChannelAT(velocity) => *velocity = Velocity(byte),
            PithBend(bend_value) => match bytes_counter % 2 {
                0 => *bend_value = PitchBendValue((bend_value.0 & 0x7F) | ((byte as u16) << 7)),
                1 => *bend_value = PitchBendValue((bend_value.0 & 0xFF80) | byte as u16),
                _ => unreachable!("'value % 2' cannot have more than 2 values"),
            },
            SysEx => {
                // info!("SysEx MIDI data byte is ignored");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MidiChannel {
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
    Ch8,
    Ch9,
    Ch10,
    Ch11,
    Ch12,
    Ch13,
    Ch14,
    Ch15,
    Ch16,
}

impl MidiChannel {
    pub fn from_byte(byte: &u8) -> Self {
        use MidiChannel::*;

        match byte & 0x0f {
            0x0 => Ch1,
            0x1 => Ch2,
            0x2 => Ch3,
            0x3 => Ch4,
            0x4 => Ch5,
            0x5 => Ch6,
            0x6 => Ch7,
            0x7 => Ch8,
            0x8 => Ch9,
            0x9 => Ch10,
            0xA => Ch11,
            0xB => Ch12,
            0xC => Ch13,
            0xD => Ch14,
            0xE => Ch15,
            0xF => Ch16,
            _ => unreachable!("u4 only has 16 values"),
        }
    }
}

pub struct RunningStatus {
    message_kind: Option<MidiMessageKind>,
    midi_channel: MidiChannel,
    bytes_counter: u32,
}

impl RunningStatus {
    pub fn new(midi_channel: MidiChannel) -> Self {
        Self {
            message_kind: None,
            midi_channel,
            bytes_counter: 0,
        }
    }

    pub fn process_midi_byte(&mut self, byte: u8) {
        if byte & 0x80 == 0x80 {
            self.midi_channel = MidiChannel::from_byte(&byte);
            self.message_kind = Some(MidiMessageKind::from_byte(&byte));
            self.bytes_counter = 0;
            return;
        }

        if let Some(kind) = self.message_kind.as_mut() {
            kind.process_data_byte(byte, &self.bytes_counter);
            self.bytes_counter += 1;
        }
    }
}

pub const fn midi_note_to_freq(note: u8) -> f32 {
    MIDI_FREQS[note as usize]
}

// MIDI:
// 1xxxxxxx -> Status
// 0xxxxxxx -> Data

// 1xxxnnnn -> nnnn is channel number => 1...16, xxx is kind of the MIDI message

// Kind of MIDI message
// 000 - note off
// 001 - note on
// 010 - polyphonic aftertouch
// 011 - CC (control change)
// 100 - program change
// 101 - channel aftertouch
// 110 - pitch bend change
// 111 - SysEx (system exclusive) message

// 1001nnnn - note on on channel nnnn
// 2 data bytes:
// 0kkkkkkk - the key
// 0vvvvvvv - the velocity

// 1000nnnn - note off. same as note on, + same 2 bytes

// 1011nnnn - CC
// 2 data bytes:
// 0ccccccc - the controller number
// 0vvvvvvv - the value

// 1100nnnn - program change
// 1 data byte:
// 0ppppppp - the program number

// 1010nnnn - polyphonic aftertouch
// 2 data bytes:
// 0kkkkkkk - the key
// 0vvvvvvv - the velocity

// 1101nnnn - channel aftertouch (from the key with the highest pressure)
// 1 data byte:
// 0vvvvvvv - the velocity

// 1110nnnn - pitch bend
// 2 data bytes:
// 0lllllll - the least significant 7 bits
// 0mmmmmmm - the most significant 7 bits

// Running Status
// do not send status part if the message kind and the channel are the same, i.e., the whole status byte is same
// a trick with it: send note on with velocity = 0 instead of note off

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_midi_messages() {
        let mut rs = RunningStatus::new(MidiChannel::Ch2);
        rs.process_midi_byte(0x80);
        assert_eq!(rs.midi_channel, MidiChannel::Ch2);
        assert_eq!(rs.bytes_counter, 1);
        assert_eq!(
            rs.message_kind,
            Some(MidiMessageKind::NoteOn(Note(69), Velocity(127)))
        );
    }
}
