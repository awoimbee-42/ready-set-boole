use crate::bool_formula_ast::{MyError, Node};
use crate::ex05_negation_normal_form::{
    rm_equivalence, rm_exclusive_or, rm_material_conditions, rm_negation,
};

fn cnf(formula: &str) -> Result<Node, MyError> {
    // let mut as_nnf = nnf(formula)?;

    // as_nnf.recursive_edit(&mut |n| {
    //     rm_exclusive_or(n);
    //     rm_equivalence(n);
    //     rm_material_conditions(n);
    //     rm_negation(n);

    // });

    // Ok(as_nnf)
    unimplemented!();
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    cnf(formula)
        .map(|n| n.to_string())
        .unwrap_or_else(|e| e.to_string())
}

fn distibutive_laws(n: &mut Node) {
    // ⋀ == &
    // ⋁ == |
    // 4. (P⋁(Q⋀R))↔(P⋁Q)⋀(P⋁R)
    // 5. (P⋀(Q⋁R))↔(P⋀Q)⋁(P⋀R)
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
