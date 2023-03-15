use super::*;
use crate::cpu::CPU;
use crate::utils::*;

pub trait Logic {
    fn xor_r8(&mut self, target: &TargetRegister8) -> u8;
    fn adc_r8(&mut self, target: &TargetRegister8) -> u8;
    fn add_r8(&mut self, target: &TargetRegister8) -> u8;
    fn add_hl(&mut self) -> u8;
    fn sub_r8(&mut self, target: &TargetRegister8) -> u8;
    fn inc(&mut self, target: &TargetIncDec) -> u8;
    fn dec(&mut self, target: &TargetIncDec) -> u8;
    fn cp_hl(&mut self) -> u8;
    fn cp_u8(&mut self) -> u8;
}

impl Logic for CPU {
    // XOR A,B - 0xA8
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	unset
    // Carry	unset
    // Group: x8/alu
    // Timingwithout branch (4t)
    // fetch
    fn xor_r8(&mut self, target: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        self.registers.a ^= match target {
            TargetRegister8::A => self.registers.a,
            TargetRegister8::B => self.registers.b,
            TargetRegister8::C => self.registers.c,
            TargetRegister8::D => self.registers.d,
            TargetRegister8::E => self.registers.e,
            TargetRegister8::H => self.registers.h,
            TargetRegister8::L => self.registers.l,
        };
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.negative = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = false;
        let cycles_used = self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // CP A,u8 - 0xFE
    // Length: 2 bytes
    // FlagsZero	dependent
    // Negative	set
    // Half Carry	dependent
    // Carry	dependent
    // Group: x8/alu
    // Timingwithout branch (8t)
    // fetch
    // read	u8
    fn cp_u8(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let byte = self.read_byte_lower();
        let a = self.registers.a;
        self.registers.flags.negative = true;
        self.registers.flags.zero = a == byte;
        self.registers.flags.carry = a < byte;
        self.registers.flags.half_carry = bytes_half_carry(a, byte);
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // CP A,(HL) - 0xBE
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	set
    // Half Carry	dependent
    // Carry	dependent
    // Group: x8/alu
    // Timingwithout branch (8t)
    // fetch
    // read	(HL)
    fn cp_hl(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let hl = self.registers.get_hl();
        let byte = self.read_byte(hl);
        let a = self.registers.a;
        self.registers.flags.negative = true;
        self.registers.flags.zero = a == byte;
        self.registers.flags.carry = a < byte;
        self.registers.flags.half_carry = bytes_half_carry(a, byte);
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // INC B - 0x04
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	dependent
    // Carry	unmodified
    // Group: x8/alu
    // Timingwithout branch (4t)
    // fetch

    // INC BC - 0x03
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x16/alu
    // Timingwithout branch (8t)
    // fetch	Probably writes to C here
    // internal	Probably writes to B here

    //TODO: inc impl half carry
    //TODO: fix r16 timing
    //TODO: r8 sync before or after set
    fn inc(&mut self, target: &TargetIncDec) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        let mut cycles_used = 0;

        match target {
            TargetIncDec::A => {
                self.registers.a = self.registers.a.wrapping_add(1);
                self.registers.flags.zero = self.registers.a == 0;
                self.registers.flags.negative = false;
                //fetch
                cycles_used += self.sync();
            }
            TargetIncDec::B => {
                self.registers.b = self.registers.b.wrapping_add(1);
                self.registers.flags.zero = self.registers.b == 0;
                self.registers.flags.negative = false;
                //fetch
                cycles_used += self.sync();
            }
            TargetIncDec::C => {
                self.registers.c = self.registers.c.wrapping_add(1);
                self.registers.flags.zero = self.registers.c == 0;
                self.registers.flags.negative = false;
                //fetch
                cycles_used += self.sync();
            }
            TargetIncDec::D => {
                self.registers.d = self.registers.d.wrapping_add(1);
                self.registers.flags.zero = self.registers.d == 0;
                self.registers.flags.negative = false;
                //fetch
                cycles_used += self.sync();
            }
            TargetIncDec::E => {
                self.registers.e = self.registers.e.wrapping_add(1);
                self.registers.flags.zero = self.registers.e == 0;
                self.registers.flags.negative = false;
                //fetch
                cycles_used += self.sync();
            }
            TargetIncDec::H => {
                self.registers.h = self.registers.h.wrapping_add(1);
                self.registers.flags.zero = self.registers.h == 0;
                self.registers.flags.negative = false;
                //fetch
                cycles_used += self.sync();
            }
            TargetIncDec::L => {
                self.registers.l = self.registers.l.wrapping_add(1);
                self.registers.flags.zero = self.registers.l == 0;
                self.registers.flags.negative = false;
                //fetch
                cycles_used += self.sync();
            }
            TargetIncDec::BC => {
                //write
                self.registers
                    .set_bc(self.registers.get_bc().wrapping_add(1));
                cycles_used += self.sync();
            }
            TargetIncDec::DE => {
                //write
                self.registers
                    .set_de(self.registers.get_de().wrapping_add(1));
                cycles_used += self.sync();
            }
            TargetIncDec::HL => {
                //write
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_add(1));
                cycles_used += self.sync();
            }
            TargetIncDec::SP => {
                //write
                self.registers.sp = self.registers.sp.wrapping_add(1);
                cycles_used += self.sync();
            }
            TargetIncDec::HLPOINTER => {
                //read
                let address = self.registers.get_hl();
                let byte = self.read_byte(address) - 1;
                cycles_used += self.sync();

                //write
                self.write_byte(address, byte);
                cycles_used += self.sync();
            }
        }

        self.pc = next_pc;
        cycles_used
    }

    // DEC B - 0x05
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	set
    // Half Carry	dependent
    // Carry	unmodified
    // Group: x8/alu
    // Timingwithout branch (4t)
    // fetch

    // DEC BC - 0x0B
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x16/alu
    // Timingwithout branch (8t)
    // fetch	Probably writes to C here
    // internal	Probably writes to B here

    //TODO: dec impl half carry
    //TODO: fix r16 timing
    //TODO: r8 sync before or after set
    fn dec(&mut self, target: &TargetIncDec) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = 0;

        match target {
            TargetIncDec::A => {
                self.registers.a = self.registers.a.wrapping_sub(1);
                self.registers.flags.zero = self.registers.a == 0;
                self.registers.flags.negative = true;
                cycles_used += self.sync();
            }
            TargetIncDec::B => {
                self.registers.b = self.registers.b.wrapping_sub(1);
                self.registers.flags.zero = self.registers.b == 0;
                self.registers.flags.negative = true;
                cycles_used += self.sync();
            }
            TargetIncDec::C => {
                self.registers.c = self.registers.c.wrapping_sub(1);
                self.registers.flags.zero = self.registers.c == 0;
                self.registers.flags.negative = true;
                cycles_used += self.sync();
            }
            TargetIncDec::D => {
                self.registers.d = self.registers.d.wrapping_sub(1);
                self.registers.flags.zero = self.registers.d == 0;
                self.registers.flags.negative = true;
                cycles_used += self.sync();
            }
            TargetIncDec::E => {
                self.registers.e = self.registers.e.wrapping_sub(1);
                self.registers.flags.zero = self.registers.e == 0;
                self.registers.flags.negative = true;
                cycles_used += self.sync();
            }
            TargetIncDec::DE => {
                self.registers
                    .set_de(self.registers.get_de().wrapping_sub(1));
                cycles_used += self.sync();
            }
            TargetIncDec::HL => {
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1));
                cycles_used += self.sync();
            }

