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
        let lower = self.read_byte_pc_lower();
        cycles_used += self.sync();

        //read upper
        let upper = self.read_byte_pc_upper();
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
        let offset = self.read_byte_pc_lower() as i8;
        cycles_used += self.sync();

        //modify PC
        let next_pc = self.pc.wrapping_add(2).wrapping_add(offset as u16);

        self.pc = next_pc;
        cycles_used += self.sync();
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
        let next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //read
        let offset = self.read_byte_pc_lower() as i8;

        if match comparison {
            Comparison::NONZERO => !self.registers.flags.zero,
            Comparison::ZERO => self.registers.flags.zero,
            Comparison::CARRY => self.registers.flags.carry,
            Comparison::NOCARRY => !self.registers.flags.carry,
        } {
            cycles_used += self.sync();
            self.pc = next_pc.wrapping_add(offset as u16);
        } else {
            self.pc = next_pc;
        }

        cycles_used += self.sync();
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
        let (return_address_upper, return_address_lower) = split_bytes(return_address);
        let mut cycles = self.sync();

        //read lower
        let lower = self.read_byte_pc_lower();
        cycles += self.sync();

        //read upper
        let upper = self.read_byte_pc_upper();
        cycles += self.sync();

        //branch
        let pc = merge_bytes(upper, lower);
        cycles += self.sync();

        //write upper
        self._push(return_address_upper);
        cycles += self.sync();

        //write lower
        self._push(return_address_lower);

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
        self.registers.sp = self.registers.sp.wrapping_add(1);
        cycles_used += self.sync();

        //read upper
        let upper = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        cycles_used += self.sync();

        //set pc
        self.pc = merge_bytes(upper, lower);
        cycles_used += self.sync();
        cycles_used
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;
    use mockall::{predicate, Sequence};

    use crate::address::*;

    use super::*;

    const COMPARISONS: [Comparison; 4] = [
        Comparison::NONZERO,
        Comparison::ZERO,
        Comparison::CARRY,
        Comparison::NOCARRY,
    ];

    #[test]
    fn test_jp() {
        const CYCLES: u8 = 16;

        const ADDRESS: u16 = 0x4000;
        const LOWER: u8 = 0x00;
        const UPPER: u8 = 0x40;

        let syncs = CYCLES / 4;
        let mut bus = Box::new(MockBus::new());
        bus.expect_sync().times(syncs as usize).return_const(());

        let mut seq = Sequence::new();
        bus.expect_read_byte()
            .with(predicate::eq(1))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(LOWER);
        bus.expect_read_byte()
            .with(predicate::eq(2))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(UPPER);

        let mut cpu = CPU::new(bus);

        cpu.jp();

        assert_eq!(cpu.pc, ADDRESS);
    }

    #[test]
    fn test_jr() {
        const CYCLES: u8 = 12;
        const LENGTH: u16 = 2;
        const JUMP_OFFSETS: [i8; 2] = [-2, 2];
        const PC: u16 = 2;

        let syncs = CYCLES / 4;

        for jump_offset in JUMP_OFFSETS {
            let mut bus = Box::new(MockBus::new());
            bus.expect_sync().times(syncs as usize).return_const(());

            bus.expect_read_byte()
                .with(predicate::eq(PC + 1))
                .times(1)
                .return_const(jump_offset as u8);

            let mut cpu = CPU::new(bus);
            cpu.pc = PC;

            cpu.jr();

            assert_eq!(
                cpu.pc,
                PC.wrapping_add(LENGTH).wrapping_add(jump_offset as u16)
            );
        }
    }

    #[test]
    fn test_jrf_without_branch() {
        const CYCLES: u8 = 8;
        const LENGTH: u16 = 2;

        let syncs = CYCLES / 4;

        for comparison in COMPARISONS {
            let mut bus = Box::new(MockBus::new());
            bus.expect_sync().times(syncs as usize).return_const(());
            bus.expect_read_byte()
                .with(predicate::eq(1))
                .times(1)
                .return_const(0); //return value doesnt matter since we aren't jumping this test

            let mut cpu = CPU::new(bus);

            match comparison {
                Comparison::ZERO => cpu.registers.flags.zero = false,
                Comparison::NONZERO => cpu.registers.flags.zero = true,
                Comparison::CARRY => cpu.registers.flags.carry = false,
                Comparison::NOCARRY => cpu.registers.flags.carry = true,
            }
            cpu.jrf(&comparison);
            assert_eq!(cpu.pc, LENGTH);
        }
    }

    #[test]
    fn test_jrf_with_branch() {
        const CYCLES: u8 = 12;
        const LENGTH: u16 = 2;

        const JUMP_OFFSETS: [i8; 2] = [-2, 2];
        const PC: u16 = 2;

        let syncs = CYCLES / 4;

        for jump_offset in JUMP_OFFSETS {
            for comparison in COMPARISONS {
                let mut bus = Box::new(MockBus::new());
                bus.expect_sync().times(syncs as usize).return_const(());
                bus.expect_read_byte()
                    .with(predicate::eq(PC.wrapping_add(1)))
                    .times(1)
                    .return_const(jump_offset as u8); //return value doesnt matter since we aren't jumping this test

                let mut cpu = CPU::new(bus);
                cpu.pc = PC;

                match comparison {
                    Comparison::ZERO => cpu.registers.flags.zero = true,
                    Comparison::NONZERO => cpu.registers.flags.zero = false,
                    Comparison::CARRY => cpu.registers.flags.carry = true,
                    Comparison::NOCARRY => cpu.registers.flags.carry = false,
                }
                cpu.jrf(&comparison);

                assert_eq!(
                    cpu.pc,
                    PC.wrapping_add(LENGTH).wrapping_add(jump_offset as u16)
                );
            }
        }
    }

    #[test]
    fn test_call() {
        const CYCLES: u8 = 24;

        const RETURN_ADDRESS_LOWER: u8 = 0x03;
        const RETURN_ADDRESS_UPPER: u8 = 0x00;

        const CALL_ADDRESS: u16 = 0x4000;
        const CALL_ADDRESS_LOWER: u8 = 0x00;
        const CALL_ADDRESS_UPPER: u8 = 0x40;

        let syncs = CYCLES / 4;
        let mut bus = Box::new(MockBus::new());
        bus.expect_sync().times(syncs as usize).return_const(());

        let mut seq = Sequence::new();
        bus.expect_read_byte()
            .with(predicate::eq(1))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(CALL_ADDRESS_LOWER);
        bus.expect_read_byte()
            .with(predicate::eq(2))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(CALL_ADDRESS_UPPER);
        bus.expect_write_byte()
            .with(predicate::eq(1), predicate::eq(RETURN_ADDRESS_UPPER))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        bus.expect_write_byte()
            .with(predicate::eq(0), predicate::eq(RETURN_ADDRESS_LOWER))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        let mut cpu = CPU::new(bus);
        cpu.registers.sp = 2;

        cpu.call();

        assert_eq!(cpu.pc, CALL_ADDRESS);
        assert_eq!(cpu.registers.sp, 0);
    }

    #[test]
    fn test_ret() {
        const CYCLES: u8 = 16;
        const LOWER: u8 = 0x00;
        const UPPER: u8 = 0x10;
        const ADDRESS: u16 = 0x1000;

        let syncs = CYCLES / 4;
        let mut bus = Box::new(MockBus::new());
        bus.expect_sync().times(syncs as usize).return_const(());

        let mut seq = Sequence::new();
        bus.expect_read_byte()
            .with(predicate::eq(0))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(LOWER);
        bus.expect_read_byte()
            .with(predicate::eq(1))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(UPPER);

        let mut cpu = CPU::new(bus);

        cpu.ret();

        assert_eq!(cpu.pc, ADDRESS);
        assert_eq!(cpu.registers.sp, 2);
    }
}
