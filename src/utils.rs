pub fn merge_bytes(upper: u8, lower: u8) -> u16 {
    (upper as u16) << 8 | lower as u16
}

pub fn split_bytes(bytes: u16) -> (u8, u8) {
    (((bytes & 0xFF00) >> 8) as u8, (bytes & 0x00FF) as u8)
}

pub fn add_bytes_half_carry(a: u8, b: u8) -> bool {
    (a & 0x0F) + (b & 0x0F) > 0x0F
}

pub fn sub_bytes_half_carry(a: u8, b: u8) -> bool {
    (a & 0x0F) < (b & 0x0F)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_bytes() {
        let bytes = 0x0809u16;

        let (upper, lower) = split_bytes(bytes);

        assert_eq!(upper, 0x08);
        assert_eq!(lower, 0x09);
    }

    #[test]
    fn test_merge_bytes() {
        let upper = 0x08;
        let lower = 0x09;

        let bytes = merge_bytes(upper, lower);

        assert_eq!(bytes, 0x0809);
    }

    #[test]
    fn test_add_bytes_half_carry_carried() {
        let upper = 0x0F;
        let lower = 0x0F;

        let half_carry = add_bytes_half_carry(upper, lower);

        assert!(half_carry);
    }

    #[test]
    fn test_add_bytes_half_carry_not_carried() {
        let a = 0x07;
        let b = 0x07;

        let half_carry = add_bytes_half_carry(a, b);

        assert!(!half_carry);
    }

    #[test]
    fn test_sub_bytes_half_carry_carried() {
        let a = 0x0E;
        let b = 0x0F;

        let half_carry = sub_bytes_half_carry(a, b);

        assert!(half_carry);
    }
    #[test]
    fn test_sub_bytes_half_carry_not_carried() {
        let a = 0x0F;
        let b = 0x0E;

        let half_carry = sub_bytes_half_carry(a, b);

        assert!(!half_carry);
    }
}
