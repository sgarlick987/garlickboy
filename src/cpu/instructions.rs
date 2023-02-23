pub const INSTRUCTION_PREFIX_BYTE: u8 = 0xCB;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum Comparison {
    NONZERO,
    NOCARRY,
    ZERO,
    CARRY,
}

#[derive(Debug)]
pub enum TargetRegister8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum TargetRegister16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
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
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::frombyteprefixed(byte)
        } else {
            Instruction::frombytenotprefixed(byte)
        }
    }

    pub fn frombyteprefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(TargetRegister8::B)),
            0x01 => Some(Instruction::RLC(TargetRegister8::C)),
            0x02 => Some(Instruction::RLC(TargetRegister8::D)),
            0x03 => Some(Instruction::RLC(TargetRegister8::E)),
            0x04 => Some(Instruction::RLC(TargetRegister8::H)),
            0x05 => Some(Instruction::RLC(TargetRegister8::L)),
            0x06 => Some(Instruction::RLCHL),
            0x07 => Some(Instruction::RLC(TargetRegister8::A)),
            0x08 => Some(Instruction::RRC(TargetRegister8::B)),
            0x09 => Some(Instruction::RRC(TargetRegister8::C)),
            0x0A => Some(Instruction::RRC(TargetRegister8::D)),
            0x0B => Some(Instruction::RRC(TargetRegister8::E)),
            0x0C => Some(Instruction::RRC(TargetRegister8::H)),
            0x0D => Some(Instruction::RRC(TargetRegister8::L)),
            0x0E => Some(Instruction::RRCHL),
            0x0F => Some(Instruction::RRC(TargetRegister8::A)),

            0x10 => Some(Instruction::RL(TargetRegister8::B)),
            0x11 => Some(Instruction::RL(TargetRegister8::C)),
            0x12 => Some(Instruction::RL(TargetRegister8::D)),
            0x13 => Some(Instruction::RL(TargetRegister8::E)),
            0x14 => Some(Instruction::RL(TargetRegister8::H)),
            0x15 => Some(Instruction::RL(TargetRegister8::L)),
            0x16 => Some(Instruction::RLHL),
            0x17 => Some(Instruction::RL(TargetRegister8::A)),
            0x18 => Some(Instruction::RR(TargetRegister8::B)),
            0x19 => Some(Instruction::RR(TargetRegister8::C)),
            0x1A => Some(Instruction::RR(TargetRegister8::D)),
            0x1B => Some(Instruction::RR(TargetRegister8::E)),
            0x1C => Some(Instruction::RR(TargetRegister8::H)),
            0x1D => Some(Instruction::RR(TargetRegister8::L)),
            0x1E => Some(Instruction::RRHL),
            0x1F => Some(Instruction::RR(TargetRegister8::A)),

            0x20 => None,
            0x21 => None,
            0x22 => None,
            0x23 => None,
            0x24 => None,
            0x25 => None,
            0x26 => None,
            0x27 => None,
            0x28 => None,
            0x29 => None,
            0x2A => None,
            0x2B => None,
            0x2C => None,
            0x2D => None,
            0x2E => None,
            0x2F => None,

            0x30 => None,
            0x31 => None,
            0x32 => None,
            0x33 => None,
            0x34 => None,
            0x35 => None,
            0x36 => None,
            0x37 => None,
            0x38 => None,
            0x39 => None,
            0x3A => None,
            0x3B => None,
            0x3C => None,
            0x3D => None,
            0x3E => None,
            0x3F => None,

            0x40 => None,
            0x41 => None,
            0x42 => None,
            0x43 => None,
            0x44 => None,
            0x45 => None,
            0x46 => None,
            0x47 => None,
            0x48 => None,
            0x49 => None,
            0x4A => None,
            0x4B => None,
            0x4C => None,
            0x4D => None,
            0x4E => None,
            0x4F => None,

            0x50 => None,
            0x51 => None,
            0x52 => None,
            0x53 => None,
            0x54 => None,
            0x55 => None,
            0x56 => None,
            0x57 => None,
            0x58 => None,
            0x59 => None,
            0x5A => None,
            0x5B => None,
            0x5C => None,
            0x5D => None,
            0x5E => None,
            0x5F => None,

            0x60 => None,
            0x61 => None,
            0x62 => None,
            0x63 => None,
            0x64 => None,
            0x65 => None,
            0x66 => None,
            0x67 => None,
            0x68 => None,
            0x69 => None,
            0x6A => None,
            0x6B => None,
            0x6C => None,
            0x6D => None,
            0x6E => None,
            0x6F => None,

            0x70 => None,
            0x71 => None,
            0x72 => None,
            0x73 => None,
            0x74 => None,
            0x75 => None,
            0x76 => None,
            0x77 => None,
            0x78 => None,
            0x79 => None,
            0x7A => None,
            0x7B => None,
            0x7C => None,
            0x7D => None,
            0x7E => None,
            0x7F => None,

            0x80 => None,
            0x81 => None,
            0x82 => None,
            0x83 => None,
            0x84 => None,
            0x85 => None,
            0x86 => None,
            0x87 => None,
            0x88 => None,
            0x89 => None,
            0x8A => None,
            0x8B => None,
            0x8C => None,
            0x8D => None,
            0x8E => None,
            0x8F => None,

            0x90 => None,
            0x91 => None,
            0x92 => None,
            0x93 => None,
            0x94 => None,
            0x95 => None,
            0x96 => None,
            0x97 => None,
            0x98 => None,
            0x99 => None,
            0x9A => None,
            0x9B => None,
            0x9C => None,
            0x9D => None,
            0x9E => None,
            0x9F => None,

            0xA0 => None,
            0xA1 => None,
            0xA2 => None,
            0xA3 => None,
            0xA4 => None,
            0xA5 => None,
            0xA6 => None,
            0xA7 => None,
            0xA8 => None,
            0xA9 => None,
            0xAA => None,
            0xAB => None,
            0xAC => None,
            0xAD => None,
            0xAE => None,
            0xAF => None,

            0xB0 => None,
            0xB1 => None,
            0xB2 => None,
            0xB3 => None,
            0xB4 => None,
            0xB5 => None,
            0xB6 => None,
            0xB7 => None,
            0xB8 => None,
            0xB9 => None,
            0xBA => None,
            0xBB => None,
            0xBC => None,
            0xBD => None,
            0xBE => None,
            0xBF => None,

            0xC0 => None,
            0xC1 => None,
            0xC2 => None,
            0xC3 => None,
            0xC4 => None,
            0xC5 => None,
            0xC6 => None,
            0xC7 => None,
            0xC8 => None,
            0xC9 => None,
            0xCA => None,
            0xCB => None,
            0xCC => None,
            0xCD => None,
            0xCE => None,
            0xCF => None,

            0xD0 => None,
            0xD1 => None,
            0xD2 => None,
            0xD3 => None,
            0xD4 => None,
            0xD5 => None,
            0xD6 => None,
            0xD7 => None,
            0xD8 => None,
            0xD9 => None,
            0xDA => None,
            0xDB => None,
            0xDC => None,
            0xDD => None,
            0xDE => None,
            0xDF => None,

            0xE0 => None,
            0xE1 => None,
            0xE2 => None,
            0xE3 => None,
            0xE4 => None,
            0xE5 => None,
            0xE6 => None,
            0xE7 => None,
            0xE8 => None,
            0xE9 => None,
            0xEA => None,
            0xEB => None,
            0xEC => None,
            0xED => None,
            0xEE => None,
            0xEF => None,

            0xF0 => None,
            0xF1 => None,
            0xF2 => None,
            0xF3 => None,
            0xF4 => None,
            0xF5 => None,
            0xF6 => None,
            0xF7 => None,
            0xF8 => None,
            0xF9 => None,
            0xFA => None,
            0xFB => None,
            0xFC => None,
            0xFD => None,
            0xFE => None,
            0xFF => None,
        }
    }

    fn frombytenotprefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x01 => None,
            0x02 => Some(Instruction::LDBCA),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x06 => None,
            0x07 => Some(Instruction::RLCA),
            0x08 => None,
            0x09 => Some(Instruction::ADDR16(TargetRegister16::BC)),
            0x0A => Some(Instruction::LDABC),
            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x0E => None,
            0x0F => Some(Instruction::RRCA),

            0x10 => Some(Instruction::STOP),
            0x11 => None,
            0x12 => Some(Instruction::LDDEA),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x16 => None,
            0x17 => Some(Instruction::RLA),
            0x18 => Some(Instruction::JR),
            0x19 => Some(Instruction::ADDR16(TargetRegister16::DE)),
            0x1A => Some(Instruction::LDADE),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x1E => None,
            0x1F => Some(Instruction::RRA),

            0x20 => None,
            0x21 => None,
            0x22 => None,
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x26 => None,
            0x27 => Some(Instruction::DAA),
            0x28 => None,
            0x29 => Some(Instruction::ADDR16(TargetRegister16::HL)),
            0x2A => None,
            0x2B => None,
            0x2C => None,
            0x2D => None,
            0x2E => None,
            0x2F => Some(Instruction::CPL),

            0x30 => None,
            0x31 => None,
            0x32 => None,
            0x33 => None,
            0x34 => None,
            0x35 => None,
            0x36 => None,
            0x37 => None,
            0x38 => None,
            0x39 => None,
            0x3A => None,
            0x3B => None,
            0x3C => None,
            0x3D => None,
            0x3E => None,
            0x3F => Some(Instruction::CCF),

            0x40 => Some(Instruction::LD(TargetRegister8::B, TargetRegister8::B)), // LD B, B
            0x41 => Some(Instruction::LD(TargetRegister8::B, TargetRegister8::C)), // LD B, C
            0x42 => Some(Instruction::LD(TargetRegister8::B, TargetRegister8::D)), // LD B, D
            0x43 => Some(Instruction::LD(TargetRegister8::B, TargetRegister8::E)), // LD B, E
            0x44 => Some(Instruction::LD(TargetRegister8::B, TargetRegister8::H)), // LD B, H
            0x45 => Some(Instruction::LD(TargetRegister8::B, TargetRegister8::L)), // LD B, L
            0x46 => None,                                                          // LD B, HL
            0x47 => Some(Instruction::LD(TargetRegister8::B, TargetRegister8::A)), // LD B, A
            0x48 => Some(Instruction::LD(TargetRegister8::C, TargetRegister8::B)), // LD C, B
            0x49 => Some(Instruction::LD(TargetRegister8::C, TargetRegister8::C)), // LD C, C
            0x4A => Some(Instruction::LD(TargetRegister8::C, TargetRegister8::D)), // LD C, D
            0x4B => Some(Instruction::LD(TargetRegister8::C, TargetRegister8::E)), // LD C, E
            0x4C => Some(Instruction::LD(TargetRegister8::C, TargetRegister8::H)), // LD C, H
            0x4D => Some(Instruction::LD(TargetRegister8::C, TargetRegister8::L)), // LD C, L
            0x4E => None,                                                          // LD C, HL
            0x4F => Some(Instruction::LD(TargetRegister8::C, TargetRegister8::A)), // LD C, A

            0x50 => Some(Instruction::LD(TargetRegister8::D, TargetRegister8::B)), // LD D, B
            0x51 => Some(Instruction::LD(TargetRegister8::D, TargetRegister8::C)), // LD D, C
            0x52 => Some(Instruction::LD(TargetRegister8::D, TargetRegister8::D)), // LD D, D
            0x53 => Some(Instruction::LD(TargetRegister8::D, TargetRegister8::E)), // LD D, E
            0x54 => Some(Instruction::LD(TargetRegister8::D, TargetRegister8::H)), // LD D, H
            0x55 => Some(Instruction::LD(TargetRegister8::D, TargetRegister8::L)), // LD D, L
            0x56 => None,                                                          // LD D, HL
            0x57 => Some(Instruction::LD(TargetRegister8::D, TargetRegister8::A)), // LD D, A
            0x58 => Some(Instruction::LD(TargetRegister8::E, TargetRegister8::B)), // LD E, B
            0x59 => Some(Instruction::LD(TargetRegister8::E, TargetRegister8::C)), // LD E, C
            0x5A => Some(Instruction::LD(TargetRegister8::E, TargetRegister8::D)), // LD E, D
            0x5B => Some(Instruction::LD(TargetRegister8::E, TargetRegister8::E)), // LD E, E
            0x5C => Some(Instruction::LD(TargetRegister8::E, TargetRegister8::H)), // LD E, H
            0x5D => Some(Instruction::LD(TargetRegister8::E, TargetRegister8::L)), // LD E, L
            0x5E => None,                                                          // LD E, HL
            0x5F => Some(Instruction::LD(TargetRegister8::E, TargetRegister8::A)), // LD E, A

            0x60 => Some(Instruction::LD(TargetRegister8::L, TargetRegister8::B)), // LD L, B
            0x61 => Some(Instruction::LD(TargetRegister8::L, TargetRegister8::C)), // LD L, C
            0x62 => Some(Instruction::LD(TargetRegister8::L, TargetRegister8::D)), // LD L, D
            0x63 => Some(Instruction::LD(TargetRegister8::L, TargetRegister8::E)), // LD L, E
            0x64 => Some(Instruction::LD(TargetRegister8::L, TargetRegister8::H)), // LD L, H
            0x65 => Some(Instruction::LD(TargetRegister8::L, TargetRegister8::L)), // LD L, L
            0x66 => None,                                                          // LD L, HL
            0x67 => Some(Instruction::LD(TargetRegister8::L, TargetRegister8::A)), // LD L, A
            0x68 => Some(Instruction::LD(TargetRegister8::H, TargetRegister8::B)), // LD H, B
            0x69 => Some(Instruction::LD(TargetRegister8::H, TargetRegister8::C)), // LD H, C
            0x6A => Some(Instruction::LD(TargetRegister8::H, TargetRegister8::D)), // LD H, D
            0x6B => Some(Instruction::LD(TargetRegister8::H, TargetRegister8::E)), // LD H, E
            0x6C => Some(Instruction::LD(TargetRegister8::H, TargetRegister8::H)), // LD H, H
            0x6D => Some(Instruction::LD(TargetRegister8::H, TargetRegister8::L)), // LD H, L
            0x6E => None,                                                          // LD H, HL
            0x6F => Some(Instruction::LD(TargetRegister8::H, TargetRegister8::A)), // LD H, A

            0x70 => None,
            0x71 => None,
            0x72 => None,
            0x73 => None,
            0x74 => None,
            0x75 => None,
            0x76 => Some(Instruction::HALT),
            0x77 => None,
            0x78 => None,
            0x79 => None,
            0x7A => None,
            0x7B => None,
            0x7C => None,
            0x7D => None,
            0x7E => None,
            0x7F => Some(Instruction::LD(TargetRegister8::A, TargetRegister8::A)), // LD A, A

            0x80 => Some(Instruction::ADDR8(TargetRegister8::B)), // ADD A, B
            0x81 => Some(Instruction::ADDR8(TargetRegister8::C)), // ADD A, C
            0x82 => Some(Instruction::ADDR8(TargetRegister8::D)), // ADD A, D
            0x83 => Some(Instruction::ADDR8(TargetRegister8::E)), // ADD A, E
            0x84 => Some(Instruction::ADDR8(TargetRegister8::H)), // ADD A, H
            0x85 => Some(Instruction::ADDR8(TargetRegister8::L)), // ADD A, L
            0x86 => Some(Instruction::ADDHL),                     // ADD A, HL
            0x87 => Some(Instruction::ADDR8(TargetRegister8::A)), // ADD A, A
            0x88 => Some(Instruction::ADCR8(TargetRegister8::B)), // ADC A, B
            0x89 => Some(Instruction::ADCR8(TargetRegister8::C)), // ADC A, C
            0x8A => Some(Instruction::ADCR8(TargetRegister8::D)), // ADC A, D
            0x8B => Some(Instruction::ADCR8(TargetRegister8::E)), // ADC A, E
            0x8C => Some(Instruction::ADCR8(TargetRegister8::H)), // ADC A, H
            0x8D => Some(Instruction::ADCR8(TargetRegister8::L)), // ADC A, L
            0x8E => Some(Instruction::ADCHL),                     // ADC A, HL
            0x8F => Some(Instruction::ADCR8(TargetRegister8::A)), // ADC A, A

            0x90 => Some(Instruction::SUBR8(TargetRegister8::B)), // SUB A, B
            0x91 => Some(Instruction::SUBR8(TargetRegister8::C)), // SUB A, C
            0x92 => Some(Instruction::SUBR8(TargetRegister8::D)), // SUB A, D
            0x93 => Some(Instruction::SUBR8(TargetRegister8::E)), // SUB A, E
            0x94 => Some(Instruction::SUBR8(TargetRegister8::H)), // SUB A, H
            0x95 => Some(Instruction::SUBR8(TargetRegister8::L)), // SUB A, L
            0x96 => Some(Instruction::SUBHL),                     // SUB A, HL
            0x97 => Some(Instruction::SUBR8(TargetRegister8::A)), // SUB A, A
            0x98 => Some(Instruction::SBCR8(TargetRegister8::B)), // SBC A, B
            0x99 => Some(Instruction::SBCR8(TargetRegister8::C)), // SBC A, C
            0x9A => Some(Instruction::SBCR8(TargetRegister8::D)), // SBC A, D
            0x9B => Some(Instruction::SBCR8(TargetRegister8::E)), // SBC A, E
            0x9C => Some(Instruction::SBCR8(TargetRegister8::H)), // SBC A, H
            0x9D => Some(Instruction::SBCR8(TargetRegister8::L)), // SBC A, L
            0x9E => Some(Instruction::SBCHL),                     // SBC A, HL
            0x9F => Some(Instruction::SBCR8(TargetRegister8::A)), // SBC A, A

            0xA0 => Some(Instruction::ANDR8(TargetRegister8::B)), // AND A, B
            0xA1 => Some(Instruction::ANDR8(TargetRegister8::C)), // AND A, C
            0xA2 => Some(Instruction::ANDR8(TargetRegister8::D)), // AND A, D
            0xA3 => Some(Instruction::ANDR8(TargetRegister8::E)), // AND A, E
            0xA4 => Some(Instruction::ANDR8(TargetRegister8::H)), // AND A, H
            0xA5 => Some(Instruction::ANDR8(TargetRegister8::L)), // AND A, L
            0xA6 => Some(Instruction::ANDHL),                     // AND A, HL
            0xA7 => Some(Instruction::ANDR8(TargetRegister8::A)), // AND A, A
            0xA8 => Some(Instruction::XORR8(TargetRegister8::B)), // XOR A, B
            0xA9 => Some(Instruction::XORR8(TargetRegister8::C)), // XOR A, C
            0xAA => Some(Instruction::XORR8(TargetRegister8::D)), // XOR A, D
            0xAB => Some(Instruction::XORR8(TargetRegister8::E)), // XOR A, E
            0xAC => Some(Instruction::XORR8(TargetRegister8::H)), // XOR A, H
            0xAD => Some(Instruction::XORR8(TargetRegister8::L)), // XOR A, L
            0xAE => Some(Instruction::XORHL),                     // XOR A, HL
            0xAF => Some(Instruction::XORR8(TargetRegister8::A)), // XOR A, A

            0xB0 => Some(Instruction::ORR8(TargetRegister8::B)), // OR A, B
            0xB1 => Some(Instruction::ORR8(TargetRegister8::C)), // OR A, C
            0xB2 => Some(Instruction::ORR8(TargetRegister8::D)), // OR A, D
            0xB3 => Some(Instruction::ORR8(TargetRegister8::E)), // OR A, E
            0xB4 => Some(Instruction::ORR8(TargetRegister8::H)), // OR A, H
            0xB5 => Some(Instruction::ORR8(TargetRegister8::L)), // OR A, L
            0xB6 => Some(Instruction::ORHL),                     // OR A, HL
            0xB7 => Some(Instruction::ORR8(TargetRegister8::A)), // OR A, A
            0xB8 => Some(Instruction::CPR8(TargetRegister8::B)), // CP A, B
            0xB9 => Some(Instruction::CPR8(TargetRegister8::C)), // CP A, C
            0xBA => Some(Instruction::CPR8(TargetRegister8::D)), // CP A, D
            0xBB => Some(Instruction::CPR8(TargetRegister8::E)), // CP A, E
            0xBC => Some(Instruction::CPR8(TargetRegister8::H)), // CP A, H
            0xBD => Some(Instruction::CPR8(TargetRegister8::L)), // CP A, L
            0xBE => Some(Instruction::CPHL),                     // CP A, HL
            0xBF => Some(Instruction::CPR8(TargetRegister8::A)), // CP A, A

            0xC0 => Some(Instruction::RETF(Comparison::NONZERO)), // RET NZ
            0xC1 => Some(Instruction::POP(TargetRegister16::BC)), // POP BC
            0xC2 => Some(Instruction::JPF(Comparison::NONZERO)), // JP NZ, u16
            0xC3 => Some(Instruction::JP), // JP u16
            0xC4 => None,
            0xC5 => Some(Instruction::PUSH(TargetRegister16::BC)), // PUSH BC
            0xC6 => Some(Instruction::ADDU8), // ADD A, u8
            0xC7 => Some(Instruction::RST00H), // RST 00h
            0xC8 => Some(Instruction::RETF(Comparison::ZERO)),   // RET Z
            0xC9 => Some(Instruction::RET),    // RET
            0xCA => Some(Instruction::JPF(Comparison::ZERO)), // JP Z, u16,
            0xCB => panic!("byte CB is the prefix byte and has no non prefix operation"),
            0xCC => None,
            0xCD => None,
            0xCE => None,
            0xCF => None,

            0xD0 => None,
            0xD1 => Some(Instruction::POP(TargetRegister16::DE)),
            0xD2 => None,
            0xD3 => panic!("byte {:X} has no documented op", byte),
            0xD4 => None,
            0xD5 => Some(Instruction::PUSH(TargetRegister16::DE)),
            0xD6 => Some(Instruction::SUBU8),
            0xD7 => None,
            0xD8 => None,
            0xD9 => None,
            0xDA => None,
            0xDB => panic!("byte {:X} has no documented op", byte),
            0xDC => None,
            0xDD => panic!("byte {:X} has no documented op", byte),
            0xDE => None,
            0xDF => None,

            0xE0 => Some(Instruction::LDAFF00U8A),
            0xE1 => Some(Instruction::POP(TargetRegister16::HL)),
            0xE2 => None,
            0xE3 => panic!("byte {:X} has no documented op", byte),
            0xE4 => panic!("byte {:X} has no documented op", byte),
            0xE5 => Some(Instruction::PUSH(TargetRegister16::HL)),
            0xE6 => Some(Instruction::ANDU8),
            0xE7 => None,
            0xE8 => None,
            0xE9 => None,
            0xEA => None,
            0xEB => panic!("byte {:X} has no documented op", byte),
            0xEC => panic!("byte {:X} has no documented op", byte),
            0xED => panic!("byte {:X} has no documented op", byte),
            0xEE => None,
            0xEF => None,

            0xF0 => Some(Instruction::LDAFF00U8), // LD A, (FF00+u8)
            0xF1 => Some(Instruction::POP(TargetRegister16::AF)), // POP AF
            0xF2 => Some(Instruction::LDAFF00C),  // LD A, (FF00+C)
            0xF3 => Some(Instruction::DI),
            0xF4 => panic!("byte {:X} has no documented op", byte),
            0xF5 => Some(Instruction::PUSH(TargetRegister16::AF)), // PUSH AF
            0xF6 => Some(Instruction::ORU8),                       // OR A, u8
            0xF7 => Some(Instruction::RST30H),                     // RST 30h
            0xF8 => None,                                          // LD HL, SP+i8
            0xF9 => None,                                          // LD SP, HL
            0xFA => Some(Instruction::LDAU16),                     // LD A, u16
            0xFB => Some(Instruction::EI),
            0xFC => panic!("byte {:X} has no documented op", byte),
            0xFD => panic!("byte {:X} has no documented op", byte),
            0xFE => Some(Instruction::CPU8), // CP A, u8
            0xFF => None,                    // RST 38h
        }
    }
}
