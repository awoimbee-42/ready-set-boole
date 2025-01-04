use crate::bool_formula_ast::{MyError, Node, NodeValue};

/// Concrete definition of `negation_normal_form` with error handling.
pub fn nnf(formula: &str) -> Result<Node, MyError> {
    let mut tree = Node::parse(formula)?;

    recurse_tree_nnf(&mut tree);
    Ok(tree)
}

pub fn negation_normal_form(formula: &str) -> String {
    nnf(formula)
        .map(|n| n.to_string())
        .unwrap_or_else(|e| e.to_string())
}

fn recurse_tree_nnf(n: &mut Node) {
    rm_exclusive_or(n);
    rm_equivalence(n);
    rm_material_conditions(n);
    rm_negation(n);

    if let NodeValue::Operator((_, children)) = &mut n.val {
        recurse_tree_nnf(&mut children[0]);
        recurse_tree_nnf(&mut children[1]);
    }
}

fn rm_material_conditions(n: &mut Node) {
    if let NodeValue::Operator((op @ '>', children)) = &mut n.val {
        *op = '|';
        children[0].neg ^= true;
    }
}

fn rm_equivalence(n: &mut Node) {
    if let NodeValue::Operator((op @ '=', children)) = &mut n.val {
        *op = '&';
        let mut children_bak = children.clone();
        children[0] = Node::new(false, NodeValue::new_operator('>', children_bak.clone()));
        children_bak.reverse();
        children[1] = Node::new(false, NodeValue::new_operator('>', children_bak));
    }
}

fn rm_negation(n: &mut Node) {
    if n.neg {
        match &mut n.val {
            NodeValue::Operator((op @ '&', children)) => {
                children[0].neg ^= true;
                children[1].neg ^= true;
                n.neg ^= true;
                *op = '|';
            }
            NodeValue::Operator((op @ '|', children)) => {
                children[0].neg ^= true;
                children[1].neg ^= true;
                n.neg ^= true;
                *op = '&';
            }
            _ => (),
        }
    }
}

fn rm_exclusive_or(n: &mut Node) {
    if let NodeValue::Operator((op @ '^', children)) = &mut n.val {
        let children_bak = children.clone();
        *op = '&';
        children[0] = Node::new(false, NodeValue::new_operator('|', children_bak.clone()));
        children[1] = Node::new(true, NodeValue::new_operator('&', children_bak));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ex04_truth_table::generate_truth_table;

    /// returns the NNF of `formula`
    ///
    /// This tests that the output is indeed NNF as per https://en.wikipedia.org/wiki/Negation_normal_form
    /// And that the truth table of the resulting formula matches
    fn assert_correct_nnf(formula: &str) -> String {
        let nnf = negation_normal_form(formula);
        let parsed_nnf = Node::parse(&nnf);
        for n in parsed_nnf.into_iter() {
            match &n.val {
                NodeValue::Operator((op, _)) => {
                    assert!(!n.neg, "In NNF, only values can be negated");
                    assert!(matches!(op, '&' | '|'), "In NNF, only & and | are allowed");
                }
                NodeValue::Variable(v) => {
                    assert!(v.is_ascii_uppercase(), "Invalid variable: {v}");
                }
            }
        }
        assert_eq!(
            generate_truth_table(&nnf).unwrap(),
            generate_truth_table(formula).unwrap()
        );
        nnf
    }

    #[test]
    fn de_morgans_laws() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
    }

    #[test]
    fn material_conditions() {
        assert_eq!(negation_normal_form("AB>"), "A!B|");
    }

    #[test]
    fn equivalence() {
        assert_eq!(assert_correct_nnf("AB="), "A!B|B!A|&");
    }

    #[test]
    fn exclusive_or() {
        assert_eq!(negation_normal_form("AB^"), "AB|A!B!|&");
        assert_eq!(negation_normal_form("A!B^"), "A!B|AB!|&");
        assert_eq!(negation_normal_form("AB!^"), "AB!|A!B|&");
        assert_eq!(negation_normal_form("A!B!^"), "A!B!|AB|&");
    }

    #[test]
    fn smoke_test() {
        assert_eq!(assert_correct_nnf("AB|C&!"), "A!B!&C!|");
        assert_correct_nnf("A!B&!C&!D!&!E!&!");
        assert_correct_nnf("A!B&!C&!D!&!E!&!A>B>!C>!!!F=G!&");
        assert_correct_nnf("AB=C=A=E=A=A=A=A=D=B=B=B=A^B&!C>");
    }
}
