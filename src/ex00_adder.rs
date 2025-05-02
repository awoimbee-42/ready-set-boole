pub fn adder(mut a: u32, mut b: u32) -> u32 {
    loop {
        let res = a ^ b;
        let carry = (a & b) << 1;
        a = res;
        b = carry;

        if carry == 0 {
            break;
        }
    }

    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adder() {
        for i in 0u32..10 {
            for j in 0u32..10 {
                assert_eq!(adder(i, j), i + j, "{i} + {j}");
            }
        }
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX.wrapping_add(u32::MAX));
    }
}
