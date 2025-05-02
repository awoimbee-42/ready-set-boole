pub fn map(x: u16, y: u16) -> f64 {
    let int = ((x as u32) << 16) | y as u32;
    (int as f64) / (u32::MAX as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(map(0, 0), 0.);
        assert_eq!(map(u16::MAX, u16::MAX), 1.);
    }
}
