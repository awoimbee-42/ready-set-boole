use std::fmt::Write;

use super::ex03_boolean_evaluation::{checked_eval_formula, ParsingError};

pub fn generate_truth_table(formula: &str) -> Result<String, ParsingError> {
    let mut output = String::new();
    let mut vars = formula
        .bytes()
        .filter(|token| token.is_ascii_alphabetic())
        .collect::<Vec<_>>();
    vars.sort_unstable();
    vars.dedup();

    for &v in vars.iter() {
        write!(output, "| {} ", v as char).unwrap();
    }
    writeln!(output, "| = |").unwrap();
    for _ in vars.iter() {
        write!(output, "|---").unwrap();
    }
    writeln!(output, "|---|").unwrap();

    for i in 0..(2_u32.pow(vars.len() as u32)) {
        let var_values = format!("{:0width$b}", i, width = vars.len()).into_bytes();
        debug_assert!(var_values.len() == vars.len());
        let mut formula_copy = formula.to_string();
        unsafe {
            for c in formula_copy.as_bytes_mut().iter_mut() {
                for (var, &val) in vars.iter().zip(var_values.iter()) {
                    if *c == *var {
                        *c = val;
                    }
                }
            }
        }
        let result = checked_eval_formula(&formula_copy)?;

        for &c in var_values.iter() {
            write!(output, "| {} ", c as char).unwrap();
        }
        writeln!(output, "| {} |", result as u32).unwrap();
    }
    Ok(output)
}

pub fn print_truth_table(formula: &str) {
    print!(
        "{}",
        generate_truth_table(formula).unwrap_or("(error)".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truth_table() {
        let res = generate_truth_table("AB&C|").unwrap();
        assert_eq!(res, "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 0 |\n| 0 | 1 | 1 | 1 |\n| 1 | 0 | 0 | 0 |\n| 1 | 0 | 1 | 1 |\n| 1 | 1 | 0 | 1 |\n| 1 | 1 | 1 | 1 |\n");
        assert!(generate_truth_table("AB&C|&").is_err());
    }
}
