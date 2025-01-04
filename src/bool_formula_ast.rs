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
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeValue {
    Operator((char, Box<[Node; 2]>)),
    Variable(char),
}
impl NodeValue {
    pub fn char(&self) -> char {
        match self {
            Self::Operator((c, _)) => *c,
            Self::Variable(c) => *c,
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

//
// Impl for ex06
//
impl Node {
    fn as_vars_and_operators(&self) -> (String, String) {
        let mut vars = String::new();
        let mut operators = String::new();
        match &self.val {
            NodeValue::Operator((op, children)) => {
                let vars_op = children.clone().map(|c| c.as_vars_and_operators());
                vars.push_str(&vars_op[0].0);
                vars.push_str(&vars_op[1].0);
                operators.push_str(&vars_op[0].1);
                operators.push_str(&vars_op[1].1);

                operators.push(*op);
            }
            NodeValue::Variable(v) => vars.push(*v),
        };
        if self.neg {
            operators.push('!');
        }
        (vars, operators)
    }

    pub fn as_string_operators_last(&self) -> String {
        let mut out = String::new();

        if let NodeValue::Operator((_, children)) = &self.val {
            let vars_op = children.clone().map(|c| c.as_vars_and_operators());
            out.push_str(&vars_op[0].0);
            out.push_str(&vars_op[1].0);
            out.push_str(&vars_op[0].1);
            out.push_str(&vars_op[1].0);
        }
        out.push(self.val.char());
        if self.neg {
            out.push('!');
        }
        out
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
    pub fn new(neg: bool, val: NodeValue) -> Self {
        Self { neg, val }
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
            _ => Err(MyError::InvalidChar(val)),
        }
    }

    pub fn parse(s: &str) -> Result<Node, MyError> {
        let res = Self::inner_parse(s.as_bytes())?;
        debug_assert!(res.1.is_empty());
        Ok(res.0)
    }
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
    fn dump_operators_last() {
        let regurgitate = |inp| Node::parse(inp).unwrap().as_string_operators_last();

        assert_eq!(regurgitate("A!!!"), "A!");
        assert_eq!(
            regurgitate("A!B&!C&!D!&!E!&!A>B>!C>!!!F=G!&"),
            "A!BCD!E!ABCFG!&!&!&!&!>>!>!=&"
        );
    }
}
