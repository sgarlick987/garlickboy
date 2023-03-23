use super::*;
use crate::cpu::{FlagsRegister, GameboyChip};
use crate::utils::*;

// impl GameboyChip {
//     // LD (HL),B - 0x70
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (8t)
//     // fetch
//     // write	B->(HL)
//     pub fn ld_hl_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //write
//         match target {
//             TargetRegister8::A => {
//                 self.write_byte(self.registers.get_hl(), self.registers.a);
//             }
//             _ => {
//                 panic!("{:?} unimplemented LDHLR8 Instruction", target);
//             }
//         }

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD A,(HL-) - 0x3A
//     // Length: 1 byte
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timing
//     // without branch (8t)
//     // fetch
//     // read	(HL--)->A
//     pub fn ldd_a_hl(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let hl = self.registers.get_hl();
//         self.registers.a = self.read_byte(hl);
//         self.registers.set_hl(hl.wrapping_sub(1));

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD A,(HL+) - 0x2A
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (8t)
//     // fetch
//     // read	(HL++)->A
//     pub fn ldi_a_hl(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let hl = self.registers.get_hl();
//         self.registers.a = self.read_byte(hl);
//         self.registers.set_hl(hl.wrapping_add(1));

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }


//     // LD (HL+),A - 0x22
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (8t)
//     // fetch
//     // write	A->(HL++)
//     pub fn ldi_hl_a(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //write
//         let hl = self.registers.get_hl();
//         self.write_byte(hl, self.registers.a);
//         self.registers.set_hl(hl.wrapping_add(1));

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD (FF00+u8),A - 0xE0
//     // Length: 2 bytes
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (12t)
//     // fetch
//     // read	u8
//     // write	A->(FF00+u8)
//     pub fn ld_ff00u8_a(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let address = 0xFF00 + self.read_byte_pc_lower() as u16;

//         //write
//         self.write_byte(address, self.registers.a);

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD (DE),A - 0x12
//     // Length: 1 byte
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timing
//     // without branch (8t)
//     // fetch
//     // write	A->(DE)
//     pub fn ld_de_a(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //write
//         self.write_byte(self.registers.get_de(), self.registers.a);

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD E,(HL) - 0x5E
//     // Length: 1 byte
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timing
//     // without branch (8t)
//     // fetch
//     // read	(HL)->E
//     pub fn ld_r8_hl(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.read_byte(self.registers.get_hl());
//         self.set_register_from_enum(target, byte);

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD A,(FF00+u8) - 0xF0
//     // Length: 2 bytes
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (12t)
//     // fetch
//     // read	u8
//     // read	(FF00+u8)->A
//     pub fn ld_a_ff00u8(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let address = 0xFF00 + self.read_byte_pc_lower() as u16;
//         cycles_used += self.sync();

//         //read
//         self.registers.a = self.read_byte(address);

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }


//     // LD A,(BC) - 0x0A
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (8t)
//     // fetch
//     // read	(BC)->A
//     pub fn ld_a_ptr(&mut self, target: &TargetPointer) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         self.registers.a = match target {
//             TargetPointer::BC => self.read_byte(self.registers.get_bc()),
//             TargetPointer::DE => self.read_byte(self.registers.get_de()),
//             TargetPointer::HL => self.read_byte(self.registers.get_hl()),
//         };

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }
//     // LD A,(u16) - 0xFA
//     // Length: 3 bytes
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timing
//     // without branch (16t)
//     // fetch
//     // read	u16:lower
//     // read	u16:upper
//     // read	(u16)->A.
//     pub fn ld_a_u16(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let lower = self.read_byte_pc_lower();
//         cycles_used += self.sync();

//         //read
//         let upper = self.read_byte_pc_upper();
//         cycles_used += self.sync();

//         //read
//         self.registers.a = self.read_byte(merge_bytes(upper, lower));

//         self.pc = self.pc.wrapping_add(3);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // PUSH BC - 0xC5
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x16/lsm
//     // Timingwithout branch (16t)
//     // fetch
//     // internal
//     // write	B->(--SP)
//     // write	C->(--SP)
//     pub fn push(&mut self, target: &TargetPushPop) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //internal
//         cycles_used += self.sync();

