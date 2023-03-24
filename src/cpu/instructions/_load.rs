use super::*;
use crate::cpu::{FlagsRegister, GameboyChip};
use crate::utils::*;

// impl GameboyChip {

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
