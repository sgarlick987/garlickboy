use garlickboy::cpu::instructions::*;
use std::collections::HashSet;

mod tests {
    use super::*;

    #[test]
    fn test_from_byte_no_prefix_nop() {
        assert_eq!(Instruction::from_byte(0x00, false), Instruction::NOP);
    }

    #[test]
    #[should_panic]
    fn test_from_byte_not_prefixed_prefix_byte_panic() {
        Instruction::from_byte(INSTRUCTION_PREFIX_BYTE, false);
    }

    #[test]
    fn test_from_byte_not_prefixed_unassigned_op_bytes_panic() {
        for byte in UNASSIGNED_INSTRUCTION_BYTES {
            let result = std::panic::catch_unwind(|| Instruction::from_byte(byte, false));
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_each_byte_not_prefixed_has_unique_instruction() {
        each_byte_has_unique_instruction(false);
    }

    #[test]
    fn test_each_byte_prefixed_has_unique_instruction() {
        each_byte_has_unique_instruction(true);
    }

    fn each_byte_has_unique_instruction(prefixed: bool) {
        let mut instructions: HashSet<Instruction> = HashSet::from([]);
        for byte in 0x00..0xFF {
            if !prefixed
                && (UNASSIGNED_INSTRUCTION_BYTES.contains(&byte) || INSTRUCTION_PREFIX_BYTE == byte)
            {
                continue;
            }
            let i = Instruction::from_byte(byte, prefixed);
            if i == Instruction::UNIMPLEMENTED {
                continue;
            }
            assert!(!instructions.contains(&i));
            instructions.insert(i);
        }
    }
}
