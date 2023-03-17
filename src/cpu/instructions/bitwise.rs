use super::TargetRegister8;
use crate::cpu::CPU;

pub trait Bitwise {
    fn bit(&mut self, bit: &u8, target: &TargetRegister8) -> u8;
    fn rla(&mut self) -> u8;
    fn rl(&mut self, target: &TargetRegister8) -> u8;
}

impl Bitwise for CPU {
    // BIT 2,B - 0x50
    // Length: 2 bytes
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	set
    // Carry	unmodified
    // Group: x8/rsb
    // Timingwithout branch (8t)
    // fetch	(0xCB)
    // fetch
    fn bit(&mut self, bit: &u8, target: &TargetRegister8) -> u8 {
        //fetch
        let mut cycles_used = self.sync();
        let check = 1 << bit;

        //fetch
        match target {
            TargetRegister8::A => self.registers.flags.zero = self.registers.a & check == 0,
            TargetRegister8::B => self.registers.flags.zero = self.registers.b & check == 0,
            TargetRegister8::C => self.registers.flags.zero = self.registers.c & check == 0,
            TargetRegister8::D => self.registers.flags.zero = self.registers.d & check == 0,
            TargetRegister8::E => self.registers.flags.zero = self.registers.e & check == 0,
            TargetRegister8::H => self.registers.flags.zero = self.registers.h & check == 0,
            TargetRegister8::L => self.registers.flags.zero = self.registers.l & check == 0,
        }
        self.registers.flags.negative = false;
        self.registers.flags.half_carry = true;

        self.pc = self.pc.wrapping_add(2);
        cycles_used += self.sync();
        cycles_used
    }

    // RL C - 0x11
    // Length: 2 bytes
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	unset
    // Carry	dependent
    // Group: x8/rsb
    // Timingwithout branch (8t)
    // fetch	(0xCB)
    // fetch
    fn rl(&mut self, target: &TargetRegister8) -> u8 {
        //fetch
        let mut cycles_used = self.sync();

        //fetch
        match target {
            TargetRegister8::C => {
                let mut new_c = self.registers.c << 1;
                if self.registers.flags.carry {
                    new_c |= 1;
                }
                self.registers.flags.carry = self.registers.c >> 7 == 1;
                self.registers.flags.zero = new_c == 0;
                self.registers.c = new_c;
            }
            _ => {
                panic!("{:?} unimplemented RL Instruction", target);
            }
        }
        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;

        self.pc = self.pc.wrapping_add(2);
        cycles_used += self.sync();
        cycles_used
    }

    // RLA - 0x17
    // Length: 1 byte
    // FlagsZero	unset
    // Negative	unset
    // Half Carry	unset
    // Carry	dependent
    // Group: x8/rsb
    // Timingwithout branch (4t)
    // fetch
    fn rla(&mut self) -> u8 {
        //fetch
        let mut new_a = self.registers.a << 1;
        if self.registers.flags.carry {
            new_a |= 1;
        }
        self.registers.flags.carry = self.registers.a >> 7 == 1;
        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;
        self.registers.flags.zero = false;
        self.registers.a = new_a;

        self.pc = self.pc.wrapping_add(1);
        self.sync()
    }
}

#[cfg(test)]
mod tests {
    use crate::address::*;

    use super::*;

    fn setup_cpu(cycles: u8) -> CPU {
        let syncs = cycles / 4;
        let mut bus = Box::new(MockBus::new());
        bus.expect_sync().times(syncs as usize).return_const(());

        CPU::new(bus)
    }

    #[test]
    fn test_bit_not_set() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 8;

        let targets = [
            TargetRegister8::A,
            TargetRegister8::B,
            TargetRegister8::C,
            TargetRegister8::D,
            TargetRegister8::E,
            TargetRegister8::H,
            TargetRegister8::L,
        ];

