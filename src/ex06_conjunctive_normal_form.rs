use crate::bool_formula_ast::{MyError, Node};
use crate::ex04_truth_table::TruthTable;
use crate::ex05_negation_normal_form::rm_negation;

fn cnf(formula: &str) -> Result<Node, MyError> {
    let truth_table = TruthTable::compute(formula)?;
    let mut new_formula = String::new();
    let variables = truth_table.variables();
    let mut i = 0;
    dbg!(truth_table.to_string());
    for (values, result) in truth_table.entries() {
        if result {
            continue;
        }
        for (j, &val) in values.iter().enumerate() {
            new_formula.push(variables[j]);
            if val == b'1' {
                new_formula.push('!');
            }
            if j > 0 {
                new_formula.push('|');
            }
        }
        if i > 0 {
            new_formula.push('&');
        }
        i += 1;
    }
    dbg!(&new_formula);

    let mut new_formula = Node::parse(&new_formula)?;
    new_formula.recursive_edit_operators(&mut |n| {
        rm_negation(n);
    });

    Ok(new_formula)

    // let mut tree = Node::parse(formula)?;
    // tree.recursive_edit_operators(&mut |n| {
    //     if let NodeValue::Operator((op, children)) = &mut n.val {
    //         match (op, children.as_slice()) {
    //             // ab&c& => abc&&
    //             ('&', [Node { val: NodeValue::Operator(('&', _)), .. }, _]) => {
    //                 let tmp = children[0].clone();
    //                 children[0] = children[1].clone();
    //                 children[1] = tmp;
    //             },
    //             // abc&| => ab|ac|b&
    //             ('|', [_, Node { val: NodeValue::Operator(('&', _)), .. }]) => {
    //                 // let new_node = Node::new(n.neg, val)

    //                 // let tmp = children[0].clone();
    //                 // children[0] = children[1].clone();
    //                 // children[1] = tmp;
    //             },
    //             // bc&a| => ab|ac|b&
    //             _ => (),

    //         }
    //     }
    // });

    // unimplemented!();
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    cnf(formula)
        .map(|n| n.to_string())
        .unwrap_or_else(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_examples() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        // assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        // assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }
}
