//     // INC B - 0x04
//     // Length: 1 byte
//     // FlagsZero	dependent
//     // Negative	unset
//     // Half Carry	dependent
//     // Carry	unmodified
//     // Group: x8/alu
//     // Timingwithout branch (4t)
//     // fetch

//     // INC BC - 0x03
//     // Length: 1 byte
//     // FlagsZero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x16/alu
//     // Timingwithout branch (8t)
//     // fetch	Probably writes to C here
//     // internal	Probably writes to B here

//     //TODO: inc impl half carry
//     //TODO: fix r16 timing
//     //TODO: r8 sync before or after set
//     pub fn inc(&mut self, target: &TargetIncDec) -> u8 {
//         let mut cycles_used = 0;

//         match target {
//             TargetIncDec::A => {
//                 self.registers.a = self.registers.a.wrapping_add(1);
//                 self.registers.flags.zero = self.registers.a == 0;
//                 self.registers.flags.negative = false;
//                 //fetch
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::B => {
//                 self.registers.b = self.registers.b.wrapping_add(1);
//                 self.registers.flags.zero = self.registers.b == 0;
//                 self.registers.flags.negative = false;
//                 //fetch
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::C => {
//                 self.registers.c = self.registers.c.wrapping_add(1);
//                 self.registers.flags.zero = self.registers.c == 0;
//                 self.registers.flags.negative = false;
//                 //fetch
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::D => {
//                 self.registers.d = self.registers.d.wrapping_add(1);
//                 self.registers.flags.zero = self.registers.d == 0;
//                 self.registers.flags.negative = false;
//                 //fetch
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::E => {
//                 self.registers.e = self.registers.e.wrapping_add(1);
//                 self.registers.flags.zero = self.registers.e == 0;
//                 self.registers.flags.negative = false;
//                 //fetch
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::H => {
//                 self.registers.h = self.registers.h.wrapping_add(1);
//                 self.registers.flags.zero = self.registers.h == 0;
//                 self.registers.flags.negative = false;
//                 //fetch
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::L => {
//                 self.registers.l = self.registers.l.wrapping_add(1);
//                 self.registers.flags.zero = self.registers.l == 0;
//                 self.registers.flags.negative = false;
//                 //fetch
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::BC => {
//                 //write
//                 self.registers
//                     .set_bc(self.registers.get_bc().wrapping_add(1));
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::DE => {
//                 //write
//                 self.registers
//                     .set_de(self.registers.get_de().wrapping_add(1));
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::HL => {
//                 //write
//                 self.registers
//                     .set_hl(self.registers.get_hl().wrapping_add(1));
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::SP => {
//                 //write
//                 self.registers.sp = self.registers.sp.wrapping_add(1);
//                 cycles_used += self.sync();
//             }
//             TargetIncDec::HLPOINTER => {
//                 //read
//                 let address = self.registers.get_hl();
//                 let byte = self.read_byte(address).wrapping_add(1);
//                 cycles_used += self.sync();

//                 //write
//                 self.write_byte(address, byte);
//                 cycles_used += self.sync();
//             }
//         }

//         self.pc = self.pc.wrapping_add(1);
//         cycles_used
//     }

use std::collections::VecDeque;

use crate::cpu::{instructions::TargetIncDec, GameboyChip};

struct Inst {
    target: TargetIncDec,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(target: &TargetIncDec) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            match inst.target {
                TargetIncDec::A => {
                    chip.registers.a = chip.registers.a.wrapping_add(1);
                    chip.registers.flags.zero = chip.registers.a == 0;
                    chip.registers.flags.negative = false;
                }
                TargetIncDec::B => {
                    chip.registers.b = chip.registers.b.wrapping_add(1);
                    chip.registers.flags.zero = chip.registers.b == 0;
                    chip.registers.flags.negative = false;
                }
                TargetIncDec::C => {
                    chip.registers.c = chip.registers.c.wrapping_add(1);
                    chip.registers.flags.zero = chip.registers.c == 0;
                    chip.registers.flags.negative = false;
                }
                TargetIncDec::D => {
                    chip.registers.d = chip.registers.d.wrapping_add(1);
                    chip.registers.flags.zero = chip.registers.d == 0;
                    chip.registers.flags.negative = false;
                }
                TargetIncDec::E => {
                    chip.registers.e = chip.registers.e.wrapping_add(1);
                    chip.registers.flags.zero = chip.registers.e == 0;
                    chip.registers.flags.negative = false;
                }
                TargetIncDec::H => {
                    chip.registers.h = chip.registers.h.wrapping_add(1);
                    chip.registers.flags.zero = chip.registers.h == 0;
                    chip.registers.flags.negative = false;
                }
                TargetIncDec::L => {
                    chip.registers.l = chip.registers.l.wrapping_add(1);
                    chip.registers.flags.zero = chip.registers.l == 0;
                    chip.registers.flags.negative = false;
                }
                TargetIncDec::BC => {
                    //write
                    chip.registers
                        .set_bc(chip.registers.get_bc().wrapping_add(1));
                }
                TargetIncDec::DE => {
                    //write
                    chip.registers
                        .set_de(chip.registers.get_de().wrapping_add(1));
                }
                TargetIncDec::HL => {
                    //write
                    chip.registers
                        .set_hl(chip.registers.get_hl().wrapping_add(1));
                }
                TargetIncDec::SP => {
                    //write
                    chip.registers.sp = chip.registers.sp.wrapping_add(1);
                }
                TargetIncDec::HLPOINTER => {
                    //read
                    let address = chip.registers.get_hl();
                    let byte = chip.read_byte(address).wrapping_add(1);

                    chip.write_byte(address, byte);
                }
            }
        }));

    Box::new(inst)
}

impl Iterator for Inst {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
