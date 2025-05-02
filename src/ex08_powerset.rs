pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    if set.len() > 63 {
        panic!("Set is too big to compute powerset in one go !")
    }
    let mut res = vec![];

    let limit = 1 << set.len(); // 2.pow(set.len())

    for i in 0..limit {
        res.push(
            (0..set.len())
                .filter(|j| (i & (1 << j)) != 0)
                .map(|j| set[j])
                .collect(),
        );
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(powerset(&[]), vec![vec![]]);
        assert_eq!(powerset(&[1]), vec![vec![], vec![1]]);

        assert_eq!(
            powerset(&[1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ]
        );
    }
}
