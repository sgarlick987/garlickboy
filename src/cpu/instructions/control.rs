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

#[cfg(test)]
mod tests {
    use crate::address::*;
    use coverage_helper::test;

    use super::*;

    #[test]
    fn test_nop() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 4;
        let mut bus = Box::new(MockBus::new());
        bus.expect_sync().times(1).return_const(());
        let mut cpu = CPU::new(bus);

        cpu.nop();

        assert_eq!(cpu.pc, 1);
    }
}
