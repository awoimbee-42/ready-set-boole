#[derive(Debug, Clone)]
struct NodeBis {
    children: Option<[Box<Self>; 2]>,
    neg: bool,
    val: char,
}

impl std::fmt::Display for NodeBis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Some(children) = &self.children {
            children[0].fmt(f)?;
            children[1].fmt(f)?;
        }
        write!(f, "{}", self.val)?;
        if self.neg {
            write!(f, "!")?;
        }
        Ok(())
    }
}

struct NodeIterator<'a> {
    stack: Vec<&'a NodeBis>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a NodeBis;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Some(children) = &node.children {
            self.stack.push(&children[0]);
            self.stack.push(&children[1]);
        }
        Some(node)
    }
}

impl<'a> IntoIterator for &'a NodeBis {
    type Item = &'a NodeBis;
    type IntoIter = NodeIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        NodeIterator { stack: vec![self] }
    }
}

impl NodeBis {
    fn new(children: Option<[Box<Self>; 2]>, neg: bool, val: char) -> Self {
        NodeBis { children, neg, val }
    }
}

fn inner_parse_bis(mut s: &str) -> (Box<NodeBis>, &str) {
    let mut val = *s.as_bytes().last().unwrap() as char;
    s = &s[..s.len() - 1];
    let mut neg = false;

    while val == '!' {
        val = *s.as_bytes().last().unwrap() as char;
        s = &s[..s.len() - 1];
        neg = !neg;
    }

    if val.is_alphanumeric() {
        (Box::new(NodeBis::new(None, neg, val)), s)
    } else {
        let right = inner_parse_bis(s);
        let left = inner_parse_bis(right.1);

        let node = NodeBis::new(Some([left.0, right.0]), neg, val);

        (Box::new(node), left.1)
    }
}

fn parse_bis(s: &str) -> NodeBis {
    let res = inner_parse_bis(s);
    assert!(res.1.is_empty());
    *res.0
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut tree = parse_bis(formula);

    recurse_tree_nnf(&mut tree);
    tree.to_string()
}

fn recurse_tree_nnf(n: &mut NodeBis) {
    rm_equivalence(n);
    rm_material_conditions(n);
    rm_negation(n);

    if let Some(children) = &mut n.children {
        recurse_tree_nnf(&mut children[0]);
        recurse_tree_nnf(&mut children[1]);
    }
}

fn rm_material_conditions(n: &mut NodeBis) {
    if n.val == '>' {
        let children_ref = n.children.as_mut().unwrap();
        n.val = '|';
        children_ref[0].neg = !children_ref[0].neg;
    }
}

fn rm_equivalence(n: &mut NodeBis) {
    if n.val == '=' {
        let children_ref = n.children.as_mut().unwrap();
        let children_clone = children_ref.clone();
        let inversed_children = [children_clone[1].clone(), children_clone[0].clone()];
        n.val = '&';
        children_ref[0] = Box::new(NodeBis::new(Some(children_clone), false, '>'));
        children_ref[1] = Box::new(NodeBis::new(Some(inversed_children), false, '>'));
    }
}

fn rm_negation(n: &mut NodeBis) {
    if n.neg {
        if let Some(children_ref) = n.children.as_mut() {
            match n.val {
                '&' => {
                    children_ref[0].neg = !children_ref[0].neg;
                    children_ref[1].neg = !children_ref[1].neg;
                    n.val = '|';
                    n.neg = !n.neg;
                }
                '|' => {
                    children_ref[0].neg = !children_ref[0].neg;
                    children_ref[1].neg = !children_ref[1].neg;
                    n.val = '&';
                    n.neg = !n.neg;
                }
                _ => {}
            }
        }
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
        let parsed_nnf = parse_bis(&nnf);
        for n in parsed_nnf.into_iter() {
            let is_value = n.val.is_alphanumeric();
            assert!(!n.neg || is_value, "In NNF, only values can be negated");
            assert!(
                is_value || matches!(n.val, '&' | '|'),
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
        assert_eq!("AB^", "AB|A!B!|&");
        assert_eq!("A!B^", "A!B|AB!|&");
        assert_eq!("AB!^", "AB!|A!B|&");
        assert_eq!("A!B!^", "A!B!|AB|&");
    }

    #[test]
    fn smoke_test() {
        assert_eq!(assert_correct_nnf("AB|C&!"), "A!B!&C!|");
        assert_correct_nnf("A!B&!C&!D!&!E!&!");
        assert_correct_nnf("A!B&!C&!D!&!E!&!A>B>!C>!!!F=G!&");
        assert_correct_nnf("AB=C=A=E=A=A=A=A=D=B=B=B=");
        // assert_correct_nnf("AB=C=A=E=A=A=A=A=D=B=B=B=C=C=C=D=D=D=E=E=E=");
    }
}
