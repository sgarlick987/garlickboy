use super::*;
use crate::cpu::GameboyChip;
use crate::utils::*;

// impl GameboyChip {
//     // ADD HL,DE - 0x19
//     // Length: 1 byte
//     // Flags
//     // Zero	unmodified
//     // Negative	unset
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x16/alu
//     // Timing
//     // without branch (8t)
//     // fetch	Probably writes to L here
//     // internal	Probably writes to H here
//     pub fn add_r16(&mut self, target: &TargetRegister16) -> u8 {
//         let hl = self.registers.get_hl();

//         let value = match target {
//             TargetRegister16::DE => self.registers.get_de(),
//             TargetRegister16::HL => self.registers.get_hl(),
//             TargetRegister16::BC => self.registers.get_bc(),
//             TargetRegister16::SP => self.registers.sp,
//             _ => panic!("{:?} not implemented for add r16", target),
//         };

//         let (added, overflowed) = hl.carrying_add(value, self.registers.flags.carry);
//         let (h, l) = split_bytes(added);
//         self.registers.flags.half_carry = false;
//         self.registers.flags.carry = overflowed;
//         self.registers.flags.negative = false;

//         //fetch set l
//         let mut cycles_used = self.sync();
//         self.registers.l = l;

//         //internal set h
//         self.registers.h = h;

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // XOR A,u8 - 0xEE
//     // Length: 2 bytes
//     // Flags
//     // Zero	dependent
//     // Negative	unset
//     // Half Carry	unset
//     // Carry	unset
//     // Group: x8/alu
//     // Timing
//     // without branch (8t)
//     // fetch
//     // read	u8
//     pub fn xor_u8(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.read_byte_pc_lower();

//         self.registers.a ^= byte;
//         self.registers.flags.zero = self.registers.a == 0;
//         self.registers.flags.negative = false;
//         self.registers.flags.half_carry = false;
//         self.registers.flags.carry = false;

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // OR A,B - 0xB0
//     // Length: 1 byte
//     // Flags
//     // Zero	dependent
//     // Negative	unset
//     // Half Carry	unset
//     // Carry	unset
//     // Group: x8/alu
//     // Timing
//     // without branch (4t)
//     // fetch
//     pub fn or_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         self.registers.a |= self.get_register_from_enum(target);
//         self.registers.flags.zero = self.registers.a == 0;
//         self.registers.flags.negative = false;
//         self.registers.flags.half_carry = false;
//         self.registers.flags.carry = false;

//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }


//     // CP A,B - 0xB8
//     // Length: 1 byte
//     // Flags
//     // Zero	dependent
//     // Negative	set
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timing
//     // without branch (4t)
//     // fetch
//     pub fn cp_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch

//         let byte = self.get_register_from_enum(target);
//         let a = self.registers.a;
//         self.registers.flags.negative = true;
//         self.registers.flags.zero = a == byte;
//         self.registers.flags.carry = a < byte;
//         self.registers.flags.half_carry = bytes_half_carry(a, byte);

//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }

//     // CP A,(HL) - 0xBE
//     // Length: 1 byte
//     // FlagsZero	dependent
//     // Negative	set
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timingwithout branch (8t)
//     // fetch
//     // read	(HL)
//     pub fn cp_hl(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let hl = self.registers.get_hl();
//         let byte = self.read_byte(hl);
//         let a = self.registers.a;
//         self.registers.flags.negative = true;
//         self.registers.flags.zero = a == byte;
//         self.registers.flags.carry = a < byte;
//         self.registers.flags.half_carry = bytes_half_carry(a, byte);

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // CPL - 0x2F
//     // Length: 1 byte
//     // Flags
//     // Zero	unmodified
//     // Negative	set
//     // Half Carry	set
//     // Carry	unmodified
//     // Group: x8/alu
//     // Timing
//     // without branch (4t)
//     // fetch
//     pub fn cpl(&mut self) -> u8 {
//         self.registers.a ^= 0xFF;
//         self.registers.flags.negative = true;
//         self.registers.flags.half_carry = true;
//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }

//     // AND A,u8 - 0xE6
//     // Length: 2 bytes
//     // Flags
//     // Zero	dependent
//     // Negative	unset
//     // Half Carry	set
//     // Carry	unset
//     // Group: x8/alu
//     // Timing
//     // without branch (8t)
//     // fetch
//     // read	u8
//     pub fn and_u8(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.read_byte_pc_lower();
//         self.registers.a &= byte;

