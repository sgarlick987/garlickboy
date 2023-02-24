#![allow(dead_code)]
pub mod instructions;
pub mod registers;

use instructions::{Instruction, TargetRegister8, INSTRUCTION_PREFIX_BYTE};
use registers::Registers;

#[derive(Debug)]
struct CPU {
    registers: Registers,
    bus: AddressBus,
    pc: u16,
}

#[derive(Debug)]
struct AddressBus {
    memory: [u8; 0xFFFF],
}

impl AddressBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::ADCR8(target) => match target {
                TargetRegister8::A => {
                    let stored = self.registers.a;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::B => {
                    let stored = self.registers.b;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::C => {
                    let stored = self.registers.c;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::D => {
                    let stored = self.registers.d;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::E => {
                    let stored = self.registers.e;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::H => {
                    let stored = self.registers.h;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::L => {
                    let stored = self.registers.l;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::ADDR8(target) => match target {
                TargetRegister8::A => {
                    let stored = self.registers.a;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::B => {
                    let stored = self.registers.b;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::C => {
                    let stored = self.registers.c;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::D => {
                    let stored = self.registers.d;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::E => {
                    let stored = self.registers.e;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::H => {
                    let stored = self.registers.h;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::L => {
                    let stored = self.registers.l;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
            },
            _ => {
                println!("{:?} unimplemented Instruction", instruction);
                self.pc
            }
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == INSTRUCTION_PREFIX_BYTE;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }

        self.pc = self.execute(instruction);
    }

    //handles the add Instructions by adding a given value to our a register
    //and setting appropiate flags.
    fn add(&mut self, value: u8, carry: bool) -> u8 {
        let (added, overflowed) = self.registers.a.carrying_add(value, carry);
        self.registers.flags.zero = added == 0;
        self.registers.flags.subtraction = false;
        self.registers.flags.carry = overflowed;
        //this is documented as taking the lower 4 bits of the a register and value add
        //checking if they are greater than 0x0F
        self.registers.flags.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;

        added
    }
}
