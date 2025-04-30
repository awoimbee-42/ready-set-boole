use crate::bool_formula_ast::{MyError, Node, Op, Oper};
use std::mem;

impl Node {
    pub fn to_primitive_connectives_mut(&mut self) {
        match self {
            Node::Neg(child) => {
                child.to_primitive_connectives_mut();
            }
            Node::Operator(Op { char: op, children }) => {
                match op {
                    Oper::ExclusiveDisjunction => {
                        // rm exclusive disjunction
                        *op = Oper::Conjunction;
                        *children = Box::new([
                            Node::Operator(Op {
                                char: Oper::Disjunction,
                                children: children.clone(),
                            }),
                            Node::Neg(Box::new(Node::Operator(Op {
                                char: Oper::Conjunction,
                                children: children.clone(),
                            }))),
                        ]);
                    }
                    Oper::Equivalence => {
                        // rm equivalence
                        let mut children_rev = children.clone();
                        children_rev.reverse();
                        *op = Oper::Conjunction;
                        *children = Box::new([
                            Node::Operator(Op {
                                char: Oper::MaterialCondition,
                                children: children.clone(),
                            }),
                            Node::Operator(Op {
                                char: Oper::MaterialCondition,
                                children: children_rev,
                            }),
                        ]);
                    }
                    Oper::MaterialCondition => {
                        // rm material condition
                        *op = Oper::Disjunction;
                        *children = Box::new([
                            Node::Neg(Box::new(children[0].clone())),
                            children[1].clone(),
                        ]);
                    }
                    Oper::Conjunction | Oper::Disjunction => (),
                }
                children[0].to_primitive_connectives_mut();
                children[1].to_primitive_connectives_mut();
            }
            Node::Value(_) | Node::Variable(_) => (),
        }
    }

    /// `self` MUST be in primitive connectives
    ///
    /// Using De Morgan's equivalences
    pub fn to_negation_normal_form_mut(&mut self) {
        if let Node::Neg(child) = self {
            if let Node::Operator(
                op @ Op {
                    char: Oper::Conjunction | Oper::Disjunction,
                    ..
                },
            ) = &mut **child
            {
                for gc in op.children.iter_mut() {
                    gc.neg();
                }
                if op.char == Oper::Conjunction {
                    op.char = Oper::Disjunction;
                } else {
                    op.char = Oper::Conjunction
                }
                *self = mem::take(child);
            }
        }
        match self {
            Self::Neg(child) => {
                child.to_negation_normal_form_mut();
            }
            Node::Operator(Op { children, .. }) => {
                children[0].to_negation_normal_form_mut();
                children[1].to_negation_normal_form_mut();
            }
            Node::Value(_) | Node::Variable(_) => (),
        }
    }
}

/// Concrete definition of `negation_normal_form` with error handling.
///
/// NNF: '!' is only applied to variables and the only other allowed operators are '&' and '|'.
pub fn nnf(formula: &str) -> Result<Node, MyError> {
    let mut tree = Node::parse(formula)?;
    tree.to_primitive_connectives_mut();
    tree.to_negation_normal_form_mut();

    Ok(tree)
}

pub fn negation_normal_form(formula: &str) -> String {
    nnf(formula)
        .map(|n| n.to_string())
        .unwrap_or_else(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ex04_truth_table::TruthTable;

    /// returns the NNF of `formula`
    ///
    /// This tests that the output is indeed NNF as per https://en.wikipedia.org/wiki/Negation_normal_form
    /// And that the truth table of the resulting formula matches
    fn assert_correct_nnf(formula: &str) -> String {
        let nnf = negation_normal_form(formula);
        println!("{formula} -> {nnf}");
        let parsed_nnf = Node::parse(&nnf);
        for n in parsed_nnf.into_iter() {
            match n {
                Node::Operator(Op { char: op, .. }) => {
                    assert!(
                        matches!(op, Oper::Conjunction | Oper::Disjunction),
                        "In NNF, only & and | are allowed"
                    );
                }
                Node::Variable(v) => {
                    assert!(v.is_ascii_uppercase(), "Invalid variable: {v}");
                }
                Node::Value(_) => (),
                Node::Neg(ref child) => {
                    assert!(
                        matches!(&**child, Node::Value(_) | Node::Variable(_)),
                        "In NNF, only values can be negated"
                    );
                }
            }
        }
        assert_eq!(
            TruthTable::compute(&nnf).unwrap().to_string(),
            TruthTable::compute(formula).unwrap().to_string(),
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

    #[test]
    fn smoke_test_random() {
        for _ in 0..100 {
            let tree = Node::new_random(&['A', 'B', 'C', 'D', 'E']);
            let formula = tree.to_string();
            assert_correct_nnf(&formula);
        }
    }
}
