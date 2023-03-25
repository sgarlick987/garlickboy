#![allow(dead_code)]
pub mod bios;
pub mod bus;
pub mod gpu;
pub mod instructions;
pub mod interrupts;
pub mod joypad;
pub mod registers;

use crate::{controller::Controller, display::Display};

use self::{
    bus::Bus,
    instructions::*,
    interrupts::{InterruptHandler, IE_ADDRESS, IF_ADDRESS},
    registers::*,
};

impl GameboyChip {
    pub fn fetch(&mut self) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == BYTE_PREFIX;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if self.print {
            println!("{:?} pc {:x}", instruction, self.pc);
        }
        instruction.fetch()
    }

    pub fn execute(&mut self, step: Box<dyn FnOnce(&mut GameboyChip)>) {
        step(self);
    }

    pub fn update_joypad(&mut self, controller: &Controller) {
        self.bus.update_joypad(controller);
    }

    pub fn update_display(&mut self, display: &mut Display) {
        self.bus.update_display(display);
    }

    pub fn inc_ly(&mut self) {
        self.bus.inc_ly();
    }

    pub fn lcd_is_enabled(&mut self) -> bool {
        self.bus.lcd_is_enabled()
    }
}

pub struct GameboyChip {
    pub registers: Registers,
    interrupt_handler: InterruptHandler,
    bus: Box<dyn Bus>,
    pc: u16,
    print: bool,
}

impl GameboyChip {
    pub fn new(bus: Box<dyn Bus>) -> GameboyChip {
        let registers = Registers::new();
        let interrupt_handler = InterruptHandler::new();
        GameboyChip {
            registers,
            interrupt_handler,
            bus,
            pc: 0,
            print: false,
        }
    }

    pub fn interrupts(
        &mut self,
    ) -> Box<dyn ExactSizeIterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
        self.interrupt_handler.step()
    }

    pub fn flag_vblank(&mut self) {
        self.interrupt_handler.flag_vblank();
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

    fn push(&mut self, byte: u8) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, byte);
    }

    fn pop(&mut self) -> u8 {
        let byte = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        byte
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
