use std::cmp::Ordering;

use thiserror::Error;

use crate::bool_formula_ast::{MyError, Node};

#[derive(Error, Debug)]
pub enum SetFormulaError {
    #[error("invalid character: '{0}'")]
    InvalidChar(char),
    #[error("missing value for operator: {0}")]
    MissingValue(char),
    #[error("missing set: {0}")]
    MissingSet(char),
    #[error("more than one result")]
    TooManyValues(),
    #[error("parsing error: {0}")]
    ParsingError(#[from] MyError),
}

pub fn checked_eval_set(formula: &str, sets: &[&[i32]]) -> Result<Vec<i32>, SetFormulaError> {
    let mut tree = Node::parse(formula)?;
    tree.to_primitive_connectives_mut();
    tree.to_negation_normal_form_mut();
    let formula = tree.to_string();

    let mut sets: Vec<Vec<i32>> = sets.iter().map(|s| s.to_vec()).collect();
    sets.iter_mut().for_each(|s| s.sort());

    let mut val_stack = Vec::new();

    for &val in formula.as_bytes() {
        match val {
            b'A'..=b'Z' => {
                let set = sets
                    .get((val - b'A') as usize)
                    .ok_or(SetFormulaError::MissingSet(val as char))?
                    .clone();
                val_stack.push(set)
            }
            b'!' => {
                let a = val_stack
                    .last_mut()
                    .ok_or(SetFormulaError::MissingValue('!'))?;
                a.clear();
            }
            b'&' | b'|' => {
                let b = val_stack
                    .pop()
                    .ok_or(SetFormulaError::MissingValue(val as char))?;
                let mut a = val_stack
                    .pop()
                    .ok_or(SetFormulaError::MissingValue(val as char))?;
                match val {
                    // Conjunction ∧
                    b'&' => {
                        let mut new = vec![];
                        let mut i = 0;
                        let mut j = 0;
                        while i < a.len() && j < b.len() {
                            match a[i].cmp(&b[j]) {
                                Ordering::Equal => {
                                    new.push(a[i]);
                                    i += 1;
                                    j += 1;
                                }
                                Ordering::Less => {
                                    i += 1;
                                }
                                Ordering::Greater => {
                                    j += 1;
                                }
                            }
                        }
                        a = new;
                    }
                    // disjunction ∨
                    b'|' => {
                        a.extend(b.iter());
                    }
                    _ => unreachable!(),
                }
                val_stack.push(a);
            }
            _ => return Err(SetFormulaError::InvalidChar(val as char)),
        }
    }
    if val_stack.len() == 1 {
        Ok(val_stack.pop().unwrap())
    } else {
        Err(SetFormulaError::TooManyValues())
    }
}

pub fn eval_set(formula: &str, sets: &[&[i32]]) -> Vec<i32> {
    checked_eval_set(formula, sets).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_set() {
        assert_eq!(eval_set("AB&", &[&[0, 1, 2], &[0, 3, 4]]), vec![0]);
        assert_eq!(
            eval_set("AB|", &[&[0, 1, 2], &[3, 4, 5]]),
            vec![0, 1, 2, 3, 4, 5]
        );
        assert_eq!(eval_set("A!", &[&[0, 1, 2]]), vec![]);
    }
}
