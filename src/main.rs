mod ex00_adder;
mod ex01_multiplier;
mod ex02_gray_code;
mod ex03_boolean_evaluation;
mod ex04_truth_table;
mod ex05_negation_normal_form;

fn main() {
    println!("1+2={}", ex00_adder::adder(1, 2));
    println!("3*2={}", ex01_multiplier::multiplier(3, 2));
    println!("gray(4)={}", ex02_gray_code::gray_code(4));
    println!(
        "eval(\"010&1|&\")={}",
        ex03_boolean_evaluation::eval_formula("010&1|&")
    );
    println!("truth_table(\"ABC&A|&\"):");
    ex04_truth_table::print_truth_table("ABC&A|&");
}
