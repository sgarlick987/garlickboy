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
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let cycles_used = self.sync();

        self.pc = next_pc;
        cycles_used
    }
}