//         self.registers.flags.negative = false;
//         self.registers.flags.carry = false;
//         self.registers.flags.zero = self.registers.a == 0;
//         self.registers.flags.half_carry = true;
//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // AND A,B - 0xA0
//     // Length: 1 byte
//     // Flags
//     // Zero	dependent
//     // Negative	unset
//     // Half Carry	set
//     // Carry	unset
//     // Group: x8/alu
//     // Timing
//     // without branch (4t)
//     // fetch
//     pub fn and_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.get_register_from_enum(target);
//         self.registers.a &= byte;

//         self.registers.flags.negative = false;
//         self.registers.flags.carry = false;
//         self.registers.flags.zero = self.registers.a == 0;
//         self.registers.flags.half_carry = true;
//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // ADD A,(HL) - 0x86
//     // Length: 1 byte
//     // FlagsZero	dependent
//     // Negative	unset
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timingwithout branch (8t)
//     // fetch
//     // read	(HL)
//     pub fn add_hl(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let hl = self.registers.get_hl();
//         let byte = self.read_byte(hl);
//         self.registers.a = self._add(byte, false);

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // SUB A,B - 0x90
//     // Length: 1 byte
//     // FlagsZero	dependent
//     // Negative	set
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timingwithout branch (4t)
//     // fetch
//     pub fn sub_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         self.registers.a = match target {
//             TargetRegister8::A => self._sub(self.registers.a, false),
//             TargetRegister8::B => self._sub(self.registers.b, false),
//             TargetRegister8::C => self._sub(self.registers.c, false),
//             TargetRegister8::D => self._sub(self.registers.d, false),
//             TargetRegister8::E => self._sub(self.registers.e, false),
//             TargetRegister8::H => self._sub(self.registers.h, false),
//             TargetRegister8::L => self._sub(self.registers.l, false),
//         };

//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }

//     // ADD A,B - 0x80
//     // Length: 1 byte
//     // FlagsZero	dependent
//     // Negative	unset
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timingwithout branch (4t)
//     // fetch
//     pub fn add_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         self.registers.a = match target {
//             TargetRegister8::A => self._add(self.registers.a, false),
//             TargetRegister8::B => self._add(self.registers.b, false),
//             TargetRegister8::C => self._add(self.registers.c, false),
//             TargetRegister8::D => self._add(self.registers.d, false),
//             TargetRegister8::E => self._add(self.registers.e, false),
//             TargetRegister8::H => self._add(self.registers.h, false),
//             TargetRegister8::L => self._add(self.registers.l, false),
//         };

//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }

//     // ADD A,u8 - 0xC6
//     // Length: 2 bytes
//     // Flags
//     // Zero	dependent
//     // Negative	unset
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timing
//     // without branch (8t)
//     // fetch
//     // read	u8
//     pub fn add_u8(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.read_byte_pc_lower();
//         cycles_used += self.sync();

//         self._add(byte, false);

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // ADC A,B - 0x88
//     // Length: 1 byte
//     // FlagsZero	dependent
//     // Negative	unset
//     // Half Carry	dependent
//     // Carry	dependent
//     // Group: x8/alu
//     // Timingwithout branch (4t)
//     // fetch
//     pub fn adc_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         self.registers.a = match target {
//             TargetRegister8::A => self._add(self.registers.a, self.registers.flags.carry),
//             TargetRegister8::B => self._add(self.registers.b, self.registers.flags.carry),
//             TargetRegister8::C => self._add(self.registers.c, self.registers.flags.carry),
//             TargetRegister8::D => self._add(self.registers.d, self.registers.flags.carry),
//             TargetRegister8::E => self._add(self.registers.e, self.registers.flags.carry),
//             TargetRegister8::H => self._add(self.registers.h, self.registers.flags.carry),
//             TargetRegister8::L => self._add(self.registers.l, self.registers.flags.carry),
//         };

//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }
// }

// impl GameboyChip {
//     pub fn _add(&mut self, value: u8, carry: bool) -> u8 {
//         let (added, overflowed) = self.registers.a.carrying_add(value, carry);
//         self.registers.flags.zero = added == 0;
//         self.registers.flags.negative = false;
//         self.registers.flags.carry = overflowed;
//         self.registers.flags.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;

//         added
//     }

//     pub fn _sub(&mut self, value: u8, carry: bool) -> u8 {
//         let (subbed, overflowed) = self.registers.a.borrowing_sub(value, carry);
//         self.registers.flags.zero = subbed == 0;
//         self.registers.flags.negative = true;
//         self.registers.flags.carry = overflowed;
//         (_, self.registers.flags.half_carry) =
//             (self.registers.a & 0x0F).overflowing_sub(value & 0x0F);

//         subbed
//     }
// }
