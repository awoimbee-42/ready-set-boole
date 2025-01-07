use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("invalid character: '{0}'")]
    InvalidChar(char),
    #[error("invalid operator: '{0}'")]
    InvalidOperator(char),
    // #[error("missing value for operator: {0}")]
    // MissingValue(char),
    // #[error("formula returns multiple values")]
    // TooManyValues,
    #[error("premature end of formula")]
    Eof,
    #[error("value for variable '{0}' is not set")]
    UnsetVariable(char), // #[error("parsing error: {0}")]
                         // ParsingError(String),
}

// impl From<crate::ex03_boolean_evaluation::ParsingError> for MyError {
//     fn from(err: crate::ex03_boolean_evaluation::ParsingError) -> Self {
//         MyError::ParsingError(err.to_string())
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum NodeValue {
    Operator((char, Box<[Node; 2]>)),
    Variable(char),
    Value(bool),
}
impl NodeValue {
    pub fn char(&self) -> char {
        match self {
            Self::Operator((c, _)) => *c,
            Self::Variable(c) => *c,
            Self::Value(b) => {
                if *b {
                    '1'
                } else {
                    '0'
                }
            }
        }
    }

    pub fn new_operator(char: char, children: Box<[Node; 2]>) -> Self {
        Self::Operator((char, children))
    }

    pub fn new_variable(char: char) -> Self {
        Self::Variable(char)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// Whether this value is negated or not
    pub neg: bool,
    pub val: NodeValue,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let NodeValue::Operator((_, children)) = &self.val {
            children[0].fmt(f)?;
            children[1].fmt(f)?;
        }
        write!(f, "{}", self.val.char())?;
        if self.neg {
            write!(f, "!")?;
        }
        Ok(())
    }
}

pub struct NodeIterator<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let NodeValue::Operator((_, children)) = &node.val {
            // children: &Box[Node; 2]
            self.stack.extend(children.iter());
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
    pub fn new(neg: bool, val: NodeValue) -> Self {
        Self { neg, val }
    }

    pub fn evaluate(&self) -> Result<bool, MyError> {
        let result = match &self.val {
            NodeValue::Value(b) => *b,
            NodeValue::Variable(c) => return Err(MyError::UnsetVariable(*c)),
            NodeValue::Operator((op, children)) => {
                let left = children[0].evaluate()?;
                let right = children[1].evaluate()?;
                match op {
                    '&' => left && right,
                    '|' => left || right,
                    '^' => left != right,
                    '>' => !left || right,
                    '=' => left == right,
                    _ => panic!("Invalid operator"),
                }
            }
        };
        Ok(result ^ self.neg)
    }

    pub fn partial_evaluate(&mut self, var: char, value: bool) {
        match &mut self.val {
            NodeValue::Variable(c) => {
                if var == *c {
                    self.val = NodeValue::Value(value);
                }
            }
            NodeValue::Operator((op, children)) => {
                children[0].partial_evaluate(var, value);
                children[1].partial_evaluate(var, value);

                // If both children are now values, evaluate this node
                if let (NodeValue::Value(left), NodeValue::Value(right)) =
                    (&children[0].val, &children[1].val)
                {
                    let result = match op {
                        '&' => *left && *right,
                        '|' => *left || *right,
                        '^' => *left != *right,
                        '>' => !*left || *right,
                        '=' => *left == *right,
                        _ => panic!("Invalid operator"),
                    };
                    self.val = NodeValue::Value(result ^ self.neg);
                    self.neg = false;
                }
            }
            _ => {}
        }
    }

    fn inner_parse(mut s: &[u8]) -> Result<(Node, &[u8]), MyError> {
        let mut val = *s.last().ok_or(MyError::Eof)? as char;
        s = &s[..s.len() - 1];
        let mut neg = false;

        while val == '!' {
            val = *s.last().ok_or(MyError::InvalidOperator('!'))? as char;
            s = &s[..s.len() - 1];
            neg ^= true;
        }

        match val {
            '&' | '^' | '|' | '>' | '=' => {
                let right = Self::inner_parse(s)?;
                let left = Self::inner_parse(right.1)?;

                let node = Node {
                    neg,
                    val: NodeValue::Operator((val, Box::new([left.0, right.0]))),
                };

                Ok((node, left.1))
            }
            'A'..='Z' => Ok((
                Node {
                    neg,
                    val: NodeValue::Variable(val),
                },
                s,
            )),
            '0' | '1' => Ok((
                Node {
                    neg,
                    val: NodeValue::Value(val == '1'),
                },
                s,
            )),
            _ => Err(MyError::InvalidChar(val)),
        }
    }

    pub fn parse(s: &str) -> Result<Node, MyError> {
        let res = Self::inner_parse(s.as_bytes())?;
        debug_assert!(res.1.is_empty());
        Ok(res.0)
    }

    #[cfg(test)]
    fn new_random(variables: &[char]) -> Self {
        let nodekind = rand::random::<usize>() % 3;

        let value = match nodekind {
            0 => NodeValue::Variable(variables[rand::random::<usize>() % variables.len()]),
            1 => NodeValue::Value(rand::random::<bool>()),
            2 => NodeValue::Operator((
                ['&', '|', '^', '>', '='][rand::random::<usize>() % 5],
                Box::new([Self::new_random(variables), Self::new_random(variables)]),
            )),
            _ => unreachable!(),
        };

        Self {
            neg: rand::random::<bool>(),
            val: value,
        }
    }

    pub fn operator_edit<F: FnMut(&mut Self)>(&mut self, f: &mut F) {
        if let NodeValue::Operator(_) = self.val {
            f(self);
        }
        if let NodeValue::Operator((_, children)) = &mut self.val {
            children[0].operator_edit(f);
            children[1].operator_edit(f);
        }
    }

    // pub fn value_edit<F: FnMut(&mut Self)>(&mut self, f: &mut F) {
    //     if let NodeValue::Operator((_, children)) = &mut self.val {
    //         children[0].operator_edit(f);
    //         children[1].operator_edit(f);
    //     } else{
    //         f(self);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Node::parse("AB|!").unwrap(),
            Node::new(
                true,
                NodeValue::new_operator(
                    '|',
                    Box::new([
                        Node::new(false, NodeValue::new_variable('A')),
                        Node::new(false, NodeValue::new_variable('B'))
                    ])
                )
            )
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
    }

    #[test]
    fn partial_evaluation() {
        let raw_formula = "AB>C&!D|A^B=";
        let final_formula = Node::parse("01>1&!0|0^1=").unwrap();
        let mut partialy_evaluated = Node::parse(raw_formula).unwrap();
        partialy_evaluated.partial_evaluate('A', false);
        partialy_evaluated.partial_evaluate('B', true);
        partialy_evaluated.partial_evaluate('C', true);
        partialy_evaluated.partial_evaluate('D', false);
        assert_eq!(partialy_evaluated.evaluate().unwrap(), final_formula.evaluate().unwrap());
    }
}
