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
    fn test_from_byte_no_prefix_prefix_byte_panic() {
        Instruction::from_byte(INSTRUCTION_PREFIX_BYTE, false);
    }

    #[test]
    fn test_from_byte_not_prefix_unassigned_op_bytes_panic() {
        for byte in UNASSIGNED_INSTRUCTION_BYTES {
            let result = std::panic::catch_unwind(|| Instruction::from_byte(byte, false));
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_each_byte_not_prefixed_has_unique_instruction() {
        let mut instructions: HashSet<Instruction> = HashSet::from([]);
        for byte in 0x00..0xFF {
            if UNASSIGNED_INSTRUCTION_BYTES.contains(&byte) || INSTRUCTION_PREFIX_BYTE == byte {
                continue;
            }
            let i = Instruction::from_byte(byte, false);
            if i == Instruction::UNIMPLEMENTED {
                continue;
            }
            assert!(!instructions.contains(&i));
            instructions.insert(i);
        }
    }

    #[test]
    fn test_each_byte_prefixed_has_unique_instruction() {
        let mut instructions: HashSet<Instruction> = HashSet::from([]);
        for byte in 0x00..0xFF {
            let i = Instruction::from_byte(byte, true);
            if i == Instruction::UNIMPLEMENTED {
                continue;
            }
            assert!(!instructions.contains(&i));
            instructions.insert(i);
        }
    }
}
