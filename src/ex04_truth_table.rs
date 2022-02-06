use std::fmt::Write;

use anyhow::Result;

use super::ex03_boolean_evaluation;

fn generate_truth_table(formula: &str) -> Result<String> {
    let mut output = String::new();
    let mut vars = formula
        .chars()
        .filter(|token| token.is_ascii_alphabetic())
        .collect::<Vec<_>>();
    vars.sort_unstable();
    vars.dedup();

    for v in vars.iter() {
        write!(output, "| {} ", v)?;
    }
    writeln!(output, "| = |")?;
    for _ in vars.iter() {
        write!(output, "|---")?;
    }
    writeln!(output, "|---|")?;

    for i in 0..(2_u32.pow(vars.len() as u32)) {
        let i_str = format!("{:0width$b}", i, width = vars.len());
        let mut formula_copy = formula.to_string();

        for (index, var) in vars.iter().enumerate() {
            formula_copy =
                formula_copy.replace(*var, &i_str.chars().nth(index).unwrap().to_string());
        }

        let result = ex03_boolean_evaluation::checked_eval_formula(&formula_copy)?;

        for c in i_str.chars() {
            write!(output, "| {} ", c)?;
        }
        writeln!(output, "| {} |", result as u32)?;
    }
    Ok(output)
}

pub fn print_truth_table(formula: &str) {
    print!("{}", generate_truth_table(formula).unwrap());
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
