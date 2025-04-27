use crate::ex04_truth_table::TruthTable;

fn sat(formula: &str) -> bool {
    TruthTable::compute(formula)
        .unwrap()
        .entries()
        .any(|(_, r)| r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }
}
