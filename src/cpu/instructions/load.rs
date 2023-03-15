use super::*;
use crate::cpu::{FlagsRegister, CPU};
use crate::utils::*;

pub trait Load {
    fn ld_hl_r8(&mut self, target: &TargetRegister8) -> u8;
    fn ldi_a_hl(&mut self) -> u8;
    fn ldi_hl_a(&mut self) -> u8;
    fn ld_ff00u8_a(&mut self) -> u8;
    fn ld_a_ff00u8(&mut self) -> u8;
    fn ld_ff00c_a(&mut self) -> u8;
    fn ld_u8(&mut self, target: &TargetRegister8) -> u8;
    fn ld_a_ptr(&mut self, target: &TargetPointer) -> u8;
    fn ld_u16(&mut self, target: &TargetRegister16) -> u8;
    fn pop(&mut self, target: &TargetPushPop) -> u8;
    fn push(&mut self, target: &TargetPushPop) -> u8;
    fn ld_r8_u8(&mut self, target: &TargetRegister8) -> u8;
    fn ld_r8_r8(&mut self, target: &TargetRegister8, source: &TargetRegister8) -> u8;
    fn ld_u16_a(&mut self) -> u8;
}

impl Load for CPU {
    // LD (HL),B - 0x70
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (8t)
    // fetch
    // write	B->(HL)
    fn ld_hl_r8(&mut self, target: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //write
        match target {
            TargetRegister8::A => {
                self.write_byte(self.registers.get_hl(), self.registers.a);
            }
            _ => {
                panic!("{:?} unimplemented LDHLR8 Instruction", target);
            }
        }
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // LD A,(HL+) - 0x2A
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (8t)
    // fetch
    // read	(HL++)->A
    fn ldi_a_hl(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let hl = self.registers.get_hl();
        self.write_bytes(hl, [self.registers.a].to_vec());
        self.registers.set_hl(hl - 1);
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // LD (HL+),A - 0x22
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (8t)
    // fetch
    // write	A->(HL++)
    fn ldi_hl_a(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //write
        let hl = self.registers.get_hl();
        self.write_byte(hl, self.registers.a);
        self.registers.set_hl(hl.wrapping_add(1));
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // LD (FF00+u8),A - 0xE0
    // Length: 2 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (12t)
    // fetch
    // read	u8
    // write	A->(FF00+u8)
    fn ld_ff00u8_a(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let address = 0xFF00 + self.read_byte_lower() as u16;

        //write
        self.write_byte(address, self.registers.a);
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // LD A,(FF00+u8) - 0xF0
    // Length: 2 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (12t)
    // fetch
    // read	u8
    // read	(FF00+u8)->A
    fn ld_a_ff00u8(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let address = 0xFF00 + self.read_byte_lower() as u16;
        cycles_used += self.sync();

        //read
        self.registers.a = self.read_byte(address);
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // LD (FF00+C),A - 0xE2
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (8t)
    // fetch
    // write	A->(FF00+C)
    fn ld_ff00c_a(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //write
        let address = 0xFF00 + self.registers.c as u16;
        self.write_byte(address, self.registers.a);
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // LD B,u8 - 0x06
    // Length: 2 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (8t)
    // fetch
    // read	u8->B
    fn ld_u8(&mut self, target: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let value = self.read_byte_lower();
        cycles_used += self.sync();

        match target {
            TargetRegister8::A => self.registers.a = value,
            TargetRegister8::B => self.registers.b = value,
            TargetRegister8::C => self.registers.c = value,
            TargetRegister8::D => self.registers.d = value,
            TargetRegister8::E => self.registers.e = value,
            TargetRegister8::H => self.registers.h = value,
            TargetRegister8::L => self.registers.l = value,
        }

        self.pc = next_pc;
        cycles_used
    }

    // LD A,(BC) - 0x0A
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (8t)
    // fetch
    // read	(BC)->A
    fn ld_a_ptr(&mut self, target: &TargetPointer) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //read
        self.registers.a = match target {
            TargetPointer::BC => self.read_byte(self.registers.get_bc()),
            TargetPointer::DE => self.read_byte(self.registers.get_de()),
            TargetPointer::HL => self.read_byte(self.registers.get_hl()),
        };
        cycles_used += self.sync();

        self.pc = next_pc;
        cycles_used
    }

    // LD BC,u16 - 0x01
    // Length: 3 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x16/lsm
    // Timingwithout branch (12t)
    // fetch
    // read	u16:lower->C
    // read	u16:upper->B
    fn ld_u16(&mut self, target: &TargetRegister16) -> u8 {
        let next_pc = self.pc.wrapping_add(3);

        //fetch
        let mut cycles_used = self.sync();

        //read lower
        let lower = self.read_byte_lower();
        cycles_used += self.sync();

        //read upper
        let upper = self.read_byte_upper();
        cycles_used += self.sync();

        match target {
            TargetRegister16::SP => {
                self.registers.set_sp(upper, lower);
            }
            TargetRegister16::HL => {
                self.registers.l = lower;
                self.registers.h = upper;
            }
            TargetRegister16::DE => {
                self.registers.e = lower;
                self.registers.d = upper;
            }
            _ => {
                panic!("{:?} unimplemented LDU16", target);
            }
        }

        self.pc = next_pc;
        cycles_used
    }

    // PUSH BC - 0xC5
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x16/lsm
    // Timingwithout branch (16t)
    // fetch
    // internal
    // write	B->(--SP)
    // write	C->(--SP)
    fn push(&mut self, target: &TargetPushPop) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //internal
        cycles_used += self.sync();

        //write write
        cycles_used += match target {
            TargetPushPop::AF => self._push(self.registers.a, self.registers.get_f()),
            TargetPushPop::HL => self._push(self.registers.h, self.registers.l),
            TargetPushPop::BC => self._push(self.registers.b, self.registers.c),
            TargetPushPop::DE => self._push(self.registers.d, self.registers.e),
        };

        self.pc = next_pc;
        cycles_used
    }

    // POP BC - 0xC1
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x16/lsm
    // Timingwithout branch (12t)
    // fetch
    // read	(SP++)->C
    // read	(SP++)->B
    fn pop(&mut self, target: &TargetPushPop) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let mut cycles_used = self.sync();

        //read read
        let (upper, lower, used) = self._pop();
        cycles_used += used;
        match target {
            TargetPushPop::AF => {
                self.registers.a = upper;
                self.registers.flags = FlagsRegister::from(lower);
            }
            TargetPushPop::HL => {
                self.registers.h = upper;
                self.registers.l = lower;
            }
            TargetPushPop::BC => {
                self.registers.b = upper;
                self.registers.c = lower;
            }
            TargetPushPop::DE => {
                self.registers.d = upper;
                self.registers.e = lower;
            }
        };

        self.pc = next_pc;
        cycles_used
    }

    // LD D,u8 - 0x16
    // Length: 2 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (8t)
    // fetch
    // read	u8->D
    fn ld_r8_u8(&mut self, target: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let byte = self.read_byte_lower();
        cycles_used += self.sync();

        match target {
            TargetRegister8::A => self.registers.a = byte,
            TargetRegister8::B => self.registers.b = byte,
            TargetRegister8::C => self.registers.c = byte,
            TargetRegister8::D => self.registers.d = byte,
            TargetRegister8::E => self.registers.e = byte,
            TargetRegister8::H => self.registers.h = byte,
            TargetRegister8::L => self.registers.l = byte,
        }

        self.pc = next_pc;
        cycles_used
    }

    // LD B,B - 0x40
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (4t)
    // fetch
    fn ld_r8_r8(&mut self, target: &TargetRegister8, source: &TargetRegister8) -> u8 {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let cycles_used = self.sync();

        match target {
            TargetRegister8::A => {
                self.registers.a = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::B => {
                self.registers.b = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::C => {
                self.registers.c = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::D => {
                self.registers.d = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::E => {
                self.registers.e = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::H => {
                self.registers.h = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::L => {
                self.registers.l = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
        }

        self.pc = next_pc;
        cycles_used
    }

    // LD (u16),A - 0xEA
    // Length: 3 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: x8/lsm
    // Timingwithout branch (16t)
    // fetch
    // read	u16:lower
    // read	u16:upper
    // write	A->(u16)
    fn ld_u16_a(&mut self) -> u8 {
        let next_pc = self.pc.wrapping_add(3);

        //fetch
        let mut cycles_used = self.sync();

        //read lower
        let lower = self.read_byte_lower();
        cycles_used += self.sync();

        //read upper
        let upper = self.read_byte_lower();
        cycles_used += self.sync();

        //write
        let address = merge_bytes(upper, lower);
        self.write_byte(address, self.registers.a);

        self.pc = next_pc;
        cycles_used
    }
}

impl CPU {
    fn _push(&mut self, upper: u8, lower: u8) -> u8 {
        //write
        self.registers.sp -= 1;
        self.write_byte(self.registers.sp, upper);
        let mut cycles_used = self.sync();

        //write
        self.registers.sp -= 1;
        self.write_byte(self.registers.sp, lower);
        cycles_used += self.sync();

        cycles_used
    }

    fn _pop(&mut self) -> (u8, u8, u8) {
        //read
        let lower = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let mut cycles_used = self.sync();

        //read
        let upper = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        cycles_used += self.sync();

        (upper, lower, cycles_used)
    }
}
