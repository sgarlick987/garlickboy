use super::Comparison;
use crate::cpu::CPU;
use crate::utils::*;

pub trait Jump {
    fn jp(&mut self) -> u8;
    fn jr(&mut self) -> u8;
    fn jrf(&mut self, comparison: &Comparison) -> u8;
    fn call(&mut self) -> u8;
    fn ret(&mut self) -> u8;
}

impl Jump for CPU {
    // JP u16 - 0xC3
    // Length: 3 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: control/br
    // Timingwith branch (16t)
    // fetch
    // read	u16:lower
    // read	u16:upper
    // internal	branch decision?
    fn jp(&mut self) -> u8 {
        //fetch
        let mut cycles_used = self.sync();

        //read lower
        let lower = self.read_byte(self.pc + 1);
        cycles_used += self.sync();

        //read upper
        let upper = self.read_byte(self.pc + 2);
        cycles_used += self.sync();

        //branch
        let address = merge_bytes(upper, lower);
        self.pc = address;
        cycles_used += self.sync();

        cycles_used
    }

    // JR i8 - 0x18
    // Length: 2 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: control/br
    // Timingwith branch (12t)
    // fetch
    // read	i8
    // internal	modify PC
    fn jr(&mut self) -> u8 {
        //fetch
        let mut cycles_used = self.sync();

        //read
        let offset = self.read_byte(self.pc + 1) as i8;
        cycles_used += self.sync();

        //modify PC
        let next_pc = self.pc.wrapping_add(2).wrapping_add(offset as u16);

        self.pc = next_pc;
        cycles_used
    }

    // JR Z,i8 - 0x28
    // Length: 2 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: control/br
    // Timing
    // without branch (8t)
    // fetch	fetch
    // with branch (12t)
    // read	read
    // i8	i8
    //     internal
    //     modify PC
    fn jrf(&mut self, comparison: &Comparison) -> u8 {
        // init assuming no branch
        let mut next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let offset = self.read_byte(self.pc + 1) as i8;
        cycles_used += self.sync();

        match comparison {
            Comparison::NONZERO => {
                if !self.registers.flags.zero {
                    // modify pc
                    next_pc = next_pc.wrapping_add(offset as u16);
                    cycles_used += self.sync();
                }
            }
            Comparison::ZERO => {
                if self.registers.flags.zero {
                    // modify pc
                    next_pc = next_pc.wrapping_add(offset as u16);
                    cycles_used += self.sync();
                }
            }
            _ => {
                panic!("{:?} unimplemented JRF Instruction", comparison);
            }
        }

        self.pc = next_pc;
        cycles_used
    }

    // CALL u16 - 0xCD
    // Length: 3 bytes
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: control/br
    // Timingwith branch (24t)
    // fetch
    // read	u16:lower
    // read	u16:upper
    // internal	branch decision?
    // write	PC:upper->(--SP)
    // write	PC:lower->(--SP)
    fn call(&mut self) -> u8 {
        //fetch
        let return_address = self.pc.wrapping_add(3);
        let bytes = split_bytes(return_address);
        let mut cycles = self.sync();

        //read lower
        let lower = self.read_byte(self.pc + 1);
        cycles += self.sync();

        //read upper
        let upper = self.read_byte(self.pc + 2);
        cycles += self.sync();

        //branch
        let pc = merge_bytes(upper, lower);
        cycles += self.sync();

        //write upper
        self.registers.sp -= 1;
        self.write_byte(self.registers.sp, bytes[0]);
        cycles += self.sync();

        //write lower
        self.registers.sp -= 1;
        self.write_byte(self.registers.sp, bytes[1]);

        self.pc = pc;
        cycles += self.sync();
        cycles
    }

    // RET - 0xC9
    // Length: 1 byte
    // FlagsZero	unmodified
    // Negative	unmodified
    // Half Carry	unmodified
    // Carry	unmodified
    // Group: control/br
    // Timingwith branch (16t)
    // fetch
    // read	(SP++)->lower
    // read	(SP++)->upper
    // internal	set PC?
    fn ret(&mut self) -> u8 {
        //fetch
        let mut cycles_used = self.sync();

        //read lower
        let lower = self.read_byte(self.registers.sp);
        self.registers.sp += 1;
        cycles_used += self.sync();

        //read upper
        let upper = self.read_byte(self.registers.sp);
        self.registers.sp += 1;
        cycles_used += self.sync();

        //set pc
        self.pc = merge_bytes(upper, lower);
        cycles_used += self.sync();
        cycles_used
    }
}
