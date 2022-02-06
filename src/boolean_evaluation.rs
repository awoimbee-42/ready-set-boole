use anyhow::{Result, anyhow};


fn parse_op(op: u8) -> Result<u8> {
    let operators = [b'|', b'&', b'|', b'^', b'>', b'='];

    if !operators.contains(&op) {
        return Err(anyhow::anyhow!("Invalid operator: {}", op));
    } else {
        Ok(op)
    }
}

fn parse_bool(char: u8) -> Result<bool> {
    match char {
        b'1' => Ok(true),
        b'0' => Ok(false),
        _ => Err(anyhow::anyhow!("invalid boolean value")),
    }
}

/// TODO: doesnt work with negate
fn eval(a: bool, b: bool, op: u8) -> bool {
    match op {
        // negation ¬
        b'!' => !a,
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
        _ => panic!("invalid operator"),
    }
}

fn inner_eval_formula(formula: &[u8]) -> Result<bool> {
    let mut it = formula.iter().peekable();
    let mut vals = Vec::new();

    while it.peek().is_some() {
        let val = it.next().unwrap();
        if parse_bool(*val).is_ok() {
            vals.push(parse_bool(*val)?);
        } else if parse_op(*val).is_ok() {
            let op = parse_op(*val)?;

            let b = vals.pop().unwrap();
            let a = vals.pop().unwrap();
            vals.push(eval(a, b, op));
        } else {
            return Err(anyhow!("Invalid char: {}", *val));
        }
    }
    // assert val.len() == 1;
    Ok(vals.pop().unwrap())
}



pub fn eval_formula(formula: &str) -> bool {
    match inner_eval_formula(formula.as_bytes()) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Could not eval formula: {}", err);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_formula() {
        // TODO: write more tests
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("101|&"));
        assert!(eval_formula("1011||="));
        assert!(!eval_formula("010&1|&"));
    }
}
