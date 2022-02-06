use num::{Integer, Unsigned};
use std::ops::{BitAnd, BitXor, Shl};

pub fn adder<
    T: Unsigned + Integer + BitXor<Output = T> + BitAnd<Output = T> + Copy + Shl<Output = T>,
>(
    a: T,
    b: T,
) -> T {
    let res = a ^ b;
    let carry = (a & b) << T::one();
    if carry == T::zero() {
        res
    } else {
        adder(res, carry)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adder() {
        for i in 0u32..10 {
            for j in 0u32..10 {
                assert_eq!(adder(i, j), i + j);
            }
        }
    }
}
