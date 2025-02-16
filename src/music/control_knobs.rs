pub mod control_knobs {
    use crate::music::midi::MidiMessage;
    use crate::music::midi::ControlChange;

    const CONTROLLER_NUMBER: u8 = 0x50;

    // bias
    pub fn create_set_guitar_distortion_message(controller_value: u8) -> MidiMessage {
        MidiMessage::ControlChange(
            ControlChange {
                channel: 0,
                controller_number: CONTROLLER_NUMBER,
                controller_value,
                delta_time: 0,
            }
        )
    }

    // weights
    pub fn create_set_guitar_mosaic_message(controller_value: u8) -> MidiMessage {
        if controller_value < 25 {
            create_activate_guitar_mosaic_message(false)
        } else {
            MidiMessage::ControlChange(
                ControlChange {
                    channel: 1,
                    controller_number: CONTROLLER_NUMBER,
                    controller_value,
                    delta_time: 0,
                }
            )
        }

    }
    pub fn create_set_organ_distortion_message(controller_value: u8) -> MidiMessage {
        MidiMessage::ControlChange(
            ControlChange {
                channel: 2,
                controller_number: CONTROLLER_NUMBER,
                controller_value,
                delta_time: 0,
            }
        )
    }

    pub fn create_set_organ_mosaic_message(controller_value: u8) -> MidiMessage {
        if controller_value < 25 {
            create_activate_organ_mosaic_message(false)
        } else {
            MidiMessage::ControlChange(
                ControlChange {
                    channel: 3,
                    controller_number: CONTROLLER_NUMBER,
                    controller_value,
                    delta_time: 0,
                }
            )
        }
    }

    pub fn create_activate_guitar_mosaic_message(is_on: bool) -> MidiMessage {
        MidiMessage::ControlChange(
            ControlChange {
                channel: 0,
                controller_number: CONTROLLER_NUMBER + 1,
                controller_value: if is_on { 127u8 } else { 0u8 },
                delta_time: 0,
            }
        )
    }

    pub fn create_activate_organ_mosaic_message(is_on: bool) -> MidiMessage {
        MidiMessage::ControlChange(
            ControlChange {
                channel: 1,
                controller_number: CONTROLLER_NUMBER + 1,
                controller_value: if is_on { 127u8 } else { 0u8 },
                delta_time: 0,
            }
        )
    }
}