        for target in targets {
            for bit in 0..8 {
                let mut cpu = setup_cpu(CYCLES);
                let check = 0xFF ^ (1 << bit);

                match target {
                    TargetRegister8::A => cpu.registers.a = check,
                    TargetRegister8::B => cpu.registers.b = check,
                    TargetRegister8::C => cpu.registers.c = check,
                    TargetRegister8::D => cpu.registers.d = check,
                    TargetRegister8::E => cpu.registers.e = check,
                    TargetRegister8::H => cpu.registers.h = check,
                    TargetRegister8::L => cpu.registers.l = check,
                };

                cpu.bit(&bit, &target);

                assert_eq!(cpu.pc, LENGTH);
                assert!(cpu.registers.flags.zero, "zero flag should be set");
                assert!(
                    cpu.registers.flags.half_carry,
                    "half carry flag should be set"
                );
                assert!(
                    !cpu.registers.flags.negative,
                    "negative flag should not be set"
                );
            }
        }
    }

    #[test]
    fn test_bit_set() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 8;

        let targets = [
            TargetRegister8::A,
            TargetRegister8::B,
            TargetRegister8::C,
            TargetRegister8::D,
            TargetRegister8::E,
            TargetRegister8::H,
            TargetRegister8::L,
        ];

        for target in targets {
            for bit in 0..8 {
                let mut cpu = setup_cpu(CYCLES);
                let check = 1 << bit;

                match target {
                    TargetRegister8::A => cpu.registers.a = check,
                    TargetRegister8::B => cpu.registers.b = check,
                    TargetRegister8::C => cpu.registers.c = check,
                    TargetRegister8::D => cpu.registers.d = check,
                    TargetRegister8::E => cpu.registers.e = check,
                    TargetRegister8::H => cpu.registers.h = check,
                    TargetRegister8::L => cpu.registers.l = check,
                };

                cpu.bit(&bit, &target);

                assert_eq!(cpu.pc, LENGTH);
                assert!(!cpu.registers.flags.zero, "zero flag should not be set");
                assert!(
                    cpu.registers.flags.half_carry,
                    "half carry flag should be set"
                );
                assert!(
                    !cpu.registers.flags.negative,
                    "negative flag should not be set"
                );
            }
        }
    }

    #[test]
    fn test_rla_carryin_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 4;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.a = 0b10000011; //carryout
        cpu.registers.flags.carry = true; //carryin

        cpu.rla();

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.a, 0b00000111);
        assert!(cpu.registers.flags.carry, "carry flag should be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }

    #[test]
    fn test_rla_no_carryin_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 4;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.a = 0b10000011; //carryout
        cpu.registers.flags.carry = false; //no carryin

        cpu.rla();

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.a, 0b00000110);
        assert!(cpu.registers.flags.carry, "carry flag should be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }
    #[test]
    fn test_rla_carryin_no_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 4;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.a = 0b00000011; //no carryout
        cpu.registers.flags.carry = true; //carryin

        cpu.rla();

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.a, 0b00000111);
        assert!(!cpu.registers.flags.carry, "carry flag should not be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }

    #[test]
    fn test_rla_no_carryin_no_carryout() {
        const LENGTH: u16 = 1;
        const CYCLES: u8 = 4;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.a = 0b00000011; //no carryout
        cpu.registers.flags.carry = false; //no carryin

        cpu.rla();

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.a, 0b00000110);
        assert!(!cpu.registers.flags.carry, "carry flag should not be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }

    #[test]
    fn test_rl_carryin_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 8;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.c = 0b10000011; //carryout
        cpu.registers.flags.carry = true; //carryin

        cpu.rl(&TargetRegister8::C);

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000111);
        assert!(cpu.registers.flags.carry, "carry flag should be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }

    #[test]
    fn test_rl_carryin_no_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 8;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.c = 0b00000011; // no carryout
        cpu.registers.flags.carry = true; //carryin

        cpu.rl(&TargetRegister8::C);

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000111);
        assert!(!cpu.registers.flags.carry, "carry flag should not be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }

    #[test]
    fn test_rl_no_carryin_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 8;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.c = 0b10000011; //carryout
        cpu.registers.flags.carry = false; //no carryin

        cpu.rl(&TargetRegister8::C);

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000110);
        assert!(cpu.registers.flags.carry, "carry flag should be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }

    #[test]
    fn test_rl_no_carryin_no_carryout() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 8;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.c = 0b00000011; //no carryout
        cpu.registers.flags.carry = false; //no carryin

        cpu.rl(&TargetRegister8::C);

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000110);
        assert!(!cpu.registers.flags.carry, "carry flag should not be set");
        assert!(!cpu.registers.flags.zero, "zero flag should not be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }

    #[test]
    fn test_rl_zero() {
        const LENGTH: u16 = 2;
        const CYCLES: u8 = 8;

        let mut cpu = setup_cpu(CYCLES);
        cpu.registers.c = 0b00000000; //no carryout
        cpu.registers.flags.carry = false; //no carryin

        cpu.rl(&TargetRegister8::C);

        assert_eq!(cpu.pc, LENGTH);
        assert_eq!(cpu.registers.c, 0b00000000);
        assert!(!cpu.registers.flags.carry, "carry flag should not be set");
        assert!(cpu.registers.flags.zero, "zero flag should be set");
        assert!(
            !cpu.registers.flags.half_carry,
            "half carry flag should not be set"
        );
        assert!(
            !cpu.registers.flags.negative,
            "negative flag should not be set"
        );
    }
}
