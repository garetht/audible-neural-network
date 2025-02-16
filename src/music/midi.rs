
#[derive(Copy, Clone, Debug)]
pub enum MidiMessage {
    NoteOn(NoteOn),
    NoteOff(NoteOff),
    ControlChange(ControlChange),
}

impl MidiMessage {
    pub fn channel(&self) -> u8 {
        match self {
            MidiMessage::NoteOn(note_on) => note_on.channel,
            MidiMessage::NoteOff(note_off) => note_off.channel,
            MidiMessage::ControlChange(control_change) => control_change.channel,
        }
    }

    pub fn delta_time(&self) -> u64 {
        match self {
            MidiMessage::NoteOn(note_on) => note_on.delta_time,
            MidiMessage::NoteOff(note_off) => note_off.delta_time,
            MidiMessage::ControlChange(control_change) => control_change.delta_time,
        }
    }

    pub fn to_message(&self) -> [u8; 3] {
        match self {
            MidiMessage::NoteOn(note_on) => note_on.to_message(),
            MidiMessage::NoteOff(note_off) => note_off.to_message(),
            MidiMessage::ControlChange(control_change) => control_change.to_message(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NoteOn {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
    pub delta_time: u64,
}

impl NoteOn {
    pub fn to_message(&self) -> [u8; 3] {
        [0x90 | self.channel, self.note, 85u8]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NoteOff {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
    pub delta_time: u64
}

impl NoteOff {
    pub fn to_message(&self) -> [u8; 3] {
        [0x80 | self.channel, self.note, 85u8]
    }
}

/*
    These mappings must be preset and exist in
    Ableton.
 */
#[derive(Copy, Clone, Debug)]
pub struct ControlChange {
    pub channel: u8,
    // the MIDI controller number
    pub controller_number: u8,
    pub controller_value: u8,
    pub delta_time: u64
}

impl ControlChange {
    pub fn to_message(&self) -> [u8; 3] {
        [0xB0 | self.channel, self.controller_number, self.controller_value]
    }
}
