// DOES NOT WORK

enum NodeValue {
    Op(char),
    Val(char)
}

#[derive(Debug, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: char
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        if let Some(node) = self.left.as_ref() {
            node.fmt(f)?;
        } if let Some(node) = self.right.as_ref() {
            node.fmt(f)?;
        }
        write!(f, "{}", self.val)?;

        Ok(())
    }
}

impl Node {
    fn childless(val: char) -> Self {
        Self {
            left:None,
            right: None,
            val
        }
    }
}

fn parse(s: &str) -> Node {
    let mut n = Node::childless(s.chars().last().unwrap());
    parse_inner(&s[..s.len()-1], &mut n);
    n   
}

fn parse_inner<'a>(s: &'a str, n: &mut Node) -> &'a str {
    let val = n.val;
    //let raw_val = s.chars().last().unwrap();
    if val.is_alphanumeric() || s.is_empty() {
        // set val to raw_val
        return s; // rest of string
    } 
    else if val == '!' {
        let mut newN = Box::new(Node::childless(s.chars().last().unwrap()));
        n.left = Some(newN);
        return parse_inner(&s[..s.len()-1], &mut n.left.as_mut().unwrap());
    }
    else {
        let mut newN = Box::new(Node::childless(s.chars().last().unwrap()));
        n.right = Some(newN);
        let s = parse_inner(&s[..s.len()-1], &mut n.right.as_mut().unwrap());
        let mut newN = Box::new(Node::childless(s.chars().last().unwrap()));
        n.left = Some(newN);
        let s = parse_inner(&s[..s.len()-1], &mut n.left.as_mut().unwrap());
        return s;
    }
}


pub fn negation_normal_form(formula: &str) -> String {
    let mut tree = parse(formula);
    println!("{}", tree);
    check_tree(&mut tree);
    format!("{}",tree)
}

// check tree
// recursice check for wrong operators ! > =
fn check_tree(mut n: &mut Node) -> &mut Node {
    rm_equivalences(n);
    rm_material_conditions(n);
    rm_negation(n);
    // second: negate
    n
}

fn rm_material_conditions(n: &mut Node) {
    if n.val == '>' {
        n.left = Some(Box::new(Node {
            val: '!', 
            left: n.left.clone(), 
            right: None
        }));
        n.val = '|';
    }
    if let Some(n) = n.left.as_mut(){
        rm_material_conditions(n);
    }
    if let Some(n) = n.right.as_mut(){
        rm_material_conditions(n);
    }

}

fn rm_equivalences(n: &mut Node) {
    if n.val == '=' {
        let left_left = n.left.clone().unwrap();
        let left_right = n.right.clone().unwrap().clone();
        let right_left = n.right.clone().unwrap().clone();
        let right_right = n.left.clone().unwrap().clone();
        n.left = Some(Box::new(Node {val: '>', left: Some(left_left), right: Some(left_right)}));
        n.right = Some(Box::new(Node {val: '>', left: Some(right_left), right: Some(right_right)}));
        n.val = '&';
    }
    if let Some(n) = n.left.as_mut(){
        rm_equivalences(n);
    }
    if let Some(n) = n.right.as_mut(){
        rm_equivalences(n);
    }
}

fn rm_negation(n: &mut Node) {
    println!("negate: {}", n);
    if n.val == '!' {
        //let left = n.left.as_mut().unwrap();
        // println!("left_val: {}", left.val);
        match n.left.as_ref().unwrap().val {
            '!' => {
                *n = *n.left.as_ref().unwrap().left.clone().unwrap();
            },
            '&' => {
                println!("befor & transform {:?}", Some(Box::new(Node {val: '!', left: n.left.as_ref().unwrap().right.clone(), right: None})));
                println!("ireal before {:?}", n);
                n.left = Some(Box::new(Node {val: '!', left: n.left.as_ref().unwrap().left.clone(), right: None}));
                n.right = Some(Box::new(Node {val: '!', left: n.left.as_ref().unwrap().right.clone(), right: None}));
                n.val = '|';
                println!("after & transform {:?}", n);
            },
            '|' => {
                n.left = Some(Box::new(Node {val: '!', left: n.left.as_ref().unwrap().left.clone(), right: None}));
                n.right = Some(Box::new(Node {val: '!', left: n.left.as_ref().unwrap().right.clone(), right: None}));
                n.val = '&'; 
            },
            _ => {}
        }
    }
    if let Some(n) = n.left.as_mut(){
        rm_negation(n);
    }
    if let Some(n) = n.right.as_mut(){
        rm_negation(n);
    }
}

/// negate tree when ! 
// fn negate_tree(n: &mut Node) -> Node {
//     if n.val == '!' {
//         return n.left;
//     } 
// }
// AB!>C=!
// fix = 


// fix >

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("A!!"), "A");
        assert_eq!(negation_normal_form("A!!!"), "A!");
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        // assert_eq!(negation_normal_form("A!B!CD|&!&CD&&!"), "A!B!|C!D!||");
    }
}
