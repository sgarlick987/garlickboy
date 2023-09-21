use super::{Gameboy, GameboyCycle, GameboyCycles};
use crate::utils::split_bytes;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub const IF_ADDRESS: u16 = 0xFF0F;
pub const IE_ADDRESS: u16 = 0xFFFF;

const VBLANK_JUMP_ADDRESS: u16 = 0x0040;
const LCD_STATE_JUMP_ADDRESS: u16 = 0x0048;
const TIMER_JUMP_ADDRESS: u16 = 0x0050;
const SERIAL_JUMP_ADDRESS: u16 = 0x0058;
const JOYPAD_JUMP_ADDRESS: u16 = 0x0060;

#[derive(Eq, Hash, PartialEq, Debug)]
enum MasterEnable {
    Disabled,
    ScheduledInitial,
    ScheduledNext,
    Enabled,
}

impl MasterEnable {
    fn is_enabled(&self) -> bool {
        matches!(self, MasterEnable::Enabled)
    }

    fn next(&self) -> Self {
        match self {
            MasterEnable::Disabled => MasterEnable::Disabled,
            MasterEnable::ScheduledInitial => MasterEnable::ScheduledNext,
            MasterEnable::ScheduledNext => MasterEnable::Enabled,
            MasterEnable::Enabled => MasterEnable::Enabled,
        }
    }
}

pub struct InterruptHandler {
    ime: MasterEnable,
    flags: Interrupts,
    enable: Interrupts,
    enable_byte: u8,
}

impl InterruptHandler {
    pub fn new() -> Self {
        let ime = MasterEnable::Disabled;
        let flags = Interrupts::new();
        let enable = Interrupts::new();

        Self {
            ime,
            flags,
            enable,
            enable_byte: 0,
        }
    }

    pub fn set_vblank_flag(&mut self) {
        self.flags.vblank = true;
    }

    pub fn set_timer_flag(&mut self) {
        if self.ime.is_enabled() {
            self.flags.timer = true;
        }
    }

    pub fn set_joypad_flag(&mut self) {
        self.flags.joypad = true;
    }

    pub fn set_lcd_stat_flag(&mut self) {
        self.flags.lcd_stat = true;
    }

    pub fn write_flags(&mut self, byte: u8) {
        self.flags = Interrupts::from(byte);
    }

    pub fn read_flags(&self) -> u8 {
        u8::from(self.flags) | 0b111_00000
    }

    pub fn write_enable(&mut self, byte: u8) {
        self.enable_byte = byte;
        self.enable = Interrupts::from(byte);
    }

    pub fn read_enable(&self) -> u8 {
        self.enable_byte
    }

    pub fn disable_ime(&mut self) {
        self.ime = MasterEnable::Disabled;
    }

    pub fn schedule_ime(&mut self) {
        if self.ime == MasterEnable::Disabled {
            self.ime = MasterEnable::ScheduledInitial;
        }
    }

    pub fn update_ime(&mut self) {
        self.ime = self.ime.next();
    }

    pub fn is_pending(&self) -> bool {
        u8::from(self.enable) & u8::from(self.flags) != 0
    }

    pub fn next_cycles(&mut self) -> GameboyCycles {
        if self.ime.is_enabled() {
            if self.flags.vblank && self.enable.vblank {
                self.flags.vblank = false;
                self.disable_ime();
                return new_interrupt_cycles(VBLANK_JUMP_ADDRESS);
            }
            if self.flags.lcd_stat && self.enable.lcd_stat {
                self.flags.lcd_stat = false;
                self.disable_ime();
                return new_interrupt_cycles(LCD_STATE_JUMP_ADDRESS);
            }
            if self.flags.timer && self.enable.timer {
                self.flags.timer = false;
                self.disable_ime();
                return new_interrupt_cycles(TIMER_JUMP_ADDRESS);
            }
            if self.flags.serial && self.enable.serial {
                self.flags.serial = false;
                self.disable_ime();
                return new_interrupt_cycles(SERIAL_JUMP_ADDRESS);
            }
            if self.flags.joypad && self.enable.joypad {
                self.flags.joypad = false;
                self.disable_ime();
                return new_interrupt_cycles(JOYPAD_JUMP_ADDRESS);
            }
        }

        Box::new(VecDeque::with_capacity(0).into_iter())
    }
}

#[derive(Clone, Copy)]
struct Interrupts {
    vblank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Interrupts {
    fn new() -> Interrupts {
        Interrupts::from(0)
    }
}

const INTERRUPTS_VBLANK_BIT: u8 = 1;
const INTERRUPTS_LCD_STAT_BIT: u8 = 1 << 1;
const INTERRUPTS_TIMER_BIT: u8 = 1 << 2;
const INTERRUPTS_SERIAL_BIT: u8 = 1 << 3;
const INTERRUPTS_JOYPAD_BIT: u8 = 1 << 4;

impl std::convert::From<Interrupts> for u8 {
    fn from(interrupts: Interrupts) -> u8 {
        (if interrupts.vblank {
            INTERRUPTS_VBLANK_BIT
        } else {
            0
        }) | (if interrupts.lcd_stat {
            INTERRUPTS_LCD_STAT_BIT
        } else {
            0
        }) | (if interrupts.timer {
            INTERRUPTS_TIMER_BIT
        } else {
            0
        }) | (if interrupts.serial {
            INTERRUPTS_SERIAL_BIT
        } else {
            0
        } | (if interrupts.joypad {
            INTERRUPTS_JOYPAD_BIT
        } else {
            0
        }))
    }
}

impl std::convert::From<u8> for Interrupts {
    fn from(byte: u8) -> Self {
        let vblank = (byte & INTERRUPTS_VBLANK_BIT) != 0;
        let lcd_stat = (byte & INTERRUPTS_LCD_STAT_BIT) != 0;
        let timer = (byte & INTERRUPTS_TIMER_BIT) != 0;
        let serial = (byte & INTERRUPTS_SERIAL_BIT) != 0;
        let joypad = (byte & INTERRUPTS_JOYPAD_BIT) != 0;

        Self {
            vblank,
            lcd_stat,
            timer,
            serial,
            joypad,
        }
    }
}

struct Context {
    upper: u8,
    lower: u8,
}

fn new_interrupt_cycles(address: u16) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(5);
    let context = Rc::new(RefCell::new(Context { upper: 0, lower: 0 }));

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //nop
    }));

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //nop
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        (context.upper, context.lower) = split_bytes(gameboy.pc);
        gameboy.push(context.upper);
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.push(context.lower);
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| gameboy.pc = address));

    Box::new(cycles.into_iter())
}
