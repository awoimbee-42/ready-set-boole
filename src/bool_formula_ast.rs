use std::default;
use std::mem;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("invalid character: '{0}'")]
    InvalidChar(char),
    #[error("invalid operator: '{0}'")]
    InvalidOperator(char),
    #[error("premature end of formula")]
    Eof,
    #[error("value for variable '{0}' is not set")]
    UnsetVariable(char),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Oper {
    Conjunction,
    Disjunction,
    ExclusiveDisjunction,
    MaterialCondition,
    Equivalence,
}

impl Oper {
    pub fn ascii_char(&self) -> char {
        match self {
            Self::Conjunction => '&',
            Self::Disjunction => '|',
            Self::ExclusiveDisjunction => '^',
            Self::MaterialCondition => '>',
            Self::Equivalence => '=',
        }
    }
    pub fn utf8_char(&self) -> char {
        match self {
            Self::Conjunction => '∧',
            Self::Disjunction => '∨',
            Self::ExclusiveDisjunction => '⊕',
            Self::MaterialCondition => '⇒',
            Self::Equivalence => '⇔',
        }
    }

    pub fn from_ascii(char: char) -> Result<Self, MyError> {
        match char {
            '&' => Ok(Self::Conjunction),
            '|' => Ok(Self::Disjunction),
            '^' => Ok(Self::ExclusiveDisjunction),
            '>' => Ok(Self::MaterialCondition),
            '=' => Ok(Self::Equivalence),
            _ => Err(MyError::InvalidChar(char)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Op {
    pub char: Oper,
    pub children: Box<[Node; 2]>,
}

impl Op {
    #[inline]
    pub fn new(char: Oper, children: Box<[Node; 2]>) -> Self {
        Self { char, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Value(bool),
    Variable(char),
    Neg(Box<Node>),
    Operator(Op),
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Node::Operator(Op { children, .. }) => {
                children[0].fmt(f)?;
                children[1].fmt(f)?;
            }
            Node::Neg(child) => {
                child.fmt(f)?;
            }
            Node::Value(_) | Node::Variable(_) => (),
        }

        write!(f, "{}", self.char())?;
        Ok(())
    }
}

impl default::Default for Node {
    fn default() -> Self {
        Self::Value(false)
    }
}

pub struct NodeIterator<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        match &node {
            Node::Operator(Op { children, .. }) => self.stack.extend(children.iter()),
            Node::Neg(child) => self.stack.push(child),
            _ => (),
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
    pub fn char(&self) -> char {
        match self {
            Self::Operator(Op { char: c, .. }) => c.ascii_char(),
            Self::Variable(c) => *c,
            Self::Value(b) => {
                if *b {
                    '1'
                } else {
                    '0'
                }
            }
            Self::Neg(_) => '!',
        }
    }

    pub fn neg(&mut self) {
        match self {
            Node::Neg(child) => *self = mem::take(child),
            _ => *self = Node::Neg(Box::new(mem::take(self))),
        };
    }

    pub fn partial_evaluate(&mut self, var: char, value: bool) {
        match self {
            Node::Variable(c) => {
                if var == *c {
                    *self = Node::Value(value);
                }
            }
            Node::Operator(Op { char: op, children }) => {
                children[0].partial_evaluate(var, value);
                children[1].partial_evaluate(var, value);

                match op {
                    Oper::Conjunction
                        if children.iter().any(|c| matches!(c, Node::Value(false))) =>
                    {
                        *self = Node::Value(false);
                    }
                    Oper::Disjunction
                        if children.iter().any(|c| matches!(c, Node::Value(true))) =>
                    {
                        *self = Node::Value(true);
                    }
                    _ => {
                        if let (Node::Value(left), Node::Value(right)) =
                            (&children[0], &children[1])
                        {
                            let result = match op {
                                Oper::Conjunction => *left && *right,
                                Oper::Disjunction => *left || *right,
                                Oper::ExclusiveDisjunction => *left != *right,
                                Oper::MaterialCondition => !*left || *right,
                                Oper::Equivalence => *left == *right,
                            };
                            *self = Node::Value(result);
                        }
                    }
                }
            }
            Self::Neg(child) => {
                child.partial_evaluate(var, value);
                match &mut **child {
                    Self::Value(val) => *self = Self::Value(!*val),
                    Self::Neg(grand_child) => {
                        *self = mem::take(&mut *grand_child);
                    }
                    Self::Operator(_) | Self::Variable(_) => (),
                }
            }
            Self::Value(_) => (),
        }
    }

    fn inner_parse(s: &mut String) -> Result<Self, MyError> {
        let val = s.pop().ok_or(MyError::Eof)?;

        match val {
            '&' | '^' | '|' | '>' | '=' => {
                let right = Self::inner_parse(s)?;
                let left = Self::inner_parse(s)?;
                let node = Self::Operator(Op {
                    char: Oper::from_ascii(val)?,
                    children: Box::new([left, right]),
                });

                Ok(node)
            }
            'A'..='Z' => Ok(Self::Variable(val)),
            '0' | '1' => Ok(Self::Value(val == '1')),
            '!' => {
                let mut child = Self::inner_parse(s)?;
                child.neg();
                Ok(child)
            }
            _ => Err(MyError::InvalidChar(val)),
        }
    }

    pub fn parse<S: Into<String>>(s: S) -> Result<Self, MyError> {
        let mut s: String = s.into();
        let res = Self::inner_parse(&mut s)?;
        debug_assert!(s.is_empty());
        Ok(res)
    }

    #[cfg(test)]
    pub fn new_random(variables: &[char]) -> Self {
        let nodekind = rand::random::<usize>() % 4;

        match nodekind {
            0 => Node::Variable(variables[rand::random::<usize>() % variables.len()]),
            1 => Node::Value(rand::random::<bool>()),
            2 => Node::Operator(Op {
                char: [
                    Oper::Conjunction,
                    Oper::Disjunction,
                    Oper::ExclusiveDisjunction,
                    Oper::MaterialCondition,
                    Oper::Equivalence,
                ][rand::random::<usize>() % 5],
                children: Box::new([Self::new_random(variables), Self::new_random(variables)]),
            }),
            3 => Node::Neg(Box::new(Self::new_random(variables))),
            _ => unreachable!(),
        }
    }

    pub fn recursive_edit_operators<F: FnMut(&mut Self)>(&mut self, f: &mut F) {
        if let Node::Neg(_) = self {
            f(self);
        }
        if let Node::Neg(child) = self {
            f(child);
        }
        if let Node::Operator(_) = self {
            f(self);
        }
        if let Node::Operator(Op { children, .. }) = self {
            children[0].recursive_edit_operators(f);
            children[1].recursive_edit_operators(f);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Node::parse("AB|!").unwrap(),
            Node::Neg(Box::new(Node::Operator(Op::new(
                Oper::Disjunction,
                Box::new([Node::Variable('A'), Node::Variable('B')])
            ))))
        );
    }

    #[test]
    fn parse_and_dump() {
        let regurgitate = |inp| Node::parse(inp).unwrap().to_string();

        assert_eq!(regurgitate("AB|!"), "AB|!");
        assert_eq!(
            regurgitate("A!B&!C&!D!&!E!&!A>B>!C>!!!F=G!&"),
            "A!B&!C&!D!&!E!&!A>B>!C>!F=G!&"
        );
        assert_eq!(regurgitate("A!!"), "A");
        assert_eq!(regurgitate("A!!!"), "A!");
        assert!(Node::parse("óë&³&!!!").is_err());
        assert_eq!(regurgitate("ABCD&&&"), "ABCD&&&");
    }

    #[test]
    fn partial_evaluation() {
        let raw_formula = "AB>C&!D|A^B=";
        // let final_formula = Node::parse("01>1&!0|0^1=").unwrap();
        let mut partialy_evaluated = Node::parse(raw_formula).unwrap();
        partialy_evaluated.partial_evaluate('A', false);
        partialy_evaluated.partial_evaluate('B', true);
        partialy_evaluated.partial_evaluate('C', true);
        partialy_evaluated.partial_evaluate('D', false);
        assert_eq!(partialy_evaluated, Node::Value(false));
    }

    #[test]
    fn neg() {
        let doit = |formula: &str| {
            let mut tree = Node::parse(formula).unwrap();
            tree.neg();
            tree.to_string()
        };

        assert_eq!(doit("AB|"), "AB|!");
        assert_eq!(doit("A!"), "A");
    }

    #[ignore = "too slow"]
    #[test]
    fn can_parse_huge_formula() {
        loop {
            let mut tree = Node::new_random(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J']);
            tree.to_primitive_connectives_mut();
            let formula = tree.to_string();
            if formula.len() < 100_000 {
                continue;
            }
            Node::parse(formula).unwrap();
            break;
        }
    }
}
