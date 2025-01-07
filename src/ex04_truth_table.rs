use core::fmt;
use std::{collections::HashMap, fmt::Write};

use crate::bool_formula_ast::{MyError, Node};


#[derive(Debug, Default, PartialEq)]
pub struct TruthTable {
    variables: Vec<char>,
    entries: Vec<bool>,
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
            entries: results,
        })
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
        for i in 0..(1 << self.variables.len()) {
            let binary_values = format!("{:0width$b}", i, width = self.variables.len());

            // Print variable values
            for c in binary_values.chars() {
                write!(f, "| {} ", c)?;
            }

            // Print result
            writeln!(f, "| {} |", self.entries[i as usize] as u32)?;
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

// fn simplify_formula() {
//     use crate::bool_formula_ast::Node;
//     let node = Node::parse(s)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truth_table() {
        let res = TruthTable::compute("AB&C|").unwrap().to_string();
        assert_eq!(
            res,
            "| A | B | C | = |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
| 0 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 0 |
| 1 | 0 | 1 | 1 |
| 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 |
"
        );
        assert!(TruthTable::compute("AB&C|&").is_err());
    }
}
