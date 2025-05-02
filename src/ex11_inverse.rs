pub fn reverse_map(n: f64) -> (u16, u16) {
    let int = (n * (u32::MAX as f64)) as u32;
    ((int >> 16) as u16, (int & (u16::MAX as u32)) as u16)
}

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;
    use crate::ex10_curve::map;

    #[test]
    fn default() {
        assert_eq!(reverse_map(0.), (0, 0));
        assert_eq!(reverse_map(1.), (u16::MAX, u16::MAX));
    }

    #[test]
    fn round_trip() {
        for _ in 0..100 {
            let vals = (random(), random());
            assert_eq!(reverse_map(map(vals.0, vals.1)), vals);
        }
    }
}
