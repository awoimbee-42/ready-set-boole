use crate::bool_formula_ast::{MyError, Node};
use crate::ex05_negation_normal_form::nnf;

fn cnf(formula: &str) -> Result<Node, MyError> {
    let as_nnf = nnf(formula)?;

    Ok(as_nnf)
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    cnf(formula)
        .map(|n| n.as_string_operators_last())
        .unwrap_or_else(|e| e.to_string())
}

// fn recurse_tree_cnf(n: &mut Node) {
//     if n.val == b'&' {

//     }

//     rm_exclusive_or(n);
//     rm_equivalence(n);
//     rm_material_conditions(n);
//     rm_negation(n);

//     if let Some(children) = &mut n.children {
//         recurse_tree_nnf(&mut children[0]);
//         recurse_tree_nnf(&mut children[1]);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_examples() {
        // assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        // assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        // assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }
}
