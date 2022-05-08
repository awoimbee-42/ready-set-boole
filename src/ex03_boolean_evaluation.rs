use anyhow::{anyhow, Result};

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

pub fn checked_eval_formula(formula: &str) -> Result<bool> {
    let mut val_stack = Vec::new();

    for &val in formula.as_bytes() {
        match val {
            b'0' | b'1' => val_stack.push((val - b'0') != 0),
            b'!' => {
                // `!` is the only one that operates on a single value
                let a = val_stack
                    .pop()
                    .ok_or_else(|| anyhow!("Missing value for `!`"))?;
                val_stack.push(!a);
            }
            b'&' | b'|' | b'^' | b'>' | b'=' => {
                let b = val_stack
                    .pop()
                    .ok_or_else(|| anyhow!("Missing 2 values for operator {val}"))?;
                let a = val_stack
                    .pop()
                    .ok_or_else(|| anyhow!("Missing 1 value for operator {val}"))?;
                val_stack.push(eval(a, b, val));
            }
            _ => return Err(anyhow!("Invalid character: {}", val)),
        }
    }
    if val_stack.len() == 1 {
        Ok(val_stack.pop().unwrap())
    } else {
        Err(anyhow!("formula returns multiple values"))
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
