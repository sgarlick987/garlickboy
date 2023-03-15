use crate::cpu::CPU;

pub trait Control {
    fn nop(&mut self) -> u8;
}

impl Control for CPU {
    // NOP - 0x00
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: control/misc
    // Timingwithout branch (4t)
    // fetch
    fn nop(&mut self) -> u8 {
        //fetch
        self.pc = self.pc.wrapping_add(1);
        self.sync()
    }
}
