#![allow(dead_code)]
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

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    fn set_bc(&mut self, bc: u16) {
        self.b = ((bc & 0xFF00) >> 8) as u8;
        self.c = (bc & 0x00FF) as u8
    }
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    fn set_de(&mut self, de: u16) {
        self.d = ((de & 0xFF00) >> 8) as u8;
        self.e = (de & 0x00FF) as u8
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    fn set_hl(&mut self, hl: u16) {
        self.h = ((hl & 0xFF00) >> 8) as u8;
        self.l = (hl & 0x00FF) as u8
    }
}

//this represents the lower 8 bits of our AF register
//since it serves a special case of the 4 upper bits being special flags
//the lower 4 bits are always set to 0 so they aren't represented here.
#[derive(Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtraction: bool,
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
        }) | (if flag.subtraction {
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
            subtraction,
            half_carry,
            carry,
        }
    }
}
