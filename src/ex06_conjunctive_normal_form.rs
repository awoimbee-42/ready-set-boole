//! <https://en.wikipedia.org/wiki/Conjunctive_normal_form>
use crate::bool_formula_ast::{MyError, Node, Op};
use std::mem;

impl Op {
    /// Returns the deep children of chained operators.
    fn op_depth_first_search(&mut self, is_top_level_call: bool) -> (Vec<Node>, Vec<Node>) {
        let mut left_ops = vec![];
        let mut right_ops = vec![];

        let children_match = self
            .children
            .iter()
            .map(|child| matches!(child, Node::Operator(child_op) if child_op.char == self.char))
            .collect::<Vec<_>>();
        if is_top_level_call && !children_match[0] && !children_match[1] {
            return (left_ops, right_ops);
        }
        for (i, child) in self.children.iter_mut().enumerate() {
            match child {
                Node::Operator(child_op) if child_op.char == self.char => {
                    let mut vecs = child_op.op_depth_first_search(false);
                    left_ops.append(&mut vecs.0);
                    right_ops.append(&mut vecs.1);
                }
                _ => {
                    if i == 0 {
                        left_ops.push(mem::take(child));
                    } else {
                        right_ops.push(mem::take(child));
                    }
                }
            }
        }

        (left_ops, right_ops)
    }

    /// Helper for `Self::op_depth_first_search`.
    fn op_depth_first_search_chained(&mut self) -> Vec<Node> {
        let operands = self.op_depth_first_search(true);
        operands.0.into_iter().chain(operands.1).collect::<Vec<_>>()
    }

    /// Balance an operator tree from a list of children nodes.
    ///
    /// => moves operator from the left to the right hand of the tree.
    fn build_right_handed_tree_from_node_list(&mut self, mut nodes: Vec<Node>, op: char) {
        if nodes.is_empty() {
            return;
        }
        debug_assert!(nodes.len() >= 2, "Invalid call: {nodes:?}");
        let mut new_right = nodes.pop().unwrap();
        while nodes.len() > 1 {
            new_right = Node::Operator(Self::new(op, Box::new([nodes.pop().unwrap(), new_right])));
        }
        self.children[1] = new_right;
        self.children[0] = nodes.pop().unwrap();
    }

    /// This function applies the equivalence over nested nodes all at once.
    ///
    /// We only apply the equivalence `| of &` -> `& of |` since it's what we need.
    fn apply_distributive_equivalence(&mut self) {
        debug_assert_eq!(self.char, '|');
        self.char = '&';
        let (nodes_l, nodes_r) = self.op_depth_first_search(true);
        if nodes_l.is_empty() && nodes_r.is_empty() {
            self.char = '|';
            return;
        }
        let mut new_operands = vec![];
        for l in nodes_l.into_iter() {
            for r in nodes_r.iter() {
                new_operands.push(Node::Operator(Op::new(
                    '|',
                    Box::new([l.clone(), r.clone()]),
                )));
            }
        }
        self.build_right_handed_tree_from_node_list(new_operands, '&');
    }
}

impl Node {
    pub fn is_conjunctive_normal_form(&mut self, accept_conjunctions: bool) -> bool {
        match self {
            Node::Operator(Op { char, children }) => match (char, accept_conjunctions) {
                ('&', true) => {
                    children[0].is_conjunctive_normal_form(false)
                        && children[1].is_conjunctive_normal_form(true)
                }
                ('&', false) => false,
                ('|', _) => {
                    children[0].is_conjunctive_normal_form(false)
                        && children[1].is_conjunctive_normal_form(false)
                }
                _ => false,
            },
            Node::Variable(_) | Node::Value(_) | Node::Neg(_) => true,
        }
    }

    /// `self` MUST be in negation normal form
    fn to_conjunctive_normal_form_mut(&mut self) {
        if let Node::Operator(op) = self {
            op.children[0].to_conjunctive_normal_form_mut();
            op.children[1].to_conjunctive_normal_form_mut();

            if op.char == '|' {
                op.apply_distributive_equivalence();
                if op.char == '&' {
                    return;
                }
            }
            let operands = op.op_depth_first_search_chained();
            op.build_right_handed_tree_from_node_list(operands, op.char);
        }
    }
}

fn cnf(formula: &str) -> Result<Node, MyError> {
    let mut tree = Node::parse(formula)?;
    tree.to_primitive_connectives_mut();
    tree.to_negation_normal_form_mut();
    tree.to_conjunctive_normal_form_mut();
    Ok(tree)
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    cnf(formula)
        .map(|n| n.to_string())
        .unwrap_or_else(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::ex04_truth_table::TruthTable;

    use super::*;

    #[test]
    fn subject_examples() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }

    #[test]
    fn smoke_test_random() {
        for _ in 0..100 {
            let mut tree = Node::new_random(&['A', 'B', 'C', 'D', 'E']);
            tree.to_primitive_connectives_mut();
            tree.to_negation_normal_form_mut();
            let formula = tree.to_string();

            let res = conjunctive_normal_form(&formula);
            let mut res_tree = Node::parse(&res).unwrap();
            println!("{formula} -> {res}");
            assert!(res_tree.is_conjunctive_normal_form(true),);
            let orig_truth_table = TruthTable::compute(&formula).unwrap().to_string();
            let cnf_truth_table = TruthTable::compute(&res).unwrap().to_string();
            assert_eq!(orig_truth_table, cnf_truth_table)
        }
    }
}
