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

    DEC(TargetIncDec),
    INC(TargetIncDec),

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
    RST(RstVector),

    //load
    LDR16U16(TargetRegister16),
    LDR8U8(TargetRegister8),
    LDR8HL(TargetRegister8),
    LDHLR8(TargetRegister8),
    LDHLU8,
    LDR8R8(TargetRegister8, TargetRegister8),

    LDAPTR(TargetPointer),

    LDAU16,
    LDU8(TargetRegister8),
    LDU16(TargetRegister16),
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
    LDFF00U8A,
    LDAFF00C,
    LDFF00CA,
    LDHLSPU8,
    LDSPHL,
    LDSP,
    POP(TargetPushPop),
    PUSH(TargetPushPop),

    //single bit ops
    BITHL(u8),
    RESHL(u8),
    SETHL(u8),
    BIT(u8, TargetRegister8),
    RES(u8, TargetRegister8),
    SET(u8, TargetRegister8),

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
pub enum RstVector {
    H00,
    H08,
    H10,
    H18,
    H20,
    H28,
    H30,
    H38,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Comparison {
    NONZERO,
    NOCARRY,
    ZERO,
    CARRY,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum TargetPointer {
    BC,
    DE,
    HL,
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
pub enum TargetPushPop {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum TargetIncDec {
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
    HLPOINTER,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Instruction {
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

            0x20 => Instruction::SLA(TargetRegister8::B),
            0x21 => Instruction::SLA(TargetRegister8::C),
            0x22 => Instruction::SLA(TargetRegister8::D),
            0x23 => Instruction::SLA(TargetRegister8::E),
            0x24 => Instruction::SLA(TargetRegister8::H),
            0x25 => Instruction::SLA(TargetRegister8::L),
            0x26 => Instruction::SLAHL,
            0x27 => Instruction::SLA(TargetRegister8::A),
            0x28 => Instruction::SRA(TargetRegister8::B),
            0x29 => Instruction::SRA(TargetRegister8::C),
            0x2A => Instruction::SRA(TargetRegister8::D),
            0x2B => Instruction::SRA(TargetRegister8::E),
            0x2C => Instruction::SRA(TargetRegister8::H),
            0x2D => Instruction::SRA(TargetRegister8::L),
            0x2E => Instruction::SRAHL,
            0x2F => Instruction::SRA(TargetRegister8::A),

            0x30 => Instruction::SWAP(TargetRegister8::B),
            0x31 => Instruction::SWAP(TargetRegister8::C),
            0x32 => Instruction::SWAP(TargetRegister8::D),
            0x33 => Instruction::SWAP(TargetRegister8::E),
            0x34 => Instruction::SWAP(TargetRegister8::H),
            0x35 => Instruction::SWAP(TargetRegister8::L),
            0x36 => Instruction::SWAPHL,
            0x37 => Instruction::SWAP(TargetRegister8::A),
            0x38 => Instruction::SRL(TargetRegister8::B),
            0x39 => Instruction::SRL(TargetRegister8::C),
            0x3A => Instruction::SRL(TargetRegister8::D),
            0x3B => Instruction::SRL(TargetRegister8::E),
            0x3C => Instruction::SRL(TargetRegister8::H),
            0x3D => Instruction::SRL(TargetRegister8::L),
            0x3E => Instruction::SRLHL,
            0x3F => Instruction::SRL(TargetRegister8::A),

            0x40 => Instruction::BIT(0x00, TargetRegister8::B),
            0x41 => Instruction::BIT(0x00, TargetRegister8::C),
            0x42 => Instruction::BIT(0x00, TargetRegister8::D),
            0x43 => Instruction::BIT(0x00, TargetRegister8::E),
            0x44 => Instruction::BIT(0x00, TargetRegister8::H),
            0x45 => Instruction::BIT(0x00, TargetRegister8::L),
            0x46 => Instruction::BITHL(0x00),
            0x47 => Instruction::BIT(0x00, TargetRegister8::A),
            0x48 => Instruction::BIT(0x01, TargetRegister8::B),
            0x49 => Instruction::BIT(0x01, TargetRegister8::C),
            0x4A => Instruction::BIT(0x01, TargetRegister8::D),
            0x4B => Instruction::BIT(0x01, TargetRegister8::E),
            0x4C => Instruction::BIT(0x01, TargetRegister8::H),
            0x4D => Instruction::BIT(0x01, TargetRegister8::L),
            0x4E => Instruction::BITHL(0x01),
            0x4F => Instruction::BIT(0x01, TargetRegister8::A),

            0x50 => Instruction::BIT(0x02, TargetRegister8::B),
            0x51 => Instruction::BIT(0x02, TargetRegister8::C),
            0x52 => Instruction::BIT(0x02, TargetRegister8::D),
            0x53 => Instruction::BIT(0x02, TargetRegister8::E),
            0x54 => Instruction::BIT(0x02, TargetRegister8::H),
            0x55 => Instruction::BIT(0x02, TargetRegister8::L),
            0x56 => Instruction::BITHL(0x02),
            0x57 => Instruction::BIT(0x02, TargetRegister8::A),
            0x58 => Instruction::BIT(0x03, TargetRegister8::B),
            0x59 => Instruction::BIT(0x03, TargetRegister8::C),
            0x5A => Instruction::BIT(0x03, TargetRegister8::D),
            0x5B => Instruction::BIT(0x03, TargetRegister8::E),
            0x5C => Instruction::BIT(0x03, TargetRegister8::H),
            0x5D => Instruction::BIT(0x03, TargetRegister8::L),
            0x5E => Instruction::BITHL(0x03),
            0x5F => Instruction::BIT(0x03, TargetRegister8::A),

            0x60 => Instruction::BIT(0x04, TargetRegister8::B),
            0x61 => Instruction::BIT(0x04, TargetRegister8::C),
            0x62 => Instruction::BIT(0x04, TargetRegister8::D),
            0x63 => Instruction::BIT(0x04, TargetRegister8::E),
            0x64 => Instruction::BIT(0x04, TargetRegister8::H),
            0x65 => Instruction::BIT(0x04, TargetRegister8::L),
            0x66 => Instruction::BITHL(0x04),
            0x67 => Instruction::BIT(0x04, TargetRegister8::A),
            0x68 => Instruction::BIT(0x05, TargetRegister8::B),
            0x69 => Instruction::BIT(0x05, TargetRegister8::C),
            0x6A => Instruction::BIT(0x05, TargetRegister8::D),
            0x6B => Instruction::BIT(0x05, TargetRegister8::E),
            0x6C => Instruction::BIT(0x05, TargetRegister8::H),
            0x6D => Instruction::BIT(0x05, TargetRegister8::L),
            0x6E => Instruction::BITHL(0x05),
            0x6F => Instruction::BIT(0x05, TargetRegister8::A),

            0x70 => Instruction::BIT(0x06, TargetRegister8::B),
            0x71 => Instruction::BIT(0x06, TargetRegister8::C),
            0x72 => Instruction::BIT(0x06, TargetRegister8::D),
            0x73 => Instruction::BIT(0x06, TargetRegister8::E),
            0x74 => Instruction::BIT(0x06, TargetRegister8::H),
            0x75 => Instruction::BIT(0x06, TargetRegister8::L),
            0x76 => Instruction::BITHL(0x06),
            0x77 => Instruction::BIT(0x06, TargetRegister8::A),
            0x78 => Instruction::BIT(0x07, TargetRegister8::B),
            0x79 => Instruction::BIT(0x07, TargetRegister8::C),
            0x7A => Instruction::BIT(0x07, TargetRegister8::D),
            0x7B => Instruction::BIT(0x07, TargetRegister8::E),
            0x7C => Instruction::BIT(0x07, TargetRegister8::H),
            0x7D => Instruction::BIT(0x07, TargetRegister8::L),
            0x7E => Instruction::BITHL(0x07),
            0x7F => Instruction::BIT(0x07, TargetRegister8::A),

            0x80 => Instruction::RES(0x00, TargetRegister8::B),
            0x81 => Instruction::RES(0x00, TargetRegister8::C),
            0x82 => Instruction::RES(0x00, TargetRegister8::D),
            0x83 => Instruction::RES(0x00, TargetRegister8::E),
            0x84 => Instruction::RES(0x00, TargetRegister8::H),
            0x85 => Instruction::RES(0x00, TargetRegister8::L),
            0x86 => Instruction::RESHL(0x00),
            0x87 => Instruction::RES(0x00, TargetRegister8::A),
            0x88 => Instruction::RES(0x01, TargetRegister8::B),
            0x89 => Instruction::RES(0x01, TargetRegister8::C),
            0x8A => Instruction::RES(0x01, TargetRegister8::D),
            0x8B => Instruction::RES(0x01, TargetRegister8::E),
            0x8C => Instruction::RES(0x01, TargetRegister8::H),
            0x8D => Instruction::RES(0x01, TargetRegister8::L),
            0x8E => Instruction::RESHL(0x01),
            0x8F => Instruction::RES(0x01, TargetRegister8::A),

            0x90 => Instruction::RES(0x02, TargetRegister8::B),
            0x91 => Instruction::RES(0x02, TargetRegister8::C),
            0x92 => Instruction::RES(0x02, TargetRegister8::D),
            0x93 => Instruction::RES(0x02, TargetRegister8::E),
            0x94 => Instruction::RES(0x02, TargetRegister8::H),
            0x95 => Instruction::RES(0x02, TargetRegister8::L),
            0x96 => Instruction::RESHL(0x02),
            0x97 => Instruction::RES(0x02, TargetRegister8::A),
            0x98 => Instruction::RES(0x03, TargetRegister8::B),
            0x99 => Instruction::RES(0x03, TargetRegister8::C),
            0x9A => Instruction::RES(0x03, TargetRegister8::D),
            0x9B => Instruction::RES(0x03, TargetRegister8::E),
            0x9C => Instruction::RES(0x03, TargetRegister8::H),
            0x9D => Instruction::RES(0x03, TargetRegister8::L),
            0x9E => Instruction::RESHL(0x03),
            0x9F => Instruction::RES(0x03, TargetRegister8::A),

            0xA0 => Instruction::RES(0x04, TargetRegister8::B),
            0xA1 => Instruction::RES(0x04, TargetRegister8::C),
            0xA2 => Instruction::RES(0x04, TargetRegister8::D),
            0xA3 => Instruction::RES(0x04, TargetRegister8::E),
            0xA4 => Instruction::RES(0x04, TargetRegister8::H),
            0xA5 => Instruction::RES(0x04, TargetRegister8::L),
            0xA6 => Instruction::RESHL(0x04),
            0xA7 => Instruction::RES(0x04, TargetRegister8::A),
            0xA8 => Instruction::RES(0x05, TargetRegister8::B),
            0xA9 => Instruction::RES(0x05, TargetRegister8::C),
            0xAA => Instruction::RES(0x05, TargetRegister8::D),
            0xAB => Instruction::RES(0x05, TargetRegister8::E),
            0xAC => Instruction::RES(0x05, TargetRegister8::H),
            0xAD => Instruction::RES(0x05, TargetRegister8::L),
            0xAE => Instruction::RESHL(0x05),
            0xAF => Instruction::RES(0x05, TargetRegister8::A),

            0xB0 => Instruction::RES(0x06, TargetRegister8::B),
            0xB1 => Instruction::RES(0x06, TargetRegister8::C),
            0xB2 => Instruction::RES(0x06, TargetRegister8::D),
            0xB3 => Instruction::RES(0x06, TargetRegister8::E),
            0xB4 => Instruction::RES(0x06, TargetRegister8::H),
            0xB5 => Instruction::RES(0x06, TargetRegister8::L),
            0xB6 => Instruction::RESHL(0x06),
            0xB7 => Instruction::RES(0x06, TargetRegister8::A),
            0xB8 => Instruction::RES(0x07, TargetRegister8::B),
            0xB9 => Instruction::RES(0x07, TargetRegister8::C),
            0xBA => Instruction::RES(0x07, TargetRegister8::D),
            0xBB => Instruction::RES(0x07, TargetRegister8::E),
            0xBC => Instruction::RES(0x07, TargetRegister8::H),
            0xBD => Instruction::RES(0x07, TargetRegister8::L),
            0xBE => Instruction::RESHL(0x07),
            0xBF => Instruction::RES(0x07, TargetRegister8::A),

            0xC0 => Instruction::SET(0x00, TargetRegister8::B),
            0xC1 => Instruction::SET(0x00, TargetRegister8::C),
            0xC2 => Instruction::SET(0x00, TargetRegister8::D),
            0xC3 => Instruction::SET(0x00, TargetRegister8::E),
            0xC4 => Instruction::SET(0x00, TargetRegister8::H),
            0xC5 => Instruction::SET(0x00, TargetRegister8::L),
            0xC6 => Instruction::SETHL(0x00),
            0xC7 => Instruction::SET(0x00, TargetRegister8::A),
            0xC8 => Instruction::SET(0x01, TargetRegister8::B),
            0xC9 => Instruction::SET(0x01, TargetRegister8::C),
            0xCA => Instruction::SET(0x01, TargetRegister8::D),
            0xCB => Instruction::SET(0x01, TargetRegister8::E),
            0xCC => Instruction::SET(0x01, TargetRegister8::H),
            0xCD => Instruction::SET(0x01, TargetRegister8::L),
            0xCE => Instruction::SETHL(0x01),
            0xCF => Instruction::SET(0x01, TargetRegister8::A),

            0xD0 => Instruction::SET(0x02, TargetRegister8::B),
            0xD1 => Instruction::SET(0x02, TargetRegister8::C),
            0xD2 => Instruction::SET(0x02, TargetRegister8::D),
            0xD3 => Instruction::SET(0x02, TargetRegister8::E),
            0xD4 => Instruction::SET(0x02, TargetRegister8::H),
            0xD5 => Instruction::SET(0x02, TargetRegister8::L),
            0xD6 => Instruction::SETHL(0x02),
            0xD7 => Instruction::SET(0x02, TargetRegister8::A),
            0xD8 => Instruction::SET(0x03, TargetRegister8::B),
            0xD9 => Instruction::SET(0x03, TargetRegister8::C),
            0xDA => Instruction::SET(0x03, TargetRegister8::D),
            0xDB => Instruction::SET(0x03, TargetRegister8::E),
            0xDC => Instruction::SET(0x03, TargetRegister8::H),
            0xDD => Instruction::SET(0x03, TargetRegister8::L),
            0xDE => Instruction::SETHL(0x03),
            0xDF => Instruction::SET(0x03, TargetRegister8::A),

            0xE0 => Instruction::SET(0x04, TargetRegister8::B),
            0xE1 => Instruction::SET(0x04, TargetRegister8::C),
            0xE2 => Instruction::SET(0x04, TargetRegister8::D),
            0xE3 => Instruction::SET(0x04, TargetRegister8::E),
            0xE4 => Instruction::SET(0x04, TargetRegister8::H),
            0xE5 => Instruction::SET(0x04, TargetRegister8::L),
            0xE6 => Instruction::SETHL(0x04),
            0xE7 => Instruction::SET(0x04, TargetRegister8::A),
            0xE8 => Instruction::SET(0x05, TargetRegister8::B),
            0xE9 => Instruction::SET(0x05, TargetRegister8::C),
            0xEA => Instruction::SET(0x05, TargetRegister8::D),
            0xEB => Instruction::SET(0x05, TargetRegister8::E),
            0xEC => Instruction::SET(0x05, TargetRegister8::H),
            0xED => Instruction::SET(0x05, TargetRegister8::L),
            0xEE => Instruction::SETHL(0x05),
            0xEF => Instruction::SET(0x05, TargetRegister8::A),

            0xF0 => Instruction::SET(0x06, TargetRegister8::B),
            0xF1 => Instruction::SET(0x06, TargetRegister8::C),
            0xF2 => Instruction::SET(0x06, TargetRegister8::D),
            0xF3 => Instruction::SET(0x06, TargetRegister8::E),
            0xF4 => Instruction::SET(0x06, TargetRegister8::H),
            0xF5 => Instruction::SET(0x06, TargetRegister8::L),
            0xF6 => Instruction::SETHL(0x06),
            0xF7 => Instruction::SET(0x06, TargetRegister8::A),
            0xF8 => Instruction::SET(0x07, TargetRegister8::B),
            0xF9 => Instruction::SET(0x07, TargetRegister8::C),
            0xFA => Instruction::SET(0x07, TargetRegister8::D),
            0xFB => Instruction::SET(0x07, TargetRegister8::E),
            0xFC => Instruction::SET(0x07, TargetRegister8::H),
            0xFD => Instruction::SET(0x07, TargetRegister8::L),
            0xFE => Instruction::SETHL(0x07),
            0xFF => Instruction::SET(0x07, TargetRegister8::A),
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::NOP,                          // NOP
            0x01 => Instruction::LDU16(TargetRegister16::BC),  // LD BC, u16
            0x02 => Instruction::LDBCA,                        // LD (BC), A
            0x03 => Instruction::INC(TargetIncDec::BC),        // INC BC
            0x04 => Instruction::INC(TargetIncDec::B),         // INC B
            0x05 => Instruction::DEC(TargetIncDec::B),         // DEC B
            0x06 => Instruction::LDU8(TargetRegister8::B),     // LD B, u8
            0x07 => Instruction::RLCA,                         // RLCA
            0x08 => Instruction::LDSP,                         // LD (u16), SP
            0x09 => Instruction::ADDR16(TargetRegister16::BC), // ADD HL, BC
            0x0A => Instruction::LDAPTR(TargetPointer::BC),    // LD A, (BC)
            0x0B => Instruction::DEC(TargetIncDec::BC),        // DEC BC
            0x0C => Instruction::INC(TargetIncDec::C),         // INC C
            0x0D => Instruction::DEC(TargetIncDec::C),         // DEC C
            0x0E => Instruction::LDU8(TargetRegister8::C),     // LD C, u8
            0x0F => Instruction::RRCA,                         // RRCA

            0x10 => Instruction::STOP,                         // STOP
            0x11 => Instruction::LDU16(TargetRegister16::DE),  // LD DE, u16
            0x12 => Instruction::LDDEA,                        // LD (DE), A
            0x13 => Instruction::INC(TargetIncDec::DE),        // INC DE
            0x14 => Instruction::INC(TargetIncDec::D),         // INC D
            0x15 => Instruction::DEC(TargetIncDec::D),         // DEC D
            0x16 => Instruction::LDU8(TargetRegister8::D),     // LD D, u8
            0x17 => Instruction::RLA,                          // RLA
            0x18 => Instruction::JR,                           // JR i8
            0x19 => Instruction::ADDR16(TargetRegister16::DE), // ADD HL, DE
            0x1A => Instruction::LDAPTR(TargetPointer::DE),    // LD A, (DE)
            0x1B => Instruction::DEC(TargetIncDec::DE),        // DEC DE
            0x1C => Instruction::INC(TargetIncDec::E),         // INC E
            0x1D => Instruction::DEC(TargetIncDec::E),         // DEC E
            0x1E => Instruction::LDU8(TargetRegister8::E),     // LD E, u8
            0x1F => Instruction::RRA,                          // RRA

            0x20 => Instruction::JRF(Comparison::NONZERO), // JR NZ, i8
            0x21 => Instruction::LDU16(TargetRegister16::HL), // LD HL, u16
            0x22 => Instruction::LDIHLA,
            0x23 => Instruction::INC(TargetIncDec::HL), // INC HL
            0x24 => Instruction::INC(TargetIncDec::H),  // INC H
            0x25 => Instruction::DEC(TargetIncDec::H),  // DEC H
            0x26 => Instruction::UNIMPLEMENTED,
            0x27 => Instruction::DAA,                          // DDA
            0x28 => Instruction::JRF(Comparison::ZERO),        // JR Z, i8
            0x29 => Instruction::ADDR16(TargetRegister16::HL), // LD HL, HL
            0x2A => Instruction::UNIMPLEMENTED,
            0x2B => Instruction::DEC(TargetIncDec::HL), // DEC HL
            0x2C => Instruction::INC(TargetIncDec::L),  // INC L
            0x2D => Instruction::DEC(TargetIncDec::L),  // DEC L
            0x2E => Instruction::LDR8U8(TargetRegister8::L), // LD L, u8
            0x2F => Instruction::CPL,

            0x30 => Instruction::JRF(Comparison::NOCARRY), // JR NC, i8
            0x31 => Instruction::LDU16(TargetRegister16::SP), // LD SP, u16
            0x32 => Instruction::LDDHLA,
            0x33 => Instruction::INC(TargetIncDec::SP), // INC SP
            0x34 => Instruction::INC(TargetIncDec::HLPOINTER), // INC (HL)
            0x35 => Instruction::DEC(TargetIncDec::HLPOINTER), // DEC (HL)
            0x36 => Instruction::UNIMPLEMENTED,
            0x37 => Instruction::SCF,
            0x38 => Instruction::JRF(Comparison::CARRY), // JR C, i8
            0x39 => Instruction::UNIMPLEMENTED,
            0x3A => Instruction::UNIMPLEMENTED,
            0x3B => Instruction::DEC(TargetIncDec::SP), // DEC SP
            0x3C => Instruction::INC(TargetIncDec::A),  // INC A
            0x3D => Instruction::DEC(TargetIncDec::A),  // DEC A
            0x3E => Instruction::LDU8(TargetRegister8::A), // LD A, u8
            0x3F => Instruction::CCF,

            0x40 => Instruction::LDR8R8(TargetRegister8::B, TargetRegister8::B), // LD B, B
            0x41 => Instruction::LDR8R8(TargetRegister8::B, TargetRegister8::C), // LD B, C
            0x42 => Instruction::LDR8R8(TargetRegister8::B, TargetRegister8::D), // LD B, D
            0x43 => Instruction::LDR8R8(TargetRegister8::B, TargetRegister8::E), // LD B, E
            0x44 => Instruction::LDR8R8(TargetRegister8::B, TargetRegister8::H), // LD B, H
            0x45 => Instruction::LDR8R8(TargetRegister8::B, TargetRegister8::L), // LD B, L
            0x46 => Instruction::UNIMPLEMENTED,                                  // LD B, (HL)
            0x47 => Instruction::LDR8R8(TargetRegister8::B, TargetRegister8::A), // LD B, A
            0x48 => Instruction::LDR8R8(TargetRegister8::C, TargetRegister8::B), // LD C, B
            0x49 => Instruction::LDR8R8(TargetRegister8::C, TargetRegister8::C), // LD C, C
            0x4A => Instruction::LDR8R8(TargetRegister8::C, TargetRegister8::D), // LD C, D
            0x4B => Instruction::LDR8R8(TargetRegister8::C, TargetRegister8::E), // LD C, E
            0x4C => Instruction::LDR8R8(TargetRegister8::C, TargetRegister8::H), // LD C, H
            0x4D => Instruction::LDR8R8(TargetRegister8::C, TargetRegister8::L), // LD C, L
            0x4E => Instruction::UNIMPLEMENTED,                                  // LD C, (HL)
            0x4F => Instruction::LDR8R8(TargetRegister8::C, TargetRegister8::A), // LD C, A

            0x50 => Instruction::LDR8R8(TargetRegister8::D, TargetRegister8::B), // LD D, B
            0x51 => Instruction::LDR8R8(TargetRegister8::D, TargetRegister8::C), // LD D, C
            0x52 => Instruction::LDR8R8(TargetRegister8::D, TargetRegister8::D), // LD D, D
            0x53 => Instruction::LDR8R8(TargetRegister8::D, TargetRegister8::E), // LD D, E
            0x54 => Instruction::LDR8R8(TargetRegister8::D, TargetRegister8::H), // LD D, H
            0x55 => Instruction::LDR8R8(TargetRegister8::D, TargetRegister8::L), // LD D, L
            0x56 => Instruction::UNIMPLEMENTED,                                  // LD D, (HL)
            0x57 => Instruction::LDR8R8(TargetRegister8::D, TargetRegister8::A), // LD D, A
            0x58 => Instruction::LDR8R8(TargetRegister8::E, TargetRegister8::B), // LD E, B
            0x59 => Instruction::LDR8R8(TargetRegister8::E, TargetRegister8::C), // LD E, C
            0x5A => Instruction::LDR8R8(TargetRegister8::E, TargetRegister8::D), // LD E, D
            0x5B => Instruction::LDR8R8(TargetRegister8::E, TargetRegister8::E), // LD E, E
            0x5C => Instruction::LDR8R8(TargetRegister8::E, TargetRegister8::H), // LD E, H
            0x5D => Instruction::LDR8R8(TargetRegister8::E, TargetRegister8::L), // LD E, L
            0x5E => Instruction::UNIMPLEMENTED,                                  // LD E, (HL)
            0x5F => Instruction::LDR8R8(TargetRegister8::E, TargetRegister8::A), // LD E, A

            0x60 => Instruction::LDR8R8(TargetRegister8::H, TargetRegister8::B), // LD H, B
            0x61 => Instruction::LDR8R8(TargetRegister8::H, TargetRegister8::C), // LD H, C
            0x62 => Instruction::LDR8R8(TargetRegister8::H, TargetRegister8::D), // LD H, D
            0x63 => Instruction::LDR8R8(TargetRegister8::H, TargetRegister8::E), // LD H, E
            0x64 => Instruction::LDR8R8(TargetRegister8::H, TargetRegister8::H), // LD H, H
            0x65 => Instruction::LDR8R8(TargetRegister8::H, TargetRegister8::L), // LD H, L
            0x66 => Instruction::UNIMPLEMENTED,                                  // LD H, (HL)
            0x67 => Instruction::LDR8R8(TargetRegister8::H, TargetRegister8::A), // LD H, A
            0x68 => Instruction::LDR8R8(TargetRegister8::L, TargetRegister8::B), // LD L, B
            0x69 => Instruction::LDR8R8(TargetRegister8::L, TargetRegister8::C), // LD L, C
            0x6A => Instruction::LDR8R8(TargetRegister8::L, TargetRegister8::D), // LD L, D
            0x6B => Instruction::LDR8R8(TargetRegister8::L, TargetRegister8::E), // LD L, E
            0x6C => Instruction::LDR8R8(TargetRegister8::L, TargetRegister8::H), // LD L, H
            0x6D => Instruction::LDR8R8(TargetRegister8::L, TargetRegister8::L), // LD L, L
            0x6E => Instruction::UNIMPLEMENTED,                                  // LD L, (HL)
            0x6F => Instruction::LDR8R8(TargetRegister8::L, TargetRegister8::A), // LD L, A

            0x70 => Instruction::LDHLR8(TargetRegister8::B),
            0x71 => Instruction::LDHLR8(TargetRegister8::C),
            0x72 => Instruction::LDHLR8(TargetRegister8::D),
            0x73 => Instruction::LDHLR8(TargetRegister8::E),
            0x74 => Instruction::LDHLR8(TargetRegister8::H),
            0x75 => Instruction::LDHLR8(TargetRegister8::L),
            0x76 => Instruction::HALT, // HALT
            0x77 => Instruction::LDHLR8(TargetRegister8::A),
            0x78 => Instruction::LDR8R8(TargetRegister8::A, TargetRegister8::B), // LD A, B
            0x79 => Instruction::LDR8R8(TargetRegister8::A, TargetRegister8::C), // LD A, C
            0x7A => Instruction::LDR8R8(TargetRegister8::A, TargetRegister8::D), // LD A, D
            0x7B => Instruction::LDR8R8(TargetRegister8::A, TargetRegister8::E), // LD A, E
            0x7C => Instruction::LDR8R8(TargetRegister8::A, TargetRegister8::H), // LD A, H
            0x7D => Instruction::LDR8R8(TargetRegister8::A, TargetRegister8::L), // LD A, L
            0x7E => Instruction::UNIMPLEMENTED,
            0x7F => Instruction::LDR8R8(TargetRegister8::A, TargetRegister8::A), // LD A, A

            0x80 => Instruction::ADDR8(TargetRegister8::B), // ADD A, B
            0x81 => Instruction::ADDR8(TargetRegister8::C), // ADD A, C
            0x82 => Instruction::ADDR8(TargetRegister8::D), // ADD A, D
            0x83 => Instruction::ADDR8(TargetRegister8::E), // ADD A, E
            0x84 => Instruction::ADDR8(TargetRegister8::H), // ADD A, H
            0x85 => Instruction::ADDR8(TargetRegister8::L), // ADD A, L
            0x86 => Instruction::ADDHL,                     // ADD A, (HL)
            0x87 => Instruction::ADDR8(TargetRegister8::A), // ADD A, A
            0x88 => Instruction::ADCR8(TargetRegister8::B), // ADC A, B
            0x89 => Instruction::ADCR8(TargetRegister8::C), // ADC A, C
            0x8A => Instruction::ADCR8(TargetRegister8::D), // ADC A, D
            0x8B => Instruction::ADCR8(TargetRegister8::E), // ADC A, E
            0x8C => Instruction::ADCR8(TargetRegister8::H), // ADC A, H
            0x8D => Instruction::ADCR8(TargetRegister8::L), // ADC A, L
            0x8E => Instruction::ADCHL,                     // ADC A, (HL)
            0x8F => Instruction::ADCR8(TargetRegister8::A), // ADC A, A

            0x90 => Instruction::SUBR8(TargetRegister8::B), // SUB A, B
            0x91 => Instruction::SUBR8(TargetRegister8::C), // SUB A, C
            0x92 => Instruction::SUBR8(TargetRegister8::D), // SUB A, D
            0x93 => Instruction::SUBR8(TargetRegister8::E), // SUB A, E
            0x94 => Instruction::SUBR8(TargetRegister8::H), // SUB A, H
            0x95 => Instruction::SUBR8(TargetRegister8::L), // SUB A, L
            0x96 => Instruction::SUBHL,                     // SUB A, (HL)
            0x97 => Instruction::SUBR8(TargetRegister8::A), // SUB A, A
            0x98 => Instruction::SBCR8(TargetRegister8::B), // SBC A, B
            0x99 => Instruction::SBCR8(TargetRegister8::C), // SBC A, C
            0x9A => Instruction::SBCR8(TargetRegister8::D), // SBC A, D
            0x9B => Instruction::SBCR8(TargetRegister8::E), // SBC A, E
            0x9C => Instruction::SBCR8(TargetRegister8::H), // SBC A, H
            0x9D => Instruction::SBCR8(TargetRegister8::L), // SBC A, L
            0x9E => Instruction::SBCHL,                     // SBC A, (HL)
            0x9F => Instruction::SBCR8(TargetRegister8::A), // SBC A, A

            0xA0 => Instruction::ANDR8(TargetRegister8::B), // AND A, B
            0xA1 => Instruction::ANDR8(TargetRegister8::C), // AND A, C
            0xA2 => Instruction::ANDR8(TargetRegister8::D), // AND A, D
            0xA3 => Instruction::ANDR8(TargetRegister8::E), // AND A, E
            0xA4 => Instruction::ANDR8(TargetRegister8::H), // AND A, H
            0xA5 => Instruction::ANDR8(TargetRegister8::L), // AND A, L
            0xA6 => Instruction::ANDHL,                     // AND A, (HL)
            0xA7 => Instruction::ANDR8(TargetRegister8::A), // AND A, A
            0xA8 => Instruction::XORR8(TargetRegister8::B), // XOR A, B
            0xA9 => Instruction::XORR8(TargetRegister8::C), // XOR A, C
            0xAA => Instruction::XORR8(TargetRegister8::D), // XOR A, D
            0xAB => Instruction::XORR8(TargetRegister8::E), // XOR A, E
            0xAC => Instruction::XORR8(TargetRegister8::H), // XOR A, H
            0xAD => Instruction::XORR8(TargetRegister8::L), // XOR A, L
            0xAE => Instruction::XORHL,                     // XOR A, (HL)
            0xAF => Instruction::XORR8(TargetRegister8::A), // XOR A, A

            0xB0 => Instruction::ORR8(TargetRegister8::B), // OR A, B
            0xB1 => Instruction::ORR8(TargetRegister8::C), // OR A, C
            0xB2 => Instruction::ORR8(TargetRegister8::D), // OR A, D
            0xB3 => Instruction::ORR8(TargetRegister8::E), // OR A, E
            0xB4 => Instruction::ORR8(TargetRegister8::H), // OR A, H
            0xB5 => Instruction::ORR8(TargetRegister8::L), // OR A, L
            0xB6 => Instruction::ORHL,                     // OR A, (HL)
            0xB7 => Instruction::ORR8(TargetRegister8::A), // OR A, A
            0xB8 => Instruction::CPR8(TargetRegister8::B), // CP A, B
            0xB9 => Instruction::CPR8(TargetRegister8::C), // CP A, C
            0xBA => Instruction::CPR8(TargetRegister8::D), // CP A, D
            0xBB => Instruction::CPR8(TargetRegister8::E), // CP A, E
            0xBC => Instruction::CPR8(TargetRegister8::H), // CP A, H
            0xBD => Instruction::CPR8(TargetRegister8::L), // CP A, L
            0xBE => Instruction::CPHL,                     // CP A, (HL)
            0xBF => Instruction::CPR8(TargetRegister8::A), // CP A, A

            0xC0 => Instruction::RETF(Comparison::NONZERO), // RET NZ
            0xC1 => Instruction::POP(TargetPushPop::BC),    // POP BC
            0xC2 => Instruction::JPF(Comparison::NONZERO),  // JP NZ, u16
            0xC3 => Instruction::JP,                        // JP u16
            0xC4 => Instruction::CALLF(Comparison::NOCARRY), // CALL NC, u16
            0xC5 => Instruction::PUSH(TargetPushPop::BC),   // PUSH BC
            0xC6 => Instruction::ADDU8,                     // ADD A, u8
            0xC7 => Instruction::RST(RstVector::H00),       // RST 0x00
            0xC8 => Instruction::RETF(Comparison::ZERO),    // RET Z
            0xC9 => Instruction::RET,                       // RET
            0xCA => Instruction::JPF(Comparison::ZERO),     // JP Z, u16,
            0xCB => panic!("byte CB is the prefix byte and has no non prefix operation"), // PREFIX BYTE
            0xCC => Instruction::CALLF(Comparison::ZERO), // CALL Z, u16
            0xCD => Instruction::CALL,                    // CALL u16
            0xCE => Instruction::ADCU8,                   // ADC A, u8
            0xCF => Instruction::RST(RstVector::H08),     // RST 0x08

            0xD0 => Instruction::RETF(Comparison::NOCARRY), // RET NC
            0xD1 => Instruction::POP(TargetPushPop::DE),    // POP DE
            0xD2 => Instruction::JPF(Comparison::NOCARRY),  // JP NC, u16
            0xD3 => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xD4 => Instruction::CALLF(Comparison::NOCARRY), // CALL NC, u16
            0xD5 => Instruction::PUSH(TargetPushPop::DE),   // PUSH DE
            0xD6 => Instruction::SUBU8,                     // SUB A, u8
            0xD7 => Instruction::RST(RstVector::H10),       // RST 0x10
            0xD8 => Instruction::RETF(Comparison::CARRY),   // RET C
            0xD9 => Instruction::RETI,                      // RETI
            0xDA => Instruction::JPF(Comparison::CARRY),    // JP C, u16
            0xDB => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xDC => Instruction::CALLF(Comparison::CARRY),  // CALL C, u16
            0xDD => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xDE => Instruction::SBCU8,                     // SBC A, u8
            0xDF => Instruction::RST(RstVector::H18),       // RST 0x18

            0xE0 => Instruction::LDFF00U8A, // LD (FF00+u8), A
            0xE1 => Instruction::POP(TargetPushPop::HL), // POP HL
            0xE2 => Instruction::LDFF00CA,  // LD (FF00 + C), A
            0xE3 => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xE4 => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xE5 => Instruction::PUSH(TargetPushPop::HL), // PUSH HL
            0xE6 => Instruction::ANDU8,     // AND A, u8
            0xE7 => Instruction::RST(RstVector::H20), // RST 0x20
            0xE8 => Instruction::ADDSP,     // ADD SP, i8
            0xE9 => Instruction::JPHL,      // JP HL
            0xEA => Instruction::LDU16A,    // LD u16, A
            0xEB => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xEC => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xED => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xEE => Instruction::XORU8,     // XOR A, u8
            0xEF => Instruction::RST(RstVector::H28), // RST 0x28,

            0xF0 => Instruction::LDAFF00U8, // LD A, (FF00+u8)
            0xF1 => Instruction::POP(TargetPushPop::AF), // POP AF
            0xF2 => Instruction::LDAFF00C,  // LD A, (FF00+C)
            0xF3 => Instruction::DI,        // DI
            0xF4 => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xF5 => Instruction::PUSH(TargetPushPop::AF), // PUSH AF
            0xF6 => Instruction::ORU8,      // OR A, u8
            0xF7 => Instruction::RST(RstVector::H30), // RST 0x30
            0xF8 => Instruction::LDHLSPU8,  // LD HL, SP+i8
            0xF9 => Instruction::LDSPHL,    // LD SP, HL
            0xFA => Instruction::LDAU16,    // LD A, u16
            0xFB => Instruction::EI,        // EI
            0xFC => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xFD => panic!("byte {:X} has no assigned op", byte), // unassigned
            0xFE => Instruction::CPU8,      // CP A, u8
            0xFF => Instruction::RST(RstVector::H38), // RST 0x38
        }
    }
}
