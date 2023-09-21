use core::panic;

const TAC_ENABLED_BIT: u8 = 1 << 2;

#[derive(Copy, Clone)]
pub struct TimerControl {
    pub enabled: bool,
    byte: u8,
    mask: u16,
}

impl TimerControl {
    pub fn new() -> Self {
        TimerControl::from(0)
    }

    pub fn is_frequency_bit_set(&self, div: u16) -> bool {
        (div & self.mask) != 0
    }
}

impl std::convert::From<TimerControl> for u8 {
    fn from(control: TimerControl) -> u8 {
        0b11111_000 | control.byte
    }
}

impl std::convert::From<u8> for TimerControl {
    fn from(byte: u8) -> Self {
        let enabled = byte & TAC_ENABLED_BIT != 0;
        let mask = match byte & 0b11 {
            0b00 => 1 << 9, // 1024
            0b01 => 1 << 3, // 16
            0b10 => 1 << 5, // 64
            0b11 => 1 << 7, // 256
            _ => panic!("timer control byte & 0b11 isn't matched"),
        };

        TimerControl {
            enabled,
            byte,
            mask,
        }
    }
}
