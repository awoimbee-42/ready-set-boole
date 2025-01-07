use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("invalid character: '{0}'")]
    InvalidChar(char),
    #[error("missing value for operator: {0}")]
    MissingValue(char),
    #[error("formula returns multiple values")]
    TooManyValues,
}

fn eval(a: bool, b: bool, op: u8) -> bool {
    match op {
        // negation ¬
        b'!' => unreachable!("This special case should be handled elsewhere"),
        // Conjunction ∧
        b'&' => a && b,
        // disjunction ∨
        b'|' => a || b,
        // exclusive disjunction ⊕
        b'^' => a ^ b,
        // material condition ⇒
        b'>' => !a || b,
        // logical equivalence ⇔
        b'=' => a == b,
        _ => unreachable!("invalid operator"),
    }
}

pub fn checked_eval_formula(formula: &str) -> Result<bool, ParsingError> {
    let mut val_stack = Vec::new();

    for &val in formula.as_bytes() {
        match val {
            b'0' | b'1' => val_stack.push((val - b'0') != 0),
            b'!' => {
                // `!` is the only one that operates on a single value
                let a = val_stack
                    .last_mut()
                    .ok_or(ParsingError::MissingValue('!'))?;
                *a ^= true;
            }
            b'&' | b'|' | b'^' | b'>' | b'=' => {
                let b = val_stack
                    .pop()
                    .ok_or(ParsingError::MissingValue(val as char))?;
                let a = val_stack
                    .pop()
                    .ok_or(ParsingError::MissingValue(val as char))?;
                val_stack.push(eval(a, b, val));
            }
            _ => return Err(ParsingError::InvalidChar(val as char)),
        }
    }
    if val_stack.len() == 1 {
        Ok(val_stack.pop().unwrap())
    } else {
        Err(ParsingError::TooManyValues)
    }
}

pub fn eval_formula(formula: &str) -> bool {
    checked_eval_formula(formula).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_formula() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("101|&"));
        assert!(eval_formula("1011||="));
        assert!(!eval_formula("010&1|&"));
        assert!(!eval_formula("1!"));

        assert!(!eval_formula("10>"));
        assert!(eval_formula("11>"));
        assert!(eval_formula("01>"));

        assert!(eval_formula("11="));
        assert!(!eval_formula("01="));

        assert!(!eval_formula("11^"));
        assert!(eval_formula("10^"));

        assert!(checked_eval_formula("1&").is_err());
    }
}