//         //write write
//         match target {
//             TargetPushPop::AF => {
//                 self._push(self.registers.a);
//                 cycles_used += self.sync();
//                 self._push(self.registers.get_f());
//             }
//             TargetPushPop::HL => {
//                 self._push(self.registers.h);
//                 cycles_used += self.sync();
//                 self._push(self.registers.l);
//             }
//             TargetPushPop::BC => {
//                 self._push(self.registers.b);
//                 cycles_used += self.sync();
//                 self._push(self.registers.c);
//             }
//             TargetPushPop::DE => {
//                 self._push(self.registers.d);
//                 cycles_used += self.sync();
//                 self._push(self.registers.e);
//             }
//         }

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // POP BC - 0xC1
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x16/lsm
//     // Timingwithout branch (12t)
//     // fetch
//     // read	(SP++)->C
//     // read	(SP++)->B
//     pub fn pop(&mut self, target: &TargetPushPop) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let lower = self._pop();
//         match target {
//             TargetPushPop::AF => self.registers.flags = FlagsRegister::from(lower),
//             TargetPushPop::HL => self.registers.l = lower,
//             TargetPushPop::BC => self.registers.c = lower,
//             TargetPushPop::DE => self.registers.e = lower,
//         }
//         cycles_used += self.sync();

//         //read
//         let upper = self._pop();
//         match target {
//             TargetPushPop::AF => self.registers.a = upper,
//             TargetPushPop::HL => self.registers.h = upper,
//             TargetPushPop::BC => self.registers.b = upper,
//             TargetPushPop::DE => self.registers.d = upper,
//         }

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD D,u8 - 0x16
//     // Length: 2 bytes
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (8t)
//     // fetch
//     // read	u8->D
//     pub fn ld_r8_u8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.read_byte_pc_lower();

//         self.set_register_from_enum(target, byte);

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD B,B - 0x40
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (4t)
//     // fetch
//     pub fn ld_r8_r8(&mut self, target: &TargetRegister8, source: &TargetRegister8) -> u8 {
//         //fetch
//         let value = self.get_register_from_enum(source);
//         self.set_register_from_enum(target, value);

//         self.pc = self.pc.wrapping_add(1);
//         let cycles_used = self.sync();
//         cycles_used
//     }

//     // LD (u16),A - 0xEA
//     // Length: 3 bytes
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timingwithout branch (16t)
//     // fetch
//     // read	u16:lower
//     // read	u16:upper
//     // write	A->(u16)
//     pub fn ld_u16_a(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read lower
//         let lower = self.read_byte_pc_lower();
//         cycles_used += self.sync();

//         //read upper
//         let upper = self.read_byte_pc_upper();
//         cycles_used += self.sync();

//         //write
//         let address = merge_bytes(upper, lower);
//         self.write_byte(address, self.registers.a);

//         self.pc = self.pc.wrapping_add(3);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     // LD (HL),u8 - 0x36
//     // Length: 2 bytes
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/lsm
//     // Timing
//     // without branch (12t)
//     // fetch
//     // read	u8
//     // write	(HL)
//     pub fn ld_hl_u8(&mut self) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         //read
//         let byte = self.read_byte_pc_lower();
//         cycles_used += self.sync();

//         //write
//         self.write_byte(self.registers.get_hl(), byte);

//         self.pc = self.pc.wrapping_add(2);
//         cycles_used += self.sync();
//         cycles_used
//     }

//     pub fn _push(&mut self, byte: u8) {
//         self.registers.sp = self.registers.sp.wrapping_sub(1);
//         self.write_byte(self.registers.sp, byte);
//     }

//     pub fn _pop(&mut self) -> u8 {
//         let byte = self.read_byte(self.registers.sp);
//         self.registers.sp = self.registers.sp.wrapping_add(1);
//         byte
//     }
// }

// #[cfg(test)]
// mod tests {

//     use crate::address::*;
//     use coverage_helper::test;
//     use mockall::{predicate, Sequence};

