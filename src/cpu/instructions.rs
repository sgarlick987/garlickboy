#![allow(dead_code)]
pub const INSTRUCTION_PREFIX_BYTE: u8 = 0xCB;
pub const UNASSIGNED_INSTRUCTION_BYTES: [u8; 11] = [
    0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
];

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Instruction {
    //arithmetic and logic
    ADCR8(TargetRegister8),
    ADCHL,
    ADDSP,
    ADCU8,

    ADDR8(TargetRegister8),
    ADDR16(TargetRegister16),
    ADDHL,
    ADDU8,

    ANDR8(TargetRegister8),
    ANDHL,
    ANDU8,

    CPR8(TargetRegister8),
    CPHL,
    CPU8,

    DEC(IncDecTarget),
    INC(IncDecTarget),

    ORR8(TargetRegister8),
    ORHL,
    ORU8,

    SBCR8(TargetRegister8),
    SBCHL,
    SBCU8,

    SUBR8(TargetRegister8),
    SUBHL,
    SUBU8,

    XORR8(TargetRegister8),
    XORHL,
    XORU8,

    CPL,
    DAA,

    //cpu control
    CCF,
    HALT,
    NOP,
    SCF,
    STOP,
    DI,
    EI,

    //jump
    JP,
    JPF(Comparison),
    JPHL,
    JR,
    JRF(Comparison),
    CALL,
    CALLF(Comparison),
    RET,
    RETI,
    RETF(Comparison),
    RST00H,
    RST10H,
    RST30H,
    RST08H,
    RST18H,
    RST28H,
    RST38H,

    //load
    LDR8R8(TargetRegister8, TargetRegister8),
    LDR8U8(TargetRegister8),
    LDR8HL(TargetRegister8),
    LDHLR8(TargetRegister8),
    LDHLU8,
    LD(TargetRegister8, TargetRegister8),
    LDABC,
    LDADE,
    LDAU16,
    LDBCA,
    LDDEA,
    LDU16A,
    LDIHLA,
    LDIAHL,
    LDDHLA,
    LDDAHL,
    LDAFF00U16,
    LDFF00U16A,
    LDAFF00U8,
    LDAFF00U8A,
    LDAFF00C,
    LDFF00CA,
    POP(TargetRegister16),
    PUSH(TargetRegister16),

    //single bit ops
    BITHL,
    RESHL,
    SETHL,
    BIT(TargetRegister8),
    RES(TargetRegister8),
    SET(TargetRegister8),

    //rotate and shift
    RLA,
    RLCA,
    RRCA,
    RRA,
    RLHL,
    RLCHL,
    RRHL,
    RRCHL,
    SLAHL,
    SRAHL,
    SRLHL,
    SWAPHL,
    RL(TargetRegister8),
    RLC(TargetRegister8),
    RR(TargetRegister8),
    RRC(TargetRegister8),
    SLA(TargetRegister8),
    SRA(TargetRegister8),
    SRL(TargetRegister8),
    SWAP(TargetRegister8),

    //placeholder unimpl
    UNIMPLEMENTED,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Comparison {
    NONZERO,
    NOCARRY,
    ZERO,
    CARRY,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum TargetRegister8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum TargetRegister16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    SP,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) ->Instruction {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::RLC(TargetRegister8::B),
            0x01 => Instruction::RLC(TargetRegister8::C),
            0x02 => Instruction::RLC(TargetRegister8::D),
            0x03 => Instruction::RLC(TargetRegister8::E),
            0x04 => Instruction::RLC(TargetRegister8::H),
            0x05 => Instruction::RLC(TargetRegister8::L),
            0x06 => Instruction::RLCHL,
            0x07 => Instruction::RLC(TargetRegister8::A),
            0x08 => Instruction::RRC(TargetRegister8::B),
            0x09 => Instruction::RRC(TargetRegister8::C),
            0x0A => Instruction::RRC(TargetRegister8::D),
            0x0B => Instruction::RRC(TargetRegister8::E),
            0x0C => Instruction::RRC(TargetRegister8::H),
            0x0D => Instruction::RRC(TargetRegister8::L),
            0x0E => Instruction::RRCHL,
            0x0F => Instruction::RRC(TargetRegister8::A),

            0x10 => Instruction::RL(TargetRegister8::B),
            0x11 => Instruction::RL(TargetRegister8::C),
            0x12 => Instruction::RL(TargetRegister8::D),
            0x13 => Instruction::RL(TargetRegister8::E),
            0x14 => Instruction::RL(TargetRegister8::H),
            0x15 => Instruction::RL(TargetRegister8::L),
            0x16 => Instruction::RLHL,
            0x17 => Instruction::RL(TargetRegister8::A),
            0x18 => Instruction::RR(TargetRegister8::B),
            0x19 => Instruction::RR(TargetRegister8::C),
            0x1A => Instruction::RR(TargetRegister8::D),
            0x1B => Instruction::RR(TargetRegister8::E),
            0x1C => Instruction::RR(TargetRegister8::H),
            0x1D => Instruction::RR(TargetRegister8::L),
            0x1E => Instruction::RRHL,
            0x1F => Instruction::RR(TargetRegister8::A),

            0x20 => Instruction::UNIMPLEMENTED,
            0x21 => Instruction::UNIMPLEMENTED,
            0x22 => Instruction::UNIMPLEMENTED,
            0x23 => Instruction::UNIMPLEMENTED,
            0x24 => Instruction::UNIMPLEMENTED,
            0x25 => Instruction::UNIMPLEMENTED,
            0x26 => Instruction::UNIMPLEMENTED,
            0x27 => Instruction::UNIMPLEMENTED,
            0x28 => Instruction::UNIMPLEMENTED,
            0x29 => Instruction::UNIMPLEMENTED,
            0x2A => Instruction::UNIMPLEMENTED,
            0x2B => Instruction::UNIMPLEMENTED,
            0x2C => Instruction::UNIMPLEMENTED,
            0x2D => Instruction::UNIMPLEMENTED,
            0x2E => Instruction::UNIMPLEMENTED,
            0x2F => Instruction::UNIMPLEMENTED,

            0x30 => Instruction::UNIMPLEMENTED,
            0x31 => Instruction::UNIMPLEMENTED,
            0x32 => Instruction::UNIMPLEMENTED,
            0x33 => Instruction::UNIMPLEMENTED,
            0x34 => Instruction::UNIMPLEMENTED,
            0x35 => Instruction::UNIMPLEMENTED,
            0x36 => Instruction::UNIMPLEMENTED,
            0x37 => Instruction::UNIMPLEMENTED,
            0x38 => Instruction::UNIMPLEMENTED,
            0x39 => Instruction::UNIMPLEMENTED,
            0x3A => Instruction::UNIMPLEMENTED,
            0x3B => Instruction::UNIMPLEMENTED,
            0x3C => Instruction::UNIMPLEMENTED,
            0x3D => Instruction::UNIMPLEMENTED,
            0x3E => Instruction::UNIMPLEMENTED,
            0x3F => Instruction::UNIMPLEMENTED,

            0x40 => Instruction::UNIMPLEMENTED,
            0x41 => Instruction::UNIMPLEMENTED,
            0x42 => Instruction::UNIMPLEMENTED,
            0x43 => Instruction::UNIMPLEMENTED,
            0x44 => Instruction::UNIMPLEMENTED,
            0x45 => Instruction::UNIMPLEMENTED,
            0x46 => Instruction::UNIMPLEMENTED,
            0x47 => Instruction::UNIMPLEMENTED,
            0x48 => Instruction::UNIMPLEMENTED,
            0x49 => Instruction::UNIMPLEMENTED,
            0x4A => Instruction::UNIMPLEMENTED,
            0x4B => Instruction::UNIMPLEMENTED,
            0x4C => Instruction::UNIMPLEMENTED,
            0x4D => Instruction::UNIMPLEMENTED,
            0x4E => Instruction::UNIMPLEMENTED,
            0x4F => Instruction::UNIMPLEMENTED,

            0x50 => Instruction::UNIMPLEMENTED,
            0x51 => Instruction::UNIMPLEMENTED,
            0x52 => Instruction::UNIMPLEMENTED,
            0x53 => Instruction::UNIMPLEMENTED,
            0x54 => Instruction::UNIMPLEMENTED,
            0x55 => Instruction::UNIMPLEMENTED,
            0x56 => Instruction::UNIMPLEMENTED,
            0x57 => Instruction::UNIMPLEMENTED,
            0x58 => Instruction::UNIMPLEMENTED,
            0x59 => Instruction::UNIMPLEMENTED,
            0x5A => Instruction::UNIMPLEMENTED,
            0x5B => Instruction::UNIMPLEMENTED,
            0x5C => Instruction::UNIMPLEMENTED,
            0x5D => Instruction::UNIMPLEMENTED,
            0x5E => Instruction::UNIMPLEMENTED,
            0x5F => Instruction::UNIMPLEMENTED,

            0x60 => Instruction::UNIMPLEMENTED,
            0x61 => Instruction::UNIMPLEMENTED,
            0x62 => Instruction::UNIMPLEMENTED,
            0x63 => Instruction::UNIMPLEMENTED,
            0x64 => Instruction::UNIMPLEMENTED,
            0x65 => Instruction::UNIMPLEMENTED,
            0x66 => Instruction::UNIMPLEMENTED,
            0x67 => Instruction::UNIMPLEMENTED,
            0x68 => Instruction::UNIMPLEMENTED,
            0x69 => Instruction::UNIMPLEMENTED,
            0x6A => Instruction::UNIMPLEMENTED,
            0x6B => Instruction::UNIMPLEMENTED,
            0x6C => Instruction::UNIMPLEMENTED,
            0x6D => Instruction::UNIMPLEMENTED,
            0x6E => Instruction::UNIMPLEMENTED,
            0x6F => Instruction::UNIMPLEMENTED,

            0x70 => Instruction::UNIMPLEMENTED,
            0x71 => Instruction::UNIMPLEMENTED,
            0x72 => Instruction::UNIMPLEMENTED,
            0x73 => Instruction::UNIMPLEMENTED,
            0x74 => Instruction::UNIMPLEMENTED,
            0x75 => Instruction::UNIMPLEMENTED,
            0x76 => Instruction::UNIMPLEMENTED,
            0x77 => Instruction::UNIMPLEMENTED,
            0x78 => Instruction::UNIMPLEMENTED,
            0x79 => Instruction::UNIMPLEMENTED,
            0x7A => Instruction::UNIMPLEMENTED,
            0x7B => Instruction::UNIMPLEMENTED,
            0x7C => Instruction::UNIMPLEMENTED,
            0x7D => Instruction::UNIMPLEMENTED,
            0x7E => Instruction::UNIMPLEMENTED,
            0x7F => Instruction::UNIMPLEMENTED,

            0x80 => Instruction::UNIMPLEMENTED,
            0x81 => Instruction::UNIMPLEMENTED,
            0x82 => Instruction::UNIMPLEMENTED,
            0x83 => Instruction::UNIMPLEMENTED,
            0x84 => Instruction::UNIMPLEMENTED,
            0x85 => Instruction::UNIMPLEMENTED,
            0x86 => Instruction::UNIMPLEMENTED,
            0x87 => Instruction::UNIMPLEMENTED,
            0x88 => Instruction::UNIMPLEMENTED,
            0x89 => Instruction::UNIMPLEMENTED,
            0x8A => Instruction::UNIMPLEMENTED,
            0x8B => Instruction::UNIMPLEMENTED,
            0x8C => Instruction::UNIMPLEMENTED,
            0x8D => Instruction::UNIMPLEMENTED,
            0x8E => Instruction::UNIMPLEMENTED,
            0x8F => Instruction::UNIMPLEMENTED,

            0x90 => Instruction::UNIMPLEMENTED,
            0x91 => Instruction::UNIMPLEMENTED,
            0x92 => Instruction::UNIMPLEMENTED,
            0x93 => Instruction::UNIMPLEMENTED,
            0x94 => Instruction::UNIMPLEMENTED,
            0x95 => Instruction::UNIMPLEMENTED,
            0x96 => Instruction::UNIMPLEMENTED,
            0x97 => Instruction::UNIMPLEMENTED,
            0x98 => Instruction::UNIMPLEMENTED,
            0x99 => Instruction::UNIMPLEMENTED,
            0x9A => Instruction::UNIMPLEMENTED,
            0x9B => Instruction::UNIMPLEMENTED,
            0x9C => Instruction::UNIMPLEMENTED,
            0x9D => Instruction::UNIMPLEMENTED,
            0x9E => Instruction::UNIMPLEMENTED,
            0x9F => Instruction::UNIMPLEMENTED,

            0xA0 => Instruction::UNIMPLEMENTED,
            0xA1 => Instruction::UNIMPLEMENTED,
            0xA2 => Instruction::UNIMPLEMENTED,
            0xA3 => Instruction::UNIMPLEMENTED,
            0xA4 => Instruction::UNIMPLEMENTED,
            0xA5 => Instruction::UNIMPLEMENTED,
            0xA6 => Instruction::UNIMPLEMENTED,
            0xA7 => Instruction::UNIMPLEMENTED,
            0xA8 => Instruction::UNIMPLEMENTED,
            0xA9 => Instruction::UNIMPLEMENTED,
            0xAA => Instruction::UNIMPLEMENTED,
            0xAB => Instruction::UNIMPLEMENTED,
            0xAC => Instruction::UNIMPLEMENTED,
            0xAD => Instruction::UNIMPLEMENTED,
            0xAE => Instruction::UNIMPLEMENTED,
            0xAF => Instruction::UNIMPLEMENTED,

            0xB0 => Instruction::UNIMPLEMENTED,
            0xB1 => Instruction::UNIMPLEMENTED,
            0xB2 => Instruction::UNIMPLEMENTED,
            0xB3 => Instruction::UNIMPLEMENTED,
            0xB4 => Instruction::UNIMPLEMENTED,
            0xB5 => Instruction::UNIMPLEMENTED,
            0xB6 => Instruction::UNIMPLEMENTED,
            0xB7 => Instruction::UNIMPLEMENTED,
            0xB8 => Instruction::UNIMPLEMENTED,
            0xB9 => Instruction::UNIMPLEMENTED,
            0xBA => Instruction::UNIMPLEMENTED,
            0xBB => Instruction::UNIMPLEMENTED,
            0xBC => Instruction::UNIMPLEMENTED,
            0xBD => Instruction::UNIMPLEMENTED,
            0xBE => Instruction::UNIMPLEMENTED,
            0xBF => Instruction::UNIMPLEMENTED,

            0xC0 => Instruction::UNIMPLEMENTED,
            0xC1 => Instruction::UNIMPLEMENTED,
            0xC2 => Instruction::UNIMPLEMENTED,
            0xC3 => Instruction::UNIMPLEMENTED,
            0xC4 => Instruction::UNIMPLEMENTED,
            0xC5 => Instruction::UNIMPLEMENTED,
            0xC6 => Instruction::UNIMPLEMENTED,
            0xC7 => Instruction::UNIMPLEMENTED,
            0xC8 => Instruction::UNIMPLEMENTED,
            0xC9 => Instruction::UNIMPLEMENTED,
            0xCA => Instruction::UNIMPLEMENTED,
            0xCB => Instruction::UNIMPLEMENTED,
            0xCC => Instruction::UNIMPLEMENTED,
            0xCD => Instruction::UNIMPLEMENTED,
            0xCE => Instruction::UNIMPLEMENTED,
            0xCF => Instruction::UNIMPLEMENTED,

            0xD0 => Instruction::UNIMPLEMENTED,
            0xD1 => Instruction::UNIMPLEMENTED,
            0xD2 => Instruction::UNIMPLEMENTED,
            0xD3 => Instruction::UNIMPLEMENTED,
            0xD4 => Instruction::UNIMPLEMENTED,
            0xD5 => Instruction::UNIMPLEMENTED,
            0xD6 => Instruction::UNIMPLEMENTED,
            0xD7 => Instruction::UNIMPLEMENTED,
            0xD8 => Instruction::UNIMPLEMENTED,
            0xD9 => Instruction::UNIMPLEMENTED,
            0xDA => Instruction::UNIMPLEMENTED,
            0xDB => Instruction::UNIMPLEMENTED,
            0xDC => Instruction::UNIMPLEMENTED,
            0xDD => Instruction::UNIMPLEMENTED,
            0xDE => Instruction::UNIMPLEMENTED,
            0xDF => Instruction::UNIMPLEMENTED,

            0xE0 => Instruction::UNIMPLEMENTED,
            0xE1 => Instruction::UNIMPLEMENTED,
            0xE2 => Instruction::UNIMPLEMENTED,
            0xE3 => Instruction::UNIMPLEMENTED,
            0xE4 => Instruction::UNIMPLEMENTED,
            0xE5 => Instruction::UNIMPLEMENTED,
            0xE6 => Instruction::UNIMPLEMENTED,
            0xE7 => Instruction::UNIMPLEMENTED,
            0xE8 => Instruction::UNIMPLEMENTED,
            0xE9 => Instruction::UNIMPLEMENTED,
            0xEA => Instruction::UNIMPLEMENTED,
            0xEB => Instruction::UNIMPLEMENTED,
            0xEC => Instruction::UNIMPLEMENTED,
            0xED => Instruction::UNIMPLEMENTED,
            0xEE => Instruction::UNIMPLEMENTED,
            0xEF => Instruction::UNIMPLEMENTED,

            0xF0 => Instruction::UNIMPLEMENTED,
            0xF1 => Instruction::UNIMPLEMENTED,
            0xF2 => Instruction::UNIMPLEMENTED,
            0xF3 => Instruction::UNIMPLEMENTED,
            0xF4 => Instruction::UNIMPLEMENTED,
            0xF5 => Instruction::UNIMPLEMENTED,
            0xF6 => Instruction::UNIMPLEMENTED,
            0xF7 => Instruction::UNIMPLEMENTED,
            0xF8 => Instruction::UNIMPLEMENTED,
            0xF9 => Instruction::UNIMPLEMENTED,
            0xFA => Instruction::UNIMPLEMENTED,
            0xFB => Instruction::UNIMPLEMENTED,
            0xFC => Instruction::UNIMPLEMENTED,
            0xFD => Instruction::UNIMPLEMENTED,
            0xFE => Instruction::UNIMPLEMENTED,
            0xFF => Instruction::UNIMPLEMENTED,
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::NOP,
            0x01 => Instruction::UNIMPLEMENTED,
            0x02 => Instruction::LDBCA,
            0x03 => Instruction::INC(IncDecTarget::BC),
            0x04 => Instruction::INC(IncDecTarget::B),
            0x05 => Instruction::DEC(IncDecTarget::B),
            0x06 => Instruction::UNIMPLEMENTED,
            0x07 => Instruction::RLCA,
            0x08 => Instruction::UNIMPLEMENTED,
            0x09 => Instruction::ADDR16(TargetRegister16::BC),
            0x0A => Instruction::LDABC,
            0x0B => Instruction::DEC(IncDecTarget::BC),
            0x0C => Instruction::INC(IncDecTarget::C),
            0x0D => Instruction::DEC(IncDecTarget::C),
            0x0E => Instruction::UNIMPLEMENTED,
            0x0F => Instruction::RRCA,

            0x10 => Instruction::STOP,
            0x11 => Instruction::UNIMPLEMENTED,
            0x12 => Instruction::LDDEA,
            0x13 => Instruction::INC(IncDecTarget::DE),
            0x14 => Instruction::INC(IncDecTarget::D),
            0x15 => Instruction::DEC(IncDecTarget::D),
            0x16 => Instruction::UNIMPLEMENTED,
            0x17 => Instruction::RLA,
            0x18 => Instruction::JR,
            0x19 => Instruction::ADDR16(TargetRegister16::DE),
            0x1A => Instruction::LDADE,
            0x1B => Instruction::DEC(IncDecTarget::DE),
            0x1C => Instruction::INC(IncDecTarget::E),
            0x1D => Instruction::DEC(IncDecTarget::E),
            0x1E => Instruction::UNIMPLEMENTED,
            0x1F => Instruction::RRA,

            0x20 => Instruction::UNIMPLEMENTED,
            0x21 => Instruction::UNIMPLEMENTED,
            0x22 => Instruction::UNIMPLEMENTED,
            0x23 => Instruction::INC(IncDecTarget::HL),
            0x24 => Instruction::INC(IncDecTarget::H),
            0x25 => Instruction::DEC(IncDecTarget::H),
            0x26 => Instruction::UNIMPLEMENTED,
            0x27 => Instruction::DAA,
            0x28 => Instruction::UNIMPLEMENTED,
            0x29 => Instruction::ADDR16(TargetRegister16::HL),
            0x2A => Instruction::UNIMPLEMENTED,
            0x2B => Instruction::UNIMPLEMENTED,
            0x2C => Instruction::UNIMPLEMENTED,
            0x2D => Instruction::UNIMPLEMENTED,
            0x2E => Instruction::UNIMPLEMENTED,
            0x2F => Instruction::CPL,

            0x30 => Instruction::UNIMPLEMENTED,
            0x31 => Instruction::UNIMPLEMENTED,
            0x32 => Instruction::UNIMPLEMENTED,
            0x33 => Instruction::UNIMPLEMENTED,
            0x34 => Instruction::UNIMPLEMENTED,
            0x35 => Instruction::UNIMPLEMENTED,
            0x36 => Instruction::UNIMPLEMENTED,
            0x37 => Instruction::UNIMPLEMENTED,
            0x38 => Instruction::UNIMPLEMENTED,
            0x39 => Instruction::UNIMPLEMENTED,
            0x3A => Instruction::UNIMPLEMENTED,
            0x3B => Instruction::UNIMPLEMENTED,
            0x3C => Instruction::UNIMPLEMENTED,
            0x3D => Instruction::UNIMPLEMENTED,
            0x3E => Instruction::UNIMPLEMENTED,
            0x3F => Instruction::CCF,

            0x40 => Instruction::LD(TargetRegister8::B, TargetRegister8::B), // LD B, B
            0x41 => Instruction::LD(TargetRegister8::B, TargetRegister8::C), // LD B, C
            0x42 => Instruction::LD(TargetRegister8::B, TargetRegister8::D), // LD B, D
            0x43 => Instruction::LD(TargetRegister8::B, TargetRegister8::E), // LD B, E
            0x44 => Instruction::LD(TargetRegister8::B, TargetRegister8::H), // LD B, H
            0x45 => Instruction::LD(TargetRegister8::B, TargetRegister8::L), // LD B, L
            0x46 => Instruction::UNIMPLEMENTED,                                                          // LD B, HL
            0x47 => Instruction::LD(TargetRegister8::B, TargetRegister8::A), // LD B, A
            0x48 => Instruction::LD(TargetRegister8::C, TargetRegister8::B), // LD C, B
            0x49 => Instruction::LD(TargetRegister8::C, TargetRegister8::C), // LD C, C
            0x4A => Instruction::LD(TargetRegister8::C, TargetRegister8::D), // LD C, D
            0x4B => Instruction::LD(TargetRegister8::C, TargetRegister8::E), // LD C, E
            0x4C => Instruction::LD(TargetRegister8::C, TargetRegister8::H), // LD C, H
            0x4D => Instruction::LD(TargetRegister8::C, TargetRegister8::L), // LD C, L
            0x4E => Instruction::UNIMPLEMENTED,                                                          // LD C, HL
            0x4F => Instruction::LD(TargetRegister8::C, TargetRegister8::A), // LD C, A

            0x50 => Instruction::LD(TargetRegister8::D, TargetRegister8::B), // LD D, B
            0x51 => Instruction::LD(TargetRegister8::D, TargetRegister8::C), // LD D, C
            0x52 => Instruction::LD(TargetRegister8::D, TargetRegister8::D), // LD D, D
            0x53 => Instruction::LD(TargetRegister8::D, TargetRegister8::E), // LD D, E
            0x54 => Instruction::LD(TargetRegister8::D, TargetRegister8::H), // LD D, H
            0x55 => Instruction::LD(TargetRegister8::D, TargetRegister8::L), // LD D, L
            0x56 => Instruction::UNIMPLEMENTED,                                                          // LD D, HL
            0x57 => Instruction::LD(TargetRegister8::D, TargetRegister8::A), // LD D, A
            0x58 => Instruction::LD(TargetRegister8::E, TargetRegister8::B), // LD E, B
            0x59 => Instruction::LD(TargetRegister8::E, TargetRegister8::C), // LD E, C
            0x5A => Instruction::LD(TargetRegister8::E, TargetRegister8::D), // LD E, D
            0x5B => Instruction::LD(TargetRegister8::E, TargetRegister8::E), // LD E, E
            0x5C => Instruction::LD(TargetRegister8::E, TargetRegister8::H), // LD E, H
            0x5D => Instruction::LD(TargetRegister8::E, TargetRegister8::L), // LD E, L
            0x5E => Instruction::UNIMPLEMENTED,                                                          // LD E, HL
            0x5F => Instruction::LD(TargetRegister8::E, TargetRegister8::A), // LD E, A

            0x60 => Instruction::LD(TargetRegister8::L, TargetRegister8::B), // LD L, B
            0x61 => Instruction::LD(TargetRegister8::L, TargetRegister8::C), // LD L, C
            0x62 => Instruction::LD(TargetRegister8::L, TargetRegister8::D), // LD L, D
            0x63 => Instruction::LD(TargetRegister8::L, TargetRegister8::E), // LD L, E
            0x64 => Instruction::LD(TargetRegister8::L, TargetRegister8::H), // LD L, H
            0x65 => Instruction::LD(TargetRegister8::L, TargetRegister8::L), // LD L, L
            0x66 => Instruction::UNIMPLEMENTED,                                                          // LD L, HL
            0x67 => Instruction::LD(TargetRegister8::L, TargetRegister8::A), // LD L, A
            0x68 => Instruction::LD(TargetRegister8::H, TargetRegister8::B), // LD H, B
            0x69 => Instruction::LD(TargetRegister8::H, TargetRegister8::C), // LD H, C
            0x6A => Instruction::LD(TargetRegister8::H, TargetRegister8::D), // LD H, D
            0x6B => Instruction::LD(TargetRegister8::H, TargetRegister8::E), // LD H, E
            0x6C => Instruction::LD(TargetRegister8::H, TargetRegister8::H), // LD H, H
            0x6D => Instruction::LD(TargetRegister8::H, TargetRegister8::L), // LD H, L
            0x6E => Instruction::UNIMPLEMENTED,                                                          // LD H, HL
            0x6F => Instruction::LD(TargetRegister8::H, TargetRegister8::A), // LD H, A

            0x70 => Instruction::UNIMPLEMENTED,
            0x71 => Instruction::UNIMPLEMENTED,
            0x72 => Instruction::UNIMPLEMENTED,
            0x73 => Instruction::UNIMPLEMENTED,
            0x74 => Instruction::UNIMPLEMENTED,
            0x75 => Instruction::UNIMPLEMENTED,
            0x76 => Instruction::HALT,
            0x77 => Instruction::UNIMPLEMENTED,
            0x78 => Instruction::UNIMPLEMENTED,
            0x79 => Instruction::UNIMPLEMENTED,
            0x7A => Instruction::UNIMPLEMENTED,
            0x7B => Instruction::UNIMPLEMENTED,
            0x7C => Instruction::UNIMPLEMENTED,
            0x7D => Instruction::UNIMPLEMENTED,
            0x7E => Instruction::UNIMPLEMENTED,
            0x7F => Instruction::LD(TargetRegister8::A, TargetRegister8::A), // LD A, A

            0x80 => Instruction::ADDR8(TargetRegister8::B), // ADD A, B
            0x81 => Instruction::ADDR8(TargetRegister8::C), // ADD A, C
            0x82 => Instruction::ADDR8(TargetRegister8::D), // ADD A, D
            0x83 => Instruction::ADDR8(TargetRegister8::E), // ADD A, E
            0x84 => Instruction::ADDR8(TargetRegister8::H), // ADD A, H
            0x85 => Instruction::ADDR8(TargetRegister8::L), // ADD A, L
            0x86 => Instruction::ADDHL,                     // ADD A, HL
            0x87 => Instruction::ADDR8(TargetRegister8::A), // ADD A, A
            0x88 => Instruction::ADCR8(TargetRegister8::B), // ADC A, B
            0x89 => Instruction::ADCR8(TargetRegister8::C), // ADC A, C
            0x8A => Instruction::ADCR8(TargetRegister8::D), // ADC A, D
            0x8B => Instruction::ADCR8(TargetRegister8::E), // ADC A, E
            0x8C => Instruction::ADCR8(TargetRegister8::H), // ADC A, H
            0x8D => Instruction::ADCR8(TargetRegister8::L), // ADC A, L
            0x8E => Instruction::ADCHL,                     // ADC A, HL
            0x8F => Instruction::ADCR8(TargetRegister8::A), // ADC A, A

            0x90 => Instruction::SUBR8(TargetRegister8::B), // SUB A, B
            0x91 => Instruction::SUBR8(TargetRegister8::C), // SUB A, C
            0x92 => Instruction::SUBR8(TargetRegister8::D), // SUB A, D
            0x93 => Instruction::SUBR8(TargetRegister8::E), // SUB A, E
            0x94 => Instruction::SUBR8(TargetRegister8::H), // SUB A, H
            0x95 => Instruction::SUBR8(TargetRegister8::L), // SUB A, L
            0x96 => Instruction::SUBHL,                     // SUB A, HL
            0x97 => Instruction::SUBR8(TargetRegister8::A), // SUB A, A
            0x98 => Instruction::SBCR8(TargetRegister8::B), // SBC A, B
            0x99 => Instruction::SBCR8(TargetRegister8::C), // SBC A, C
            0x9A => Instruction::SBCR8(TargetRegister8::D), // SBC A, D
            0x9B => Instruction::SBCR8(TargetRegister8::E), // SBC A, E
            0x9C => Instruction::SBCR8(TargetRegister8::H), // SBC A, H
            0x9D => Instruction::SBCR8(TargetRegister8::L), // SBC A, L
            0x9E => Instruction::SBCHL,                     // SBC A, HL
            0x9F => Instruction::SBCR8(TargetRegister8::A), // SBC A, A

            0xA0 => Instruction::ANDR8(TargetRegister8::B), // AND A, B
            0xA1 => Instruction::ANDR8(TargetRegister8::C), // AND A, C
            0xA2 => Instruction::ANDR8(TargetRegister8::D), // AND A, D
            0xA3 => Instruction::ANDR8(TargetRegister8::E), // AND A, E
            0xA4 => Instruction::ANDR8(TargetRegister8::H), // AND A, H
            0xA5 => Instruction::ANDR8(TargetRegister8::L), // AND A, L
            0xA6 => Instruction::ANDHL,                     // AND A, HL
            0xA7 => Instruction::ANDR8(TargetRegister8::A), // AND A, A
            0xA8 => Instruction::XORR8(TargetRegister8::B), // XOR A, B
            0xA9 => Instruction::XORR8(TargetRegister8::C), // XOR A, C
            0xAA => Instruction::XORR8(TargetRegister8::D), // XOR A, D
            0xAB => Instruction::XORR8(TargetRegister8::E), // XOR A, E
            0xAC => Instruction::XORR8(TargetRegister8::H), // XOR A, H
            0xAD => Instruction::XORR8(TargetRegister8::L), // XOR A, L
            0xAE => Instruction::XORHL,                     // XOR A, HL
            0xAF => Instruction::XORR8(TargetRegister8::A), // XOR A, A

            0xB0 => Instruction::ORR8(TargetRegister8::B), // OR A, B
            0xB1 => Instruction::ORR8(TargetRegister8::C), // OR A, C
            0xB2 => Instruction::ORR8(TargetRegister8::D), // OR A, D
            0xB3 => Instruction::ORR8(TargetRegister8::E), // OR A, E
            0xB4 => Instruction::ORR8(TargetRegister8::H), // OR A, H
            0xB5 => Instruction::ORR8(TargetRegister8::L), // OR A, L
            0xB6 => Instruction::ORHL,                     // OR A, HL
            0xB7 => Instruction::ORR8(TargetRegister8::A), // OR A, A
            0xB8 => Instruction::CPR8(TargetRegister8::B), // CP A, B
            0xB9 => Instruction::CPR8(TargetRegister8::C), // CP A, C
            0xBA => Instruction::CPR8(TargetRegister8::D), // CP A, D
            0xBB => Instruction::CPR8(TargetRegister8::E), // CP A, E
            0xBC => Instruction::CPR8(TargetRegister8::H), // CP A, H
            0xBD => Instruction::CPR8(TargetRegister8::L), // CP A, L
            0xBE => Instruction::CPHL,                     // CP A, HL
            0xBF => Instruction::CPR8(TargetRegister8::A), // CP A, A

            0xC0 => Instruction::RETF(Comparison::NONZERO), // RET NZ
            0xC1 => Instruction::POP(TargetRegister16::BC), // POP BC
            0xC2 => Instruction::JPF(Comparison::NONZERO),  // JP NZ, u16
            0xC3 => Instruction::JP,                        // JP u16
            0xC4 => Instruction::UNIMPLEMENTED,
            0xC5 => Instruction::PUSH(TargetRegister16::BC), // PUSH BC
            0xC6 => Instruction::ADDU8,                      // ADD A, u8
            0xC7 => Instruction::RST00H,                     // RST 00h
            0xC8 => Instruction::RETF(Comparison::ZERO),     // RET Z
            0xC9 => Instruction::RET,                        // RET
            0xCA => Instruction::JPF(Comparison::ZERO),      // JP Z, u16,
            0xCB => panic!("byte CB is the prefix byte and has no non prefix operation"),
            0xCC => Instruction::UNIMPLEMENTED,
            0xCD => Instruction::UNIMPLEMENTED,
            0xCE => Instruction::UNIMPLEMENTED,
            0xCF => Instruction::UNIMPLEMENTED,

            0xD0 => Instruction::UNIMPLEMENTED,
            0xD1 => Instruction::POP(TargetRegister16::DE),
            0xD2 => Instruction::UNIMPLEMENTED,
            0xD3 => panic!("byte {:X} has no documented op", byte),
            0xD4 => Instruction::UNIMPLEMENTED,
            0xD5 => Instruction::PUSH(TargetRegister16::DE),
            0xD6 => Instruction::SUBU8,
            0xD7 => Instruction::UNIMPLEMENTED,
            0xD8 => Instruction::UNIMPLEMENTED,
            0xD9 => Instruction::UNIMPLEMENTED,
            0xDA => Instruction::UNIMPLEMENTED,
            0xDB => panic!("byte {:X} has no documented op", byte),
            0xDC => Instruction::UNIMPLEMENTED,
            0xDD => panic!("byte {:X} has no documented op", byte),
            0xDE => Instruction::UNIMPLEMENTED,
            0xDF => Instruction::UNIMPLEMENTED,

            0xE0 => Instruction::LDAFF00U8A,
            0xE1 => Instruction::POP(TargetRegister16::HL),
            0xE2 => Instruction::UNIMPLEMENTED,
            0xE3 => panic!("byte {:X} has no documented op", byte),
            0xE4 => panic!("byte {:X} has no documented op", byte),
            0xE5 => Instruction::PUSH(TargetRegister16::HL),
            0xE6 => Instruction::ANDU8,
            0xE7 => Instruction::UNIMPLEMENTED,
            0xE8 => Instruction::UNIMPLEMENTED,
            0xE9 => Instruction::UNIMPLEMENTED,
            0xEA => Instruction::UNIMPLEMENTED,
            0xEB => panic!("byte {:X} has no documented op", byte),
            0xEC => panic!("byte {:X} has no documented op", byte),
            0xED => panic!("byte {:X} has no documented op", byte),
            0xEE => Instruction::UNIMPLEMENTED,
            0xEF => Instruction::UNIMPLEMENTED,

            0xF0 => Instruction::LDAFF00U8, // LD A, (FF00+u8)
            0xF1 => Instruction::POP(TargetRegister16::AF), // POP AF
            0xF2 => Instruction::LDAFF00C,  // LD A, (FF00+C)
            0xF3 => Instruction::DI,
            0xF4 => panic!("byte {:X} has no documented op", byte),
            0xF5 => Instruction::PUSH(TargetRegister16::AF), // PUSH AF
            0xF6 => Instruction::ORU8,                       // OR A, u8
            0xF7 => Instruction::RST30H,                     // RST 30h
            0xF8 => Instruction::UNIMPLEMENTED,                                          // LD HL, SP+i8
            0xF9 => Instruction::UNIMPLEMENTED,                                          // LD SP, HL
            0xFA => Instruction::LDAU16,                     // LD A, u16
            0xFB => Instruction::EI,
            0xFC => panic!("byte {:X} has no documented op", byte),
            0xFD => panic!("byte {:X} has no documented op", byte),
            0xFE => Instruction::CPU8, // CP A, u8
            0xFF => Instruction::UNIMPLEMENTED,                    // RST 38h
        }
    }
}
