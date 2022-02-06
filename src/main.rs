mod adder;
mod multiplier;
mod gray_code;
mod boolean_evaluation;
mod truth_table;
mod rewrite_rules;

use adder::adder;

fn main() {
    truth_table::print_truth_table("AB&");
    truth_table::print_truth_table("AB&C|");
}
