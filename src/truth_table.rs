use super::boolean_evaluation;

use anyhow::Result;

fn inner_print_truth_table(formula: &str) -> Result<()> {
    let mut vars = formula
        .chars()
        .filter(|token| token.is_ascii_alphabetic())
        .collect::<Vec<_>>();
    vars.sort_unstable();
    vars.dedup();

    for v in vars.iter() {
        print!("| {} ", v);
    }
    println!("| = |");
    for _ in vars.iter() {
        print!("|---");
    }
    println!("|---|");

    for i in 0..(2_u32.pow(vars.len() as u32)) {
        let i_str = format!("{:0width$b}", i, width = vars.len());

        let mut formula_copy = formula.to_string();
        for (index, var) in vars.iter().enumerate() {
            formula_copy =
                formula_copy.replace(*var, &i_str.chars().nth(index).unwrap().to_string());
        }

        let result = boolean_evaluation::eval_formula(&formula_copy);

        for c in i_str.chars() {
            print!("| {} ", c);
        }
        println!("| {} |", result as u32);
    }
    Ok(())
}

pub fn print_truth_table(formula: &str) {
    inner_print_truth_table(formula).unwrap();
}