//     use super::*;

//     fn setup_bus(cycles: u8) -> Box<MockBus> {
//         let syncs = cycles / 4;
//         let mut bus = Box::new(MockBus::new());
//         bus.expect_sync().times(syncs as usize).return_const(0);
//         bus
//     }

//     fn setup_cpu(cycles: u8) -> GameboyChip {
//         GameboyChip::new(setup_bus(cycles))
//     }

//     #[test]
//     fn test_ld_u16_a() {
//         const CYCLES: u8 = 16;
//         const LENGTH: u16 = 3;
//         const ADDRESS: u16 = 0x2310;
//         const LOWER: u8 = 0x10;
//         const UPPER: u8 = 0x23;
//         const REGISTER_VALUE: u8 = 0xD2;

//         let mut bus = setup_bus(CYCLES);
//         let mut seq = Sequence::new();
//         bus.expect_read_byte()
//             .with(predicate::eq(1))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(LOWER);
//         bus.expect_read_byte()
//             .with(predicate::eq(2))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(UPPER);
//         bus.expect_write_byte()
//             .with(predicate::eq(ADDRESS), predicate::eq(REGISTER_VALUE))
//             .once()
//             .return_const(());
//         let mut cpu = GameboyChip::new(bus);
//         cpu.registers.a = REGISTER_VALUE;
//         cpu.ld_u16_a();
//         assert_eq!(cpu.pc, LENGTH);
//     }

//     #[test]
//     fn test_ld_r8_u8() {
//         const CYCLES: u8 = 8;
//         const LENGTH: u16 = 2;
//         const VALUE: u8 = 0xF3;

//         let targets = [
//             TargetRegister8::A,
//             TargetRegister8::B,
//             TargetRegister8::C,
//             TargetRegister8::D,
//             TargetRegister8::E,
//             TargetRegister8::H,
//             TargetRegister8::L,
//         ];
//         for target in &targets {
//             let mut bus = setup_bus(CYCLES);
//             bus.expect_read_byte()
//                 .with(predicate::eq(1))
//                 .once()
//                 .return_const(VALUE);
//             let mut cpu = GameboyChip::new(bus);
//             cpu.ld_r8_u8(target);
//             assert_eq!(cpu.pc, LENGTH);
//             assert_eq!(cpu.get_register_from_enum(target), VALUE);
//         }
//     }

//     // fn ld_r8_u8(&mut self, target: &TargetRegister8) -> u8 {
//     //     //fetch
//     //     let mut cycles_used = self.sync();

//     //     //read
//     //     let byte = self.read_byte_pc_lower();

//     //     match target {
//     //         TargetRegister8::A => self.registers.a = byte,
//     //         TargetRegister8::B => self.registers.b = byte,
//     //         TargetRegister8::C => self.registers.c = byte,
//     //         TargetRegister8::D => self.registers.d = byte,
//     //         TargetRegister8::E => self.registers.e = byte,
//     //         TargetRegister8::H => self.registers.h = byte,
//     //         TargetRegister8::L => self.registers.l = byte,
//     //     }

//     //     self.pc = self.pc.wrapping_add(2);
//     //     cycles_used += self.sync();
//     //     cycles_used
//     // }

//     #[test]
//     fn test_ld_r8_r8() {
//         const CYCLES: u8 = 4;
//         const LENGTH: u16 = 1;

//         let targets = [
//             TargetRegister8::A,
//             TargetRegister8::B,
//             TargetRegister8::C,
//             TargetRegister8::D,
//             TargetRegister8::E,
//             TargetRegister8::H,
//             TargetRegister8::L,
//         ];

//         for src in &targets {
//             for dst in &targets {
//                 let mut cpu = setup_cpu(CYCLES);

//                 cpu.set_register_from_enum(src, 0xFF);

//                 cpu.ld_r8_r8(dst, src);

//                 assert_eq!(cpu.pc, LENGTH);
//                 assert_eq!(
//                     cpu.get_register_from_enum(dst),
//                     cpu.get_register_from_enum(src)
//                 );
//             }
//         }
//     }
// }