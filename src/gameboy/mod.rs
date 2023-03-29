#![allow(dead_code)]
mod bus;
pub mod gpu;
mod instructions;
mod interrupts;
mod registers;

use crate::{
    emu::{controller::Controller, display::Display, rom::Rom},
    utils::{add_bytes_half_carry, sub_bytes_half_carry},
};

use bus::Bus;
use instructions::*;
use interrupts::{InterruptHandler, IE_ADDRESS, IF_ADDRESS};
use registers::*;

use self::bus::new_address_bus;

pub type GameboyCycle = Box<dyn FnOnce(&mut Gameboy)>;

pub struct Gameboy {
    registers: Registers,
    interrupt_handler: InterruptHandler,
    timers: Timers,
    bus: Box<dyn Bus>,
    pc: u16,
    halted: bool,
    trace: bool,
}

struct Timers {
    ly: u8,
    div: u8,
    vblank: u8,
}

impl Gameboy {
    pub fn new() -> Self {
        let registers = Registers::new();
        let interrupt_handler = InterruptHandler::new();
        let bus = new_address_bus();
        let timers = Timers {
            ly: 0,
            vblank: 0,
            div: 0,
        };

        Self {
            registers,
            interrupt_handler,
            timers,
            bus,
            pc: 0,
            halted: false,
            trace: false,
        }
    }

    pub fn cycles(&mut self) -> Box<dyn Iterator<Item = GameboyCycle>> {
        let cycles = self.interrupt_handler.cycles();
        let has_interrupts = cycles.len() != 0;

        if has_interrupts {
            cycles
        } else {
            self.prefetch()
        }
    }

    pub fn execute(&mut self, step: GameboyCycle) {
        self.bus.update_dma();
        step(self);
        self.update_timers();
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        self.bus.load_rom(rom);
    }

    pub fn update_joypad(&mut self, controller: &Box<dyn Controller>) {
        self.bus.update_joypad(controller);
    }

    pub fn update_display(&mut self, display: &mut Box<dyn Display>) {
        self.bus.update_display(display);
    }

    fn prefetch(&mut self) -> Box<dyn Iterator<Item = GameboyCycle>> {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == BYTE_PREFIX;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }
        let instruction = if self.halted {
            if self.interrupt_handler.is_pending() {
                Instruction::NOP
            } else {
                Instruction::HALT
            }
        } else {
            Instruction::from_byte(instruction_byte, prefixed)
        };
        if self.pc == 0x0100 {
            self.trace = false;
        }
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if self.trace {
            println!("{:?} pc {:x}", instruction, self.pc);
            println!("registers before {:X?}", self.registers)
        }
        instruction.fetch()
    }

    fn update_timers(&mut self) {
        self.update_div_timer();
        self.update_ppu_timer();
    }

    fn update_div_timer(&mut self) {
        self.timers.div += 1;
        if self.timers.div == 64 {
            self.inc_div();
            self.timers.div = 0;
        }
    }

    fn update_ppu_timer(&mut self) {
        if self.lcd_is_enabled() {
            self.timers.ly += 1;
            if self.timers.ly == 114 {
                self.timers.ly = 0;
                self.inc_ly();
                self.timers.vblank += 1;
            }
            if self.timers.vblank == 145 {
                self.flag_vblank();
            }
            if self.timers.vblank == 155 {
                self.timers.vblank = 0;
            }
        } else {
            self.timers.ly = 0;
            self.timers.vblank = 0;
        }
    }

    fn inc_ly(&mut self) {
        self.bus.inc_ly();
    }

    fn lcd_is_enabled(&mut self) -> bool {
        self.bus.lcd_is_enabled()
    }

    fn inc_div(&mut self) {
        self.bus.inc_div();
    }

    fn flag_vblank(&mut self) {
        self.interrupt_handler.set_vblank_flag();
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            IF_ADDRESS => self.interrupt_handler.set_flags(byte),
            IE_ADDRESS => self.interrupt_handler.set_enable(byte),
            _ => self.bus.write_byte(address, byte),
        }
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            IF_ADDRESS => self.interrupt_handler.get_flags(),
            IE_ADDRESS => self.interrupt_handler.get_enable(),
            _ => self.bus.read_byte(address),
        }
    }

    fn read_byte_pc_lower(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(1))
    }

    fn read_byte_pc_upper(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(2))
    }

    fn carry_flag(&mut self) -> bool {
        self.registers.flags.carry
    }

    fn update_carry_flag(&mut self, carry: bool) {
        self.registers.flags.carry = carry;
    }

    fn set_carry_flag(&mut self) {
        self.update_carry_flag(true);
    }

    fn reset_carry_flag(&mut self) {
        self.update_carry_flag(false);
    }

    fn half_carry_flag(&mut self) -> bool {
        self.registers.flags.half_carry
    }

    fn update_half_carry_flag(&mut self, half_carry: bool) {
        self.registers.flags.half_carry = half_carry;
    }

    fn set_half_carry_flag(&mut self) {
        self.update_half_carry_flag(true);
    }

    fn reset_half_carry_flag(&mut self) {
        self.update_half_carry_flag(false);
    }

    fn zero_flag(&mut self) -> bool {
        self.registers.flags.zero
    }

    fn update_zero_flag(&mut self, zero: bool) {
        self.registers.flags.zero = zero;
    }

    fn set_zero_flag(&mut self) {
        self.update_zero_flag(true);
    }

    fn reset_zero_flag(&mut self) {
        self.update_zero_flag(false);
    }

    fn update_negative_flag(&mut self, negative: bool) {
        self.registers.flags.negative = negative;
    }

    fn negative_flag(&mut self) -> bool {
        self.registers.flags.negative
    }

    fn set_negative_flag(&mut self) {
        self.update_negative_flag(true);
    }

    fn reset_negative_flag(&mut self) {
        self.update_negative_flag(false);
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
        self.update_zero_flag(added == 0);
        self.reset_negative_flag();
        self.update_carry_flag(overflowed);
        self.update_half_carry_flag(add_bytes_half_carry(self.registers.a, value));

        added
    }

    fn sub(&mut self, value: u8, carry: bool) -> u8 {
        let (subbed, overflowed) = self.registers.a.borrowing_sub(value, carry);
        self.update_zero_flag(subbed == 0);
        self.set_negative_flag();
        self.update_carry_flag(overflowed);
        self.update_half_carry_flag(sub_bytes_half_carry(self.registers.a, value));

        subbed
    }
}
