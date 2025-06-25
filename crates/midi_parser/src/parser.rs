#[cfg(not(feature = "std"))]
use heapless::Vec;

use crate::tables::MIDI_FREQS;

#[derive(Debug, PartialEq)]
pub struct Note(pub u8);

#[derive(Debug, PartialEq)]
pub struct Velocity(pub u8);

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

    pub fn bytes_requires(&self) -> usize {
        match self {
            Self::NoteOff(_, _) => 2,
            Self::NoteOn(_, _) => 2,
            Self::PolyphonicAT(_, _) => 2,
            Self::CC(_, _) => 2,
            Self::ProgramChange(_) => 1,
            Self::ChannelAT(_) => 1,
            Self::PithBend(_) => 2,
            Self::SysEx => 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug)]
pub struct RunningStatus {
    message_kind: Option<MidiMessageKind>,
    message_reading: Option<MidiMessageKind>,
    midi_channel: MidiChannel,
    #[cfg(feature = "std")]
    data_buffer: Vec<u8>,
    #[cfg(not(feature = "std"))]
    data_buffer: Vec<u8, 3>,
    bytes_to_read: usize,
}

impl RunningStatus {
    pub fn new(midi_channel: MidiChannel) -> Self {
        Self {
            message_kind: None,
            message_reading: None,
            midi_channel,
            data_buffer: Vec::new(),
            bytes_to_read: 0,
        }
    }

    pub fn message_kind(&self) -> &Option<MidiMessageKind> {
        &self.message_kind
    }

    pub fn process_midi_byte(&mut self, byte: u8) {
        // Is it a data byte?
        if byte & 0x80 != 0x80 {
            self.process_data_byte(byte);
            return;
        }

        if MidiChannel::from_byte(&byte) != self.midi_channel {
            return;
        }

        let msg_kind = MidiMessageKind::from_byte(&byte);
        self.bytes_to_read = msg_kind.bytes_requires();
        self.message_reading = Some(msg_kind);
    }

    fn process_data_byte(&mut self, byte: u8) {
        use MidiMessageKind::*;

        if self
            .message_reading
            .as_ref()
            .or(self.message_kind.as_ref())
            .is_none()
        {
            return;
        }

        #[cfg(feature = "std")]
        self.data_buffer.push(byte);

        #[cfg(not(feature = "std"))]
        // todo do proper "unwrap"
        self.data_buffer.push(byte).unwrap();

        if self.bytes_to_read > self.data_buffer.len() {
            return;
        }

        match self
            .message_reading
            .as_mut()
            .or_else(|| self.message_kind.as_mut())
            .unwrap()
        {
            NoteOff(note, velocity) | NoteOn(note, velocity) | PolyphonicAT(note, velocity) => {
                *note = Note(self.data_buffer[0]);
                *velocity = Velocity(self.data_buffer[1]);
            }
            CC(cc_number, cc_value) => {
                *cc_number = ControlNum(self.data_buffer[0]);
                *cc_value = ControlVal(self.data_buffer[1]);
            }
            ProgramChange(program) => *program = ProgramNumber(self.data_buffer[0]),
            ChannelAT(velocity) => *velocity = Velocity(self.data_buffer[0]),
            PithBend(bend_value) => {
                let value = (self.data_buffer[0] as u16) | ((self.data_buffer[1] as u16) << 7);
                *bend_value = PitchBendValue(value);
            }
            SysEx => {
                // info!("SysEx MIDI data byte is ignored");
            }
        }

        if self.message_reading.is_some() {
            self.message_kind = self.message_reading.take();
        }

        self.data_buffer.clear();
        return;
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
    use MidiMessageKind::*;

    #[test]
    fn parser_ignores_messages_for_another_channels() {
        let ch = MidiChannel::Ch2;
        let mut rs = RunningStatus::new(ch);

        rs.process_midi_byte(0x80);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0x82);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0x93);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0xA4);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0xB5);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0xC6);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0xD7);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0xE8);
        assert_status_is_init(&rs, ch);

        rs.process_midi_byte(0xFF);
        assert_status_is_init(&rs, ch);
    }

    fn assert_status_is_init(rs: &RunningStatus, channel: MidiChannel) {
        assert_eq!(rs.midi_channel, channel);
        assert_eq!(rs.bytes_to_read, 0);
        assert_eq!(rs.message_kind, None);
    }

    #[test]
    fn note_on() {
        let ch = MidiChannel::Ch11;
        let mut rs = RunningStatus::new(ch);

        rs.process_midi_byte(0x9A);
        assert_eq!(rs.message_kind, None);
        rs.process_midi_byte(0x73);
        rs.process_midi_byte(0x48);
        assert_eq!(rs.message_kind, Some(NoteOn(Note(115), Velocity(72))));
    }

    #[test]
    fn running_status() {
        let ch = MidiChannel::Ch5;
        let mut rs = RunningStatus::new(ch);

        rs.process_midi_byte(0x94);
        assert_eq!(rs.message_kind, None);

        rs.process_midi_byte(0x73);
        rs.process_midi_byte(0x48);
        assert_eq!(rs.message_kind, Some(NoteOn(Note(115), Velocity(72))));

        rs.process_midi_byte(0x39);
        rs.process_midi_byte(0x77);
        assert_eq!(rs.message_kind, Some(NoteOn(Note(57), Velocity(119))));

        rs.process_midi_byte(0x53);
        // it keeps previous message kind until all required data received
        assert_eq!(rs.message_kind, Some(NoteOn(Note(57), Velocity(119))));
        rs.process_midi_byte(0x0F);
        // println!("{rs:?}");
        assert_eq!(rs.message_kind, Some(NoteOn(Note(83), Velocity(15))));
    }
}
