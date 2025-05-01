//! <https://en.wikipedia.org/wiki/Conjunctive_normal_form>
use crate::bool_formula_ast::{MyError, Node, Op, Oper};
use std::mem;

impl Op {
    /// Returns the deep children of chained operators.
    fn op_depth_first_search(&mut self, is_top_level_call: bool, ops: &mut (Vec<Node>, Vec<Node>)) {
        let children_match = self
            .children
            .iter()
            .map(|child| matches!(child, Node::Operator(child_op) if child_op.char == self.char))
            .collect::<Vec<_>>();

        if is_top_level_call && !children_match[0] && !children_match[1] {
            return;
        }
        match &mut self.children[0] {
            Node::Operator(child_op) if child_op.char == self.char => {
                child_op.op_depth_first_search(false, ops);
            }
            child => ops.0.push(mem::take(child)),
        }
        match &mut self.children[1] {
            Node::Operator(child_op) if child_op.char == self.char => {
                child_op.op_depth_first_search(false, ops);
            }
            child => ops.1.push(mem::take(child)),
        }
    }

    /// Helper for `Self::op_depth_first_search`.
    fn op_depth_first_search_chained(&mut self) -> Vec<Node> {
        let mut ops = (Vec::with_capacity(100), Vec::with_capacity(100));
        self.op_depth_first_search(true, &mut ops);
        let (mut left, mut right) = ops;
        left.append(&mut right);
        left
    }

    /// Balance an operator tree from a list of children nodes.
    ///
    /// => moves operators from the left to the right hand of the tree.
    fn build_right_handed_tree_from_node_list(&mut self, nodes: &mut [Node], op: Oper) {
        if nodes.is_empty() {
            return;
        }
        debug_assert!(nodes.len() >= 2, "Invalid call: {nodes:?}");
        let mut i = nodes.len() - 1;
        let mut new_right = mem::take(&mut nodes[i]);
        i -= 1;
        while i != 0 {
            new_right = Node::Operator(Self::new(
                op,
                Box::new([mem::take(&mut nodes[i]), new_right]),
            ));
            i -= 1;
        }
        self.children[1] = new_right;
        self.children[0] = mem::take(&mut nodes[i]);
    }

    /// We only apply the equivalence `| of &` -> `& of |` since it's what we need.
    fn apply_distributive_equivalence(&mut self) {
        debug_assert_eq!(self.char, Oper::Disjunction);
        let new_or = |left, right| Op {
            char: Oper::Disjunction,
            children: Box::new([left, right]),
        };

        match &mut *self.children {
            [
                Node::Operator(Op {
                    char: Oper::Conjunction,
                    children: child_l,
                }),
                Node::Operator(Op {
                    char: Oper::Conjunction,
                    children: child_r,
                }),
            ] => {
                self.char = Oper::Conjunction;

                let mut new_operands = Box::new([
                    new_or(child_l[0].clone(), child_r[0].clone()),
                    new_or(child_l[1].clone(), mem::take(&mut child_r[0])),
                    new_or(mem::take(&mut child_l[0]), child_r[1].clone()),
                    new_or(mem::take(&mut child_l[1]), mem::take(&mut child_r[1])),
                ]);
                for op in new_operands.iter_mut() {
                    op.apply_distributive_equivalence();
                }
                let mut new_operands = new_operands.map(Node::Operator);
                self.build_right_handed_tree_from_node_list(&mut new_operands, Oper::Conjunction);
            }
            [
                child_l,
                Node::Operator(Op {
                    char: Oper::Conjunction,
                    children: nested_children,
                }),
            ] => {
                self.char = Oper::Conjunction;
                let mut new_ops = Box::new([
                    new_or(child_l.clone(), mem::take(&mut nested_children[0])),
                    new_or(mem::take(child_l), mem::take(&mut nested_children[1])),
                ]);
                for op in new_ops.iter_mut() {
                    op.apply_distributive_equivalence();
                }
                self.build_right_handed_tree_from_node_list(
                    &mut new_ops.map(Node::Operator),
                    Oper::Conjunction,
                );
            }
            [
                Node::Operator(Op {
                    char: Oper::Conjunction,
                    children: nested_children,
                }),
                child_r,
            ] => {
                self.char = Oper::Conjunction;
                let mut new_ops = Box::new([
                    new_or(mem::take(&mut nested_children[0]), child_r.clone()),
                    new_or(mem::take(&mut nested_children[1]), mem::take(child_r)),
                ]);
                for op in new_ops.iter_mut() {
                    op.apply_distributive_equivalence();
                }
                self.build_right_handed_tree_from_node_list(
                    &mut new_ops.map(Node::Operator),
                    Oper::Conjunction,
                );
            }
            _ => (),
        }
    }
}

impl Node {
    pub fn is_conjunctive_normal_form(&mut self, accept_conjunctions: bool) -> bool {
        match self {
            Node::Operator(Op { char, children }) => match (char, accept_conjunctions) {
                (Oper::Conjunction, true) => {
                    children[0].is_conjunctive_normal_form(false)
                        && children[1].is_conjunctive_normal_form(true)
                }
                (Oper::Conjunction, false) => false,
                (Oper::Disjunction, _) => {
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

            if op.char == Oper::Disjunction {
                op.apply_distributive_equivalence();
                // Broken since I apply the equivalence on the nested ops ?
                if op.char == Oper::Equivalence {
                    return;
                }
            }
            let mut operands = op.op_depth_first_search_chained();
            op.build_right_handed_tree_from_node_list(&mut operands, op.char);
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
    use super::*;
    use crate::ex04_truth_table::TruthTable;

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
            if formula.len() > 100 {
                // That's a tad big
                continue;
            }

            let res = conjunctive_normal_form(&formula);
            let mut res_tree = Node::parse(&res).unwrap();
            println!("{formula} -> {res}");
            assert!(res_tree.is_conjunctive_normal_form(true),);
            let orig_truth_table = TruthTable::compute(&formula).unwrap();
            let cnf_truth_table =
                TruthTable::compute_with_given_vars(&res, orig_truth_table.variables().to_vec())
                    .unwrap();
            assert_eq!(orig_truth_table.to_string(), cnf_truth_table.to_string());
        }
    }
}
