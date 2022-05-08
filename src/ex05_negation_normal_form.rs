use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
struct Node {
    children: Option<[Box<Self>; 2]>,
    neg: bool,
    val: u8,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Some(children) = &self.children {
            children[0].fmt(f)?;
            children[1].fmt(f)?;
        }
        write!(f, "{}", self.val as char)?;
        if self.neg {
            write!(f, "!")?;
        }
        Ok(())
    }
}

struct NodeIterator<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Some(children) = &node.children {
            self.stack.push(&children[0]);
            self.stack.push(&children[1]);
        }
        Some(node)
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = &'a Node;
    type IntoIter = NodeIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        NodeIterator { stack: vec![self] }
    }
}

impl Node {
    fn new(children: Option<[Box<Self>; 2]>, neg: bool, val: u8) -> Self {
        Node { children, neg, val }
    }
}

fn inner_parse(mut s: &[u8]) -> Result<(Box<Node>, &[u8])> {
    let mut val = *s.last().unwrap();
    s = &s[..s.len() - 1];
    let mut neg = false;

    while val == b'!' {
        val = *s
            .last()
            .ok_or_else(|| anyhow!("invalid input (invalid negation)"))?;
        s = &s[..s.len() - 1];
        neg = !neg;
    }

    match val {
        b'&' | b'^' | b'|' | b'>' | b'=' => {
            let right = inner_parse(s)?;
            let left = inner_parse(right.1)?;

            let node = Node::new(Some([left.0, right.0]), neg, val);

            Ok((Box::new(node), left.1))
        }
        b'A'..=b'Z' => Ok((Box::new(Node::new(None, neg, val)), s)),
        _ => Err(anyhow!("invalid input (invalid char `{}`)", val as char)),
    }
}

fn parse(s: &str) -> Result<Node> {
    let res = inner_parse(s.as_bytes())?;
    assert!(res.1.is_empty());
    Ok(*res.0)
}

pub fn nnf(formula: &str) -> Result<String> {
    let mut tree = parse(formula)?;

    recurse_tree_nnf(&mut tree);
    Ok(tree.to_string())
}

pub fn negation_normal_form(formula: &str) -> String {
    nnf(formula).unwrap()
}

fn recurse_tree_nnf(n: &mut Node) {
    rm_exclusive_or(n);
    rm_equivalence(n);
    rm_material_conditions(n);
    rm_negation(n);

    if let Some(children) = &mut n.children {
        recurse_tree_nnf(&mut children[0]);
        recurse_tree_nnf(&mut children[1]);
    }
}

fn rm_material_conditions(n: &mut Node) {
    if n.val == b'>' {
        let children_ref = n.children.as_mut().unwrap();
        n.val = b'|';
        children_ref[0].neg = !children_ref[0].neg;
    }
}

fn rm_equivalence(n: &mut Node) {
    if n.val == b'=' {
        let children_ref = n.children.as_mut().unwrap();
        let children_clone = children_ref.clone();
        let inversed_children = [children_clone[1].clone(), children_clone[0].clone()];
        n.val = b'&';
        children_ref[0] = Box::new(Node::new(Some(children_clone), false, b'>'));
        children_ref[1] = Box::new(Node::new(Some(inversed_children), false, b'>'));
    }
}

fn rm_negation(n: &mut Node) {
    if n.neg {
        if let Some(children_ref) = n.children.as_mut() {
            match n.val {
                b'&' => {
                    children_ref[0].neg = !children_ref[0].neg;
                    children_ref[1].neg = !children_ref[1].neg;
                    n.val = b'|';
                    n.neg = !n.neg;
                }
                b'|' => {
                    children_ref[0].neg = !children_ref[0].neg;
                    children_ref[1].neg = !children_ref[1].neg;
                    n.val = b'&';
                    n.neg = !n.neg;
                }
                _ => {}
            }
        }
    }
}

fn rm_exclusive_or(n: &mut Node) {
    if n.val == b'^' {
        let children_ref = n.children.as_mut().unwrap();
        let a = children_ref[0].clone();
        let b = children_ref[1].clone();
        n.val = b'&';
        children_ref[0] = Box::new(Node::new(Some([a.clone(), b.clone()]), false, b'|'));
        children_ref[1] = Box::new(Node::new(Some([a, b]), true, b'&'));
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
        let parsed_nnf = parse(&nnf);
        for n in parsed_nnf.into_iter() {
            let is_value = n.val.is_ascii_alphanumeric();
            assert!(!n.neg || is_value, "In NNF, only values can be negated");
            assert!(
                is_value || matches!(n.val, b'&' | b'|'),
                "In NNF, only & and | are allowed"
            );
        }
        assert_eq!(
            generate_truth_table(&nnf).unwrap(),
            generate_truth_table(formula).unwrap()
        );
        nnf
    }

    #[test]
    fn parsing() {
        assert_eq!(negation_normal_form("A!!"), "A");
        assert_eq!(negation_normal_form("A!!!"), "A!");
        assert!(nnf("óë&³&!!!").is_err());
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
