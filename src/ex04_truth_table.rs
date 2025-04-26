use core::fmt;
use std::fmt::Write;

use crate::bool_formula_ast::{MyError, Node};

#[derive(Debug, Default, PartialEq)]
pub struct TruthTable {
    variables: Vec<char>,
    results: Vec<bool>,
}

fn recursive_truth_table_results(formula: &Node, vars: &[char]) -> Result<Vec<bool>, MyError> {
    let mut result = vec![];
    let var = vars[0];
    for val in [false, true] {
        let mut f = formula.clone();
        f.partial_evaluate(var, val);
        if vars.len() == 1 {
            result.push(f.evaluate()?);
        } else {
            result.append(&mut recursive_truth_table_results(&f, &vars[1..])?);
        }
    }
    Ok(result)
}

/// Iterator over truth table entries in the form:
/// `([b'0', b'1', ...], true)`
pub struct TruthTableEntriesIterator<'a> {
    truth_table: &'a TruthTable,
    i: usize,
}

impl Iterator for TruthTableEntriesIterator<'_> {
    type Item = (Vec<u8>, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let result = *self.truth_table.results.get(self.i)?;
        let values = format!(
            "{:0width$b}",
            self.i,
            width = self.truth_table.variables.len()
        )
        .into_bytes();
        self.i += 1;

        Some((values, result))
    }
}

impl TruthTable {
    pub fn compute(formula: &str) -> Result<Self, MyError> {
        let mut vars = formula
            .chars()
            .filter(|token| token.is_ascii_alphabetic())
            .collect::<Vec<_>>();
        vars.sort();
        vars.dedup();

        let formula = Node::parse(formula)?;
        let results = recursive_truth_table_results(&formula, &vars)?;

        Ok(Self {
            variables: vars,
            results,
        })
    }

    pub fn variables(&self) -> &[char] {
        &self.variables
    }

    pub fn entries(&self) -> TruthTableEntriesIterator {
        TruthTableEntriesIterator {
            i: 0,
            truth_table: self,
        }
    }
}

impl fmt::Display for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print header
        for &v in self.variables.iter() {
            write!(f, "| {} ", v)?;
        }
        writeln!(f, "| = |")?;

        // Print separator
        for _ in self.variables.iter() {
            write!(f, "|---")?;
        }
        writeln!(f, "|---|")?;

        // Print rows
        for (values, result) in self.entries() {
            for c in values {
                write!(f, "| {} ", c as char)?;
            }
            writeln!(f, "| {} |", ((result as u8) + b'0') as char)?;
        }

        Ok(())
    }
}

pub fn print_truth_table(formula: &str) {
    match TruthTable::compute(formula) {
        Ok(tt) => print!("{}", tt),
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truth_table() {
        let res = TruthTable::compute("AB&C|").unwrap().to_string();
        assert_eq!(
            res,
            "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 0 |\n| 0 | 1 | 1 | 1 |\n| 1 | 0 | 0 | 0 |\n| 1 | 0 | 1 | 1 |\n| 1 | 1 | 0 | 1 |\n| 1 | 1 | 1 | 1 |\n"
        );
        assert!(TruthTable::compute("AB&C|&").is_err());
    }
}
