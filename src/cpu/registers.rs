#![allow(dead_code)]

use crate::utils::*;

#[derive(Debug)]
//documented gameboy registers
//f of the af register is represented
//by the special FlagsRegister
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

pub fn new_registers() -> Registers {
    Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        sp: 0,
        flags: FlagsRegister {
            zero: false,
            negative: false,
            half_carry: false,
            carry: false,
        },
    }
}

impl Registers {
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
        let bytes = split_bytes(af);
        self.a = bytes[0];
        self.flags = FlagsRegister::from(bytes[1]);
    }

    pub fn set_bc(&mut self, bc: u16) {
        let bytes = split_bytes(bc);
        self.b = bytes[0];
        self.c = bytes[1];
    }

    pub fn set_de(&mut self, de: u16) {
        let bytes = split_bytes(de);
        self.d = bytes[0];
        self.e = bytes[1];
    }

    pub fn set_hl(&mut self, hl: u16) {
        let bytes = split_bytes(hl);
        self.h = bytes[0];
        self.l = bytes[1];
    }

    pub fn set_sp(&mut self, upper: u8, lower: u8) {
        self.sp = (upper as u16) << 8 | lower as u16;
    }
}

//this represents the lower 8 bits of our AF register
//since it serves a special case of the 4 upper bits being special flags
//the lower 4 bits are always set to 0 so they aren't represented here.
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    pub negative: bool,
    pub half_carry: bool,
    pub carry: bool,
}

//this values are listed in the docs only the upper 4 bits are used with lower 4 always being 0
//each value is a byte with the documented byte position set to 1 so it can be used to with
//bitwise & to determine if a specific flag on our flag register is set.
const FLAGS_REGISTER_ZERO_BYTE: u8 = 0b10000000;
const FLAGS_REGISTER_SUBTRACTION_BYTE: u8 = 0b01000000;
const FLAGS_REGISTER_HALF_CARRY_BYTE: u8 = 0b00100000;
const FLAGS_REGISTER_CARRY_BYTE: u8 = 0b00010000;

//convert our FlagsRegister to a u8 by bitwise oring each flags byte consant when their corresponding boolean is set
//lastly shift left by 4 so that we have a u8 with upper 4 bits set to our flags.
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {
            FLAGS_REGISTER_ZERO_BYTE
        } else {
            0
        }) | (if flag.negative {
            FLAGS_REGISTER_SUBTRACTION_BYTE
        } else {
            0
        }) | (if flag.half_carry {
            FLAGS_REGISTER_HALF_CARRY_BYTE
        } else {
            0
        }) | (if flag.carry {
            FLAGS_REGISTER_CARRY_BYTE
        } else {
            0
        }) << 4
    }
}

//convert a byte to our FlagRegister by shifting right for our flag register byte positions
//and checking if the bit is set.
impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = (byte & FLAGS_REGISTER_ZERO_BYTE) != 0;
        let subtraction = (byte & FLAGS_REGISTER_SUBTRACTION_BYTE) != 0;
        let half_carry = (byte & FLAGS_REGISTER_HALF_CARRY_BYTE) != 0;
        let carry = (byte & FLAGS_REGISTER_CARRY_BYTE) != 0;

        FlagsRegister {
            zero,
            negative: subtraction,
            half_carry,
            carry,
        }
    }
}
