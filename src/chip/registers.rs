#![allow(dead_code)]

use crate::utils::*;

use super::instructions::TargetRegister8;

//documented gameboy registers
//f of the af register is represented
//by the special FlagsRegister
#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub flags: FlagsRegister,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            flags: FlagsRegister::from(0),
        }
    }

    pub fn get_from_enum(&mut self, target: &TargetRegister8) -> u8 {
        match target {
            TargetRegister8::A => self.a,
            TargetRegister8::B => self.b,
            TargetRegister8::C => self.c,
            TargetRegister8::D => self.d,
            TargetRegister8::E => self.e,
            TargetRegister8::H => self.h,
            TargetRegister8::L => self.l,
        }
    }

    pub fn set_from_enum(&mut self, target: &TargetRegister8, value: u8) {
        match target {
            TargetRegister8::A => self.a = value,
            TargetRegister8::B => self.b = value,
            TargetRegister8::C => self.c = value,
            TargetRegister8::D => self.d = value,
            TargetRegister8::E => self.e = value,
            TargetRegister8::H => self.h = value,
            TargetRegister8::L => self.l = value,
        }
    }

    pub fn get_af(&self) -> u16 {
        merge_bytes(self.a, u8::from(self.flags))
    }

    pub fn get_bc(&self) -> u16 {
        merge_bytes(self.b, self.c)
    }

    pub fn get_de(&self) -> u16 {
        merge_bytes(self.d, self.e)
    }

    pub fn get_hl(&self) -> u16 {
        merge_bytes(self.h, self.l)
    }

    pub fn get_f(&self) -> u8 {
        u8::from(self.flags)
    }

    pub fn set_af(&mut self, af: u16) {
        let (upper, lower) = split_bytes(af);
        self.a = upper;
        self.flags = FlagsRegister::from(lower);
    }

    pub fn set_bc(&mut self, bc: u16) {
        let (upper, lower) = split_bytes(bc);
        self.b = upper;
        self.c = lower;
    }

    pub fn set_de(&mut self, de: u16) {
        let (upper, lower) = split_bytes(de);
        self.d = upper;
        self.e = lower;
    }

    pub fn set_hl(&mut self, hl: u16) {
        let (upper, lower) = split_bytes(hl);
        self.h = upper;
        self.l = lower;
    }

    pub fn set_sp(&mut self, upper: u8, lower: u8) {
        self.sp = (upper as u16) << 8 | lower as u16;
    }
}

//this represents the lower 8 bits of our AF register
//since it serves a special case of the 4 upper bits being special flags
//the lower 4 bits are always set to 0 so they aren't represented here.
#[derive(Debug, Copy, Clone)]
pub struct FlagsRegister {
    pub zero: bool,
    pub negative: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const FLAGS_REGISTER_ZERO_BIT: u8 = 1 << 7;
const FLAGS_REGISTER_SUBTRACTION_BIT: u8 = 1 << 6;
const FLAGS_REGISTER_HALF_CARRY_BIT: u8 = 1 << 5;
const FLAGS_REGISTER_CARRY_BIT: u8 = 1 << 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {
            FLAGS_REGISTER_ZERO_BIT
        } else {
            0
        }) | (if flag.negative {
            FLAGS_REGISTER_SUBTRACTION_BIT
        } else {
            0
        }) | (if flag.half_carry {
            FLAGS_REGISTER_HALF_CARRY_BIT
        } else {
            0
        }) | (if flag.carry {
            FLAGS_REGISTER_CARRY_BIT
        } else {
            0
        })
    }
}

//convert a byte to our FlagRegister by shifting right for our flag register byte positions
//and checking if the bit is set.
impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = (byte & FLAGS_REGISTER_ZERO_BIT) != 0;
        let negative = (byte & FLAGS_REGISTER_SUBTRACTION_BIT) != 0;
        let half_carry = (byte & FLAGS_REGISTER_HALF_CARRY_BIT) != 0;
        let carry = (byte & FLAGS_REGISTER_CARRY_BIT) != 0;

        FlagsRegister {
            zero,
            negative,
            half_carry,
            carry,
        }
    }
}
