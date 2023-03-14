#![allow(dead_code)]
pub mod instructions;
pub mod registers;

use crate::address::*;

use self::{instructions::*, registers::*};

pub struct CPU {
    registers: Registers,
    pub address_bus: AddressBus,
    pc: u16,
}

struct Step {
    cycles: u8,
    pc: u16,
}

impl CPU {
    pub fn new(address_bus: AddressBus) -> CPU {
        let registers = registers::new_registers();
        CPU {
            registers,
            address_bus,
            pc: 0,
        }
    }

    pub fn render(&mut self) {
        self.address_bus.gpu.render();
    }

    pub fn write_bytes(&mut self, address: u16, bytes: Vec<u8>) {
        self.address_bus.write_bytes(address, bytes);
    }

    fn execute(&mut self, instruction: Instruction) -> Step {
        match instruction {
            Instruction::NOP => Step {
                cycles: 4,
                pc: self.pc.wrapping_add(1),
            },
            Instruction::ADCR8(target) => {
                match target {
                    TargetRegister8::A => {
                        let stored = self.registers.a;
                        let added = self.add(stored, self.registers.flags.carry);
                        self.registers.a = added;
                    }
                    TargetRegister8::B => {
                        let stored = self.registers.b;
                        let added = self.add(stored, self.registers.flags.carry);
                        self.registers.a = added;
                    }
                    TargetRegister8::C => {
                        let stored = self.registers.c;
                        let added = self.add(stored, self.registers.flags.carry);
                        self.registers.a = added;
                    }
                    TargetRegister8::D => {
                        let stored = self.registers.d;
                        let added = self.add(stored, self.registers.flags.carry);
                        self.registers.a = added;
                    }
                    TargetRegister8::E => {
                        let stored = self.registers.e;
                        let added = self.add(stored, self.registers.flags.carry);
                        self.registers.a = added;
                    }
                    TargetRegister8::H => {
                        let stored = self.registers.h;
                        let added = self.add(stored, self.registers.flags.carry);
                        self.registers.a = added;
                    }
                    TargetRegister8::L => {
                        let stored = self.registers.l;
                        let added = self.add(stored, self.registers.flags.carry);
                        self.registers.a = added;
                    }
                }
                Step {
                    cycles: 4,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::ADDHL => {
                let hl = self.registers.get_hl();
                let stored = self.address_bus.read_byte(hl);
                let added = self.add(stored, false);
                self.registers.a = added;
                Step {
                    cycles: 4,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::ADDR8(target) => {
                match target {
                    TargetRegister8::A => {
                        let stored = self.registers.a;
                        let added = self.add(stored, false);
                        self.registers.a = added;
                    }
                    TargetRegister8::B => {
                        let stored = self.registers.b;
                        let added = self.add(stored, false);
                        self.registers.a = added;
                    }
                    TargetRegister8::C => {
                        let stored = self.registers.c;
                        let added = self.add(stored, false);
                        self.registers.a = added;
                    }
                    TargetRegister8::D => {
                        let stored = self.registers.d;
                        let added = self.add(stored, false);
                        self.registers.a = added;
                    }
                    TargetRegister8::E => {
                        let stored = self.registers.e;
                        let added = self.add(stored, false);
                        self.registers.a = added;
                    }
                    TargetRegister8::H => {
                        let stored = self.registers.h;
                        let added = self.add(stored, false);
                        self.registers.a = added;
                    }
                    TargetRegister8::L => {
                        let stored = self.registers.l;
                        let added = self.add(stored, false);
                        self.registers.a = added;
                    }
                }
                Step {
                    cycles: 4,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::SUBR8(target) => {
                match target {
                    TargetRegister8::A => {
                        let stored = self.registers.a;
                        let subbed = self.sub(stored, false);
                        self.registers.a = subbed;
                    }
                    TargetRegister8::B => {
                        let stored = self.registers.b;
                        let subbed = self.sub(stored, false);
                        self.registers.a = subbed;
                    }
                    TargetRegister8::C => {
                        let stored = self.registers.c;
                        let subbed = self.sub(stored, false);
                        self.registers.a = subbed;
                    }
                    TargetRegister8::D => {
                        let stored = self.registers.d;
                        let subbed = self.sub(stored, false);
                        self.registers.a = subbed;
                    }
                    TargetRegister8::E => {
                        let stored = self.registers.e;
                        let subbed = self.sub(stored, false);
                        self.registers.a = subbed;
                    }
                    TargetRegister8::H => {
                        let stored = self.registers.h;
                        let subbed = self.sub(stored, false);
                        self.registers.a = subbed;
                    }
                    TargetRegister8::L => {
                        let stored = self.registers.l;
                        let subbed = self.sub(stored, false);
                        self.registers.a = subbed;
                    }
                }
                Step {
                    cycles: 4,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::INC(target) => {
                let mut cycles = 4;
                match target {
                    TargetIncDec::A => {
                        self.registers.a = self.registers.a.wrapping_add(1);
                        self.registers.flags.zero = self.registers.a == 0;
                        self.registers.flags.negative = false;
                    }
                    TargetIncDec::B => {
                        self.registers.b = self.registers.b.wrapping_add(1);
                        self.registers.flags.zero = self.registers.b == 0;
                        self.registers.flags.negative = false;
                    }
                    TargetIncDec::C => {
                        self.registers.c = self.registers.c.wrapping_add(1);
                        self.registers.flags.zero = self.registers.c == 0;
                        self.registers.flags.negative = false;
                    }
                    TargetIncDec::D => {
                        self.registers.d = self.registers.d.wrapping_add(1);
                        self.registers.flags.zero = self.registers.d == 0;
                        self.registers.flags.negative = false;
                    }
                    TargetIncDec::E => {
                        self.registers.e = self.registers.e.wrapping_add(1);
                        self.registers.flags.zero = self.registers.e == 0;
                        self.registers.flags.negative = false;
                    }
                    TargetIncDec::H => {
                        self.registers.h = self.registers.h.wrapping_add(1);
                        self.registers.flags.zero = self.registers.h == 0;
                        self.registers.flags.negative = false;
                    }
                    TargetIncDec::L => {
                        self.registers.l = self.registers.l.wrapping_add(1);
                        self.registers.flags.zero = self.registers.l == 0;
                        self.registers.flags.negative = false;
                    }
                    TargetIncDec::BC => {
                        self.registers
                            .set_bc(self.registers.get_bc().wrapping_add(1));
                        cycles = 8;
                    }
                    TargetIncDec::DE => {
                        self.registers
                            .set_de(self.registers.get_de().wrapping_add(1));
                        cycles = 8;
                    }
                    TargetIncDec::HL => {
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_add(1));
                        cycles = 8;
                    }
                    TargetIncDec::SP => {
                        self.registers.sp = self.registers.sp.wrapping_add(1);
                        cycles = 8;
                    }
                    TargetIncDec::HLPOINTER => {
                        let address = self.registers.get_hl();
                        let byte = self.address_bus.read_byte(address) - 1;
                        self.address_bus.write_byte(address, byte);
                        cycles = 12;
                    }
                }
                Step {
                    cycles,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::DEC(target) => {
                let mut cycles = 4;
                match target {
                    TargetIncDec::A => {
                        self.registers.a = self.registers.a.wrapping_sub(1);
                        self.registers.flags.zero = self.registers.a == 0;
                        self.registers.flags.negative = true;
                    }
                    TargetIncDec::B => {
                        self.registers.b = self.registers.b.wrapping_sub(1);
                        self.registers.flags.zero = self.registers.b == 0;
                        self.registers.flags.negative = true;
                    }
                    TargetIncDec::C => {
                        self.registers.c = self.registers.c.wrapping_sub(1);
                        self.registers.flags.zero = self.registers.c == 0;
                        self.registers.flags.negative = true;
                    }
                    TargetIncDec::D => {
                        self.registers.d = self.registers.d.wrapping_sub(1);
                        self.registers.flags.zero = self.registers.d == 0;
                        self.registers.flags.negative = true;
                    }
                    TargetIncDec::E => {
                        self.registers.e = self.registers.e.wrapping_sub(1);
                        self.registers.flags.zero = self.registers.e == 0;
                        self.registers.flags.negative = true;
                    }
                    TargetIncDec::DE => {
                        self.registers
                            .set_de(self.registers.get_de().wrapping_sub(1));
                    }
                    TargetIncDec::HL => {
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_sub(1));
                        cycles = 12;
                    }

                    _ => {
                        panic!("{:?} unimplemented DEC", target);
                    }
                }

                Step {
                    cycles,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::LDU16(target) => {
                match target {
                    TargetRegister16::SP => {
                        let lower = self.address_bus.read_byte(self.pc + 1);
                        let upper = self.address_bus.read_byte(self.pc + 2);

                        self.registers.set_sp(upper, lower);
                    }
                    TargetRegister16::HL => {
                        self.registers.l = self.address_bus.read_byte(self.pc + 1);
                        self.registers.h = self.address_bus.read_byte(self.pc + 2);
                    }
                    TargetRegister16::DE => {
                        self.registers.e = self.address_bus.read_byte(self.pc + 1);
                        self.registers.d = self.address_bus.read_byte(self.pc + 2);
                    }
                    _ => {
                        panic!("{:?} unimplemented LDU16", target);
                    }
                }

                Step {
                    cycles: 12,
                    pc: self.pc.wrapping_add(3),
                }
            }
            Instruction::LDAPTR(target) => {
                self.registers.a = match target {
                    TargetPointer::BC => self.address_bus.read_byte(self.registers.get_bc()),
                    TargetPointer::DE => self.address_bus.read_byte(self.registers.get_de()),
                    TargetPointer::HL => self.address_bus.read_byte(self.registers.get_hl()),
                };

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::XORR8(target) => {
                match target {
                    TargetRegister8::A => {
                        self.registers.a ^= self.registers.a;
                    }
                    _ => {
                        panic!("{:?} unimplemented XORR8", target);
                    }
                }
                Step {
                    cycles: 4,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::LDU8(target) => {
                let value = self.address_bus.read_byte(self.pc + 1);

                match target {
                    TargetRegister8::A => {
                        self.registers.a = value;
                    }
                    TargetRegister8::B => {
                        self.registers.b = value;
                    }
                    TargetRegister8::C => {
                        self.registers.c = value;
                    }
                    TargetRegister8::D => {
                        self.registers.d = value;
                    }
                    TargetRegister8::E => {
                        self.registers.e = value;
                    }
                    TargetRegister8::H => {
                        self.registers.h = value;
                    }
                    TargetRegister8::L => {
                        self.registers.l = value;
                    }
                }

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(2),
                }
            }
            Instruction::LDFF00CA => {
                let address = 0xFF00 + self.registers.c as u16;
                self.address_bus
                    .write_bytes(address, [self.registers.a].to_vec());

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::LDAFF00U8 => {
                let address = 0xFF00 + self.address_bus.read_byte(self.pc + 1) as u16;
                self.registers.a = self.address_bus.read_byte(address);

                Step {
                    cycles: 12,
                    pc: self.pc.wrapping_add(2),
                }
            }
            Instruction::LDDHLA => {
                let hl = self.registers.get_hl();
                self.address_bus
                    .write_bytes(hl, [self.registers.a].to_vec());
                self.registers.set_hl(hl - 1);

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::LDIHLA => {
                let hl = self.registers.get_hl();
                self.address_bus
                    .write_bytes(hl, [self.registers.a].to_vec());
                self.registers.set_hl(hl + 1);

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::LDHLR8(target) => match target {
                TargetRegister8::A => {
                    self.address_bus
                        .write_bytes(self.registers.get_hl(), [self.registers.a].to_vec());

                    Step {
                        cycles: 8,
                        pc: self.pc.wrapping_add(1),
                    }
                }
                _ => {
                    panic!("{:?} unimplemented LDHLR8 Instruction", target);
                }
            },
            Instruction::LDFF00U8A => {
                let address = 0xFF00 + self.address_bus.read_byte(self.pc + 1) as u16;
                self.address_bus
                    .write_bytes(address, [self.registers.a].to_vec());

                Step {
                    cycles: 12,
                    pc: self.pc.wrapping_add(2),
                }
            }
            Instruction::PUSH(target) => {
                self.registers.sp -= 2;

                match target {
                    TargetPushPop::AF => {
                        self.address_bus.write_bytes(
                            self.registers.sp,
                            [self.registers.get_f(), self.registers.a].to_vec(),
                        );
                    }
                    TargetPushPop::HL => {
                        self.address_bus.write_bytes(
                            self.registers.sp,
                            [self.registers.l, self.registers.h].to_vec(),
                        );
                    }
                    TargetPushPop::BC => {
                        self.address_bus.write_bytes(
                            self.registers.sp,
                            [self.registers.c, self.registers.b].to_vec(),
                        );
                    }
                    TargetPushPop::DE => {
                        self.address_bus.write_bytes(
                            self.registers.sp,
                            [self.registers.e, self.registers.d].to_vec(),
                        );
                    }
                }

                Step {
                    cycles: 16,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::POP(target) => {
                match target {
                    TargetPushPop::AF => {
                        self.registers.a = self.address_bus.read_byte(self.registers.sp + 1);
                        self.registers.flags =
                            FlagsRegister::from(self.address_bus.read_byte(self.registers.sp));
                    }
                    TargetPushPop::HL => {
                        self.registers.h = self.address_bus.read_byte(self.registers.sp + 1);
                        self.registers.l = self.address_bus.read_byte(self.registers.sp);
                    }
                    TargetPushPop::BC => {
                        self.registers.b = self.address_bus.read_byte(self.registers.sp + 1);
                        self.registers.c = self.address_bus.read_byte(self.registers.sp);
                    }
                    TargetPushPop::DE => {
                        self.registers.d = self.address_bus.read_byte(self.registers.sp + 1);
                        self.registers.e = self.address_bus.read_byte(self.registers.sp);
                    }
                }
                self.registers.sp += 2;

                Step {
                    cycles: 16,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::BIT(bit, target) => {
                match target {
                    TargetRegister8::H => {
                        self.registers.flags.negative = false;
                        self.registers.flags.half_carry = true;

                        let check = 1 << bit;
                        self.registers.flags.zero = self.registers.h & check == 0;
                    }
                    _ => {
                        panic!("{:?} unimplemented BIT Instruction", target);
                    }
                }
                Step {
                    cycles: 12,
                    pc: self.pc.wrapping_add(2),
                }
            }
            Instruction::JP => {
                let address = merge_bytes(
                    self.address_bus.read_byte(self.pc + 2),
                    self.address_bus.read_byte(self.pc + 1),
                );

                Step {
                    cycles: 16,
                    pc: address,
                }
            }
            Instruction::JR => {
                let offset = self.address_bus.read_byte(self.pc + 1) as i8;

                Step {
                    cycles: 12,
                    pc: self.pc.wrapping_add(2).wrapping_add(offset as u16),
                }
            }
            Instruction::JRF(comparison) => {
                // init assuming no branch
                let mut pc = self.pc.wrapping_add(2);
                let mut cycles = 8;

                match comparison {
                    Comparison::NONZERO => {
                        if !self.registers.flags.zero {
                            let offset = self.address_bus.read_byte(self.pc + 1) as i8;

                            pc = self.pc.wrapping_add(2).wrapping_add(offset as u16);
                            cycles = 12;
                        }
                    }
                    Comparison::ZERO => {
                        if self.registers.flags.zero {
                            let offset = self.address_bus.read_byte(self.pc + 1) as i8;

                            pc = self.pc.wrapping_add(2).wrapping_add(offset as u16);
                            cycles = 12;
                        }
                    }
                    _ => {
                        panic!("{:?} unimplemented JRF Instruction", comparison);
                    }
                }
                Step { cycles, pc }
            }
            Instruction::LDR8U8(target) => {
                let byte = self.address_bus.read_byte(self.pc + 1);
                match target {
                    TargetRegister8::A => {
                        self.registers.a = byte;
                    }
                    TargetRegister8::B => {
                        self.registers.b = byte;
                    }
                    TargetRegister8::C => {
                        self.registers.c = byte;
                    }
                    TargetRegister8::D => {
                        self.registers.d = byte;
                    }
                    TargetRegister8::E => {
                        self.registers.e = byte;
                    }
                    TargetRegister8::H => {
                        self.registers.h = byte;
                    }
                    TargetRegister8::L => {
                        self.registers.l = byte;
                    }
                }

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(2),
                }
            }
            Instruction::LDR8R8(target, source) => {
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

                Step {
                    cycles: 4,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::LDU16A => {
                let address = merge_bytes(
                    self.address_bus.read_byte(self.pc + 2),
                    self.address_bus.read_byte(self.pc + 1),
                );
                self.address_bus
                    .write_bytes(address, [self.registers.a].to_vec());

                Step {
                    cycles: 16,
                    pc: self.pc.wrapping_add(3),
                }
            }
            Instruction::RL(target) => {
                let cycles = 8;
                self.registers.flags.half_carry = false;
                self.registers.flags.negative = false;
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

                Step {
                    cycles,
                    pc: self.pc.wrapping_add(2),
                }
            }
            Instruction::RLA => {
                self.registers.flags.half_carry = false;
                self.registers.flags.negative = false;
                self.registers.flags.zero = false;

                let mut new_a = self.registers.a << 1;

                if self.registers.flags.carry {
                    new_a |= 1;
                }

                self.registers.flags.carry = self.registers.a >> 7 == 1;

                self.registers.a = new_a;

                Step {
                    cycles: 4,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::CPU8 => {
                let byte = self.address_bus.read_byte(self.pc + 1);
                let a = self.registers.a;

                self.registers.flags.negative = true;
                self.registers.flags.zero = a == byte;
                self.registers.flags.carry = a < byte;
                self.registers.flags.half_carry = bytes_half_carry(a, byte);

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(2),
                }
            }
            Instruction::CPHL => {
                let hl = self.registers.get_hl();
                let byte = self.address_bus.read_byte(hl);
                let a = self.registers.a;

                self.registers.flags.negative = true;
                self.registers.flags.zero = a == byte;
                self.registers.flags.carry = a < byte;
                self.registers.flags.half_carry = bytes_half_carry(a, byte);

                Step {
                    cycles: 8,
                    pc: self.pc.wrapping_add(1),
                }
            }
            Instruction::CALL => {
                self.registers.sp -= 2;
                let bytes = split_bytes(self.pc.wrapping_add(3));
                self.address_bus
                    .write_bytes(self.registers.sp, [bytes[1], bytes[0]].to_vec());
                let lower = self.address_bus.read_byte(self.pc + 1);
                let upper = self.address_bus.read_byte(self.pc + 2);
                let pc = merge_bytes(upper, lower);

                Step { cycles: 24, pc }
            }
            Instruction::RET => {
                let pc = merge_bytes(
                    self.address_bus.read_byte(self.registers.sp + 1),
                    self.address_bus.read_byte(self.registers.sp),
                );
                self.registers.sp += 2;

                Step { cycles: 16, pc }
            }
            _ => {
                panic!(
                    "{:?} unimplemented Instruction pc: {:x}",
                    instruction, self.pc
                );
            }
        }
    }

    pub fn step(&mut self) -> u32 {
        let mut instruction_byte = self.address_bus.read_byte(self.pc);

        let prefixed = instruction_byte == INSTRUCTION_PREFIX_BYTE;
        if prefixed {
            instruction_byte = self.address_bus.read_byte(self.pc + 1);
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if instruction == Instruction::NOP {
            return 1000;
        }
        let step = self.execute(instruction);
        self.pc = step.pc;
        step.cycles as u32
    }

    fn add(&mut self, value: u8, carry: bool) -> u8 {
        let (added, overflowed) = self.registers.a.carrying_add(value, carry);
        self.registers.flags.zero = added == 0;
        self.registers.flags.negative = false;
        self.registers.flags.carry = overflowed;
        self.registers.flags.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;

        added
    }

    fn sub(&mut self, value: u8, carry: bool) -> u8 {
        let (subbed, overflowed) = self.registers.a.borrowing_sub(value, carry);
        self.registers.flags.zero = subbed == 0;
        self.registers.flags.negative = true;
        self.registers.flags.carry = overflowed;
        (_, self.registers.flags.half_carry) =
            (self.registers.a & 0x0F).overflowing_sub(value & 0x0F);

        subbed
    }
}