            _ => {
                panic!("{:?} unimplemented DEC", target);
            }
        }

        self.pc = next_pc;
        cycles_used
    }

    // ADD A,(HL) - 0x86
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	dependent
    // Carry	dependent
    // Group: x8/alu
    // Timingwithout branch (8t)
    // fetch
    // read	(HL)
    fn add_hl(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let hl = self.registers.get_hl();
        let byte = self.read_byte(hl);
        self.registers.a = self._add(byte, false);
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // SUB A,B - 0x90
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	set
    // Half Carry	dependent
    // Carry	dependent
    // Group: x8/alu
    // Timingwithout branch (4t)
    // fetch
    fn sub_r8(&mut self, target: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        self.registers.a = match target {
            TargetRegister8::A => self._sub(self.registers.a, false),
            TargetRegister8::B => self._sub(self.registers.b, false),
            TargetRegister8::C => self._sub(self.registers.c, false),
            TargetRegister8::D => self._sub(self.registers.d, false),
            TargetRegister8::E => self._sub(self.registers.e, false),
            TargetRegister8::H => self._sub(self.registers.h, false),
            TargetRegister8::L => self._sub(self.registers.l, false),
        };
        let cycles_used = self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // ADD A,B - 0x80
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	dependent
    // Carry	dependent
    // Group: x8/alu
    // Timingwithout branch (4t)
    // fetch
    fn add_r8(&mut self, target: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        self.registers.a = match target {
            TargetRegister8::A => self._add(self.registers.a, false),
            TargetRegister8::B => self._add(self.registers.b, false),
            TargetRegister8::C => self._add(self.registers.c, false),
            TargetRegister8::D => self._add(self.registers.d, false),
            TargetRegister8::E => self._add(self.registers.e, false),
            TargetRegister8::H => self._add(self.registers.h, false),
            TargetRegister8::L => self._add(self.registers.l, false),
        };
        let cycles_used = self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // ADC A,B - 0x88
    // Length: 1 byte
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	dependent
    // Carry	dependent
    // Group: x8/alu
    // Timingwithout branch (4t)
    // fetch
    fn adc_r8(&mut self, target: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        self.registers.a = match target {
            TargetRegister8::A => self._add(self.registers.a, self.registers.flags.carry),
            TargetRegister8::B => self._add(self.registers.b, self.registers.flags.carry),
            TargetRegister8::C => self._add(self.registers.c, self.registers.flags.carry),
            TargetRegister8::D => self._add(self.registers.d, self.registers.flags.carry),
            TargetRegister8::E => self._add(self.registers.e, self.registers.flags.carry),
            TargetRegister8::H => self._add(self.registers.h, self.registers.flags.carry),
            TargetRegister8::L => self._add(self.registers.l, self.registers.flags.carry),
        };
        let cycles_used = self.sync();

        self.pc = next_pc;
        cycles_used
    }
}

impl CPU {
    fn _add(&mut self, value: u8, carry: bool) -> u8 {
        let (added, overflowed) = self.registers.a.carrying_add(value, carry);
        self.registers.flags.zero = added == 0;
        self.registers.flags.negative = false;
        self.registers.flags.carry = overflowed;
        self.registers.flags.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;

        added
    }

    fn _sub(&mut self, value: u8, carry: bool) -> u8 {
        let (subbed, overflowed) = self.registers.a.borrowing_sub(value, carry);
        self.registers.flags.zero = subbed == 0;
        self.registers.flags.negative = true;
        self.registers.flags.carry = overflowed;
        (_, self.registers.flags.half_carry) =
            (self.registers.a & 0x0F).overflowing_sub(value & 0x0F);

        subbed
    }
}
