mod bool_formula_ast;

mod ex00_adder;
mod ex01_multiplier;
mod ex02_gray_code;
mod ex03_boolean_evaluation;
mod ex04_truth_table;
mod ex05_negation_normal_form;
mod ex06_conjunctive_normal_form;

fn main() {
    // Bitwise operations
    println!("1+2={}", ex00_adder::adder(1, 2));
    println!("3*2={}", ex01_multiplier::multiplier(3, 2));
    println!("gray(4)={}", ex02_gray_code::gray_code(4));

    // Boolean evaluation
    let eval_formula_input = "010&1|&";
    println!(
        "eval({:?})={}",
        eval_formula_input,
        ex03_boolean_evaluation::eval_formula("010&1|&")
    );
    let truth_table_input = "ABC&A|&";
    println!("truth_table({truth_table_input:?}):");
    ex04_truth_table::print_truth_table(truth_table_input);
    let nnf_input = "AB|C&!";
    println!(
        "NNF({:?})={}",
        nnf_input,
        ex05_negation_normal_form::negation_normal_form(nnf_input)
    );
}
