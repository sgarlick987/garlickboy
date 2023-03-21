#![allow(dead_code)]
pub mod instructions;
pub mod interrupts;
pub mod registers;

use crate::{address::*, utils::split_bytes};

use self::{
    instructions::{execute::Execution, *},
    interrupts::{InterruptHandler, IE_ADDRESS, IF_ADDRESS},
    registers::*,
};

pub struct CPU {
    registers: Registers,
    interrupt_handler: InterruptHandler,
    bus: Box<dyn Bus>,
    pc: u16,
    print: bool,
}

impl CPU {
    pub fn new(bus: Box<dyn Bus>) -> CPU {
        let registers = Registers::new();
        let interrupt_handler = InterruptHandler::new();
        CPU {
            registers,
            interrupt_handler,
            bus,
            pc: 0,
            print: false,
        }
    }

    pub fn get_register_from_enum(&mut self, target: &TargetRegister8) -> u8 {
        match target {
            TargetRegister8::A => self.registers.a,
            TargetRegister8::B => self.registers.b,
            TargetRegister8::C => self.registers.c,
            TargetRegister8::D => self.registers.d,
            TargetRegister8::E => self.registers.e,
            TargetRegister8::H => self.registers.h,
            TargetRegister8::L => self.registers.l,
        }
    }

    pub fn set_register_from_enum(&mut self, target: &TargetRegister8, value: u8) {
        match target {
            TargetRegister8::A => self.registers.a = value,
            TargetRegister8::B => self.registers.b = value,
            TargetRegister8::C => self.registers.c = value,
            TargetRegister8::D => self.registers.d = value,
            TargetRegister8::E => self.registers.e = value,
            TargetRegister8::H => self.registers.h = value,
            TargetRegister8::L => self.registers.l = value,
        }
    }

    pub fn render(&mut self) {
        self.bus.render();
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            IF_ADDRESS => self.interrupt_handler.set_flags(byte),
            IE_ADDRESS => self.interrupt_handler.set_enable(byte),
            _ => self.bus.write_byte(address, byte),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            IF_ADDRESS => self.interrupt_handler.get_flags(),
            IE_ADDRESS => self.interrupt_handler.get_enable(),
            _ => self.bus.read_byte(address),
        }
    }

    pub fn read_byte_pc_lower(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(1))
    }

    pub fn read_byte_pc_upper(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(2))
    }

    pub fn write_bytes(&mut self, address: u16, bytes: Vec<u8>) {
        self.bus.write_bytes(address as usize, bytes);
    }

    pub fn write_bios(&mut self, bytes: [u8; 0x100]) {
        self.bus.write_boot(bytes);
    }

    fn sync(&mut self) -> u8 {
        let ly = self.bus.sync();
        if ly == 145 {
            self.interrupt_handler.flag_vblank();
        }
        4
    }

    pub fn step(&mut self) -> u8 {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == BYTE_PREFIX;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if self.pc == 0x02cd {
            self.print = false;
        }
        if self.print {
            println!("pc: {:x}, inst: {:?}", self.pc, instruction);
        }
        let interrupt_address = self.interrupt_handler.handle();
        if interrupt_address != 0x0000 {
            self.sync();
            self.sync();
            let (upper, lower) = split_bytes(self.pc);
            self._push(upper);
            self.sync();
            self._push(lower);
            self.sync();
            self.pc = interrupt_address;
            self.sync();
            return 20;
        }
        instruction.execute(self)
    }
}
