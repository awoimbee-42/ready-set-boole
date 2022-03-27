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
        // println!("is alphanum: {} ({})", val, s);
        return (Box::new(NodeBis::new(None, neg, val)), s);
    } else {
        // println!("parsing children of `{}`", s);

        let right = inner_parse_bis(s);
        let left = inner_parse_bis(right.1);

        let node = NodeBis::new(Some([left.0, right.0]), neg, val);

        return (Box::new(node), left.1);
    }
}

fn parse_bis(s: &str) -> NodeBis {
    let res = inner_parse_bis(s);
    assert!(res.1.is_empty());
    *res.0
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut tree = parse_bis(formula);

    // println!("{}", tree);
    check_tree(&mut tree);
    format!("{}", tree)
}

// check tree
// recursice check for wrong operators ! > =
fn check_tree(mut n: &mut NodeBis) -> &mut NodeBis {
    println!("parsed input: {}", n);
    rm_equivalences(n);
    println!("after rm_equivalences: {}", n);
    rm_material_conditions(n);
    println!("after rm_material_conditions: {}", n);
    // rm_distributivity(n);
    // println!("after rm_distributivity: {}", n);
    rm_negation(n);
    println!("after rm_negation: {}", n);

    n
}

fn rm_material_conditions(n: &mut NodeBis) {
    if n.val == '>' {
        let children_ref = n.children.as_mut().unwrap();
        n.val = '|';
        children_ref[0].neg = !children_ref[0].neg;
    }
    if let Some(children) = &mut n.children {
        rm_material_conditions(&mut children[0]);
        rm_material_conditions(&mut children[1]);
    }
}

fn rm_equivalences(n: &mut NodeBis) {
    if n.val == '=' {
        let children_ref = n.children.as_mut().unwrap();
        let children_clone = children_ref.clone();
        let inversed_children = [children_clone[1].clone(), children_clone[0].clone()];
        n.val = '&';
        children_ref[0] = Box::new(NodeBis::new(Some(children_clone), false, '>'));
        children_ref[1] = Box::new(NodeBis::new(Some(inversed_children), false, '>'));
    }
    if let Some(children) = &mut n.children {
        rm_equivalences(&mut children[0]);
        rm_equivalences(&mut children[1]);
    }
}

fn rm_negation(n: &mut NodeBis) {
    // println!("negate: {}", n);
    if n.neg {
        if let Some(children_ref) = n.children.as_mut() {
            match n.val {
                '&' => {
                    // De Morgan’s laws
                    children_ref[0].neg = !children_ref[0].neg;
                    children_ref[1].neg = !children_ref[1].neg;
                    n.val = '|';
                    n.neg = !n.neg;
                }
                '|' => {
                    // De Morgan’s laws
                    children_ref[0].neg = !children_ref[0].neg;
                    children_ref[1].neg = !children_ref[1].neg;
                    n.val = '&';
                    n.neg = !n.neg;
                }
                _ => {
                    println!("can't propagate negation on: {}", n.val);
                }
            }
        }
    }
    if let Some(children) = &mut n.children {
        rm_negation(&mut children[0]);
        rm_negation(&mut children[1]);
    }
}
fn rm_distributivity(n: &mut NodeBis) {
    if let Some(children_ref) = n.children.as_mut() {
        if n.val == '|' && children_ref[1].val == '&' {
            // A OR (B AND C) => (A OR B) AND (A OR C)
            n.val = '&';
            let b_c = children_ref[1].children.as_mut().unwrap().clone();
            let a_b = [children_ref[0].clone(), b_c[0].clone()];
            let a_c = [children_ref[0].clone(), b_c[1].clone()];

            children_ref[0] = Box::new(NodeBis::new(Some(a_b), false, '|'));
            children_ref[1] = Box::new(NodeBis::new(Some(a_c), false, '|'));
        } else if n.val == '&' && children_ref[1].val == '|' {
            // A AND (B OR C) => (A AND B) OR (A AND C)
            n.val = '|';
            let b_c = children_ref[1].children.as_mut().unwrap().clone();
            let a_b = [children_ref[0].clone(), b_c[0].clone()];
            let a_c = [children_ref[0].clone(), b_c[1].clone()];

            children_ref[0] = Box::new(NodeBis::new(Some(a_b), false, '&'));
            children_ref[1] = Box::new(NodeBis::new(Some(a_c), false, '&'));
        }
    }
    if let Some(children) = &mut n.children {
        rm_distributivity(&mut children[0]);
        rm_distributivity(&mut children[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let t = &mut parse_bis("AB&A!B!&|");
        // rm_distributivity(t);
        // assert_eq!(format!("{}", t), "A!B|B!A|&");
        // -> "AB&A!|AB&B!|&"

        // "A!B|B!A|&" => "AB&A!B!&|" ?

        // parsing
        assert_eq!(negation_normal_form("A!!"), "A");
        assert_eq!(negation_normal_form("A!!!"), "A!");

        // normal form
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");

        // assert_eq!(negation_normal_form("A!B!CD|&!&CD&&!"), "A!B!|C!D!||");
    }
}
