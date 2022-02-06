// DOES NOT WORK

pub fn negation_normal_form(formula: &str) -> String {
    let mut stack = formula.to_string();
    let mut i = 0;
    while i < stack.len() {
        if stack.chars().nth(i).unwrap() == '!' && !stack.chars().nth(i - 1).unwrap().is_alphanumeric() {
            stack = negate_string(&stack[..i], 1) + &stack[i+1..];
        }
        i+=1;
    }


    stack


}


fn negate_string(mut stack: &str, c: u32) -> String {
    if c == 0 {
        return stack.to_string();
    }
    if stack.ends_with('!') {
        stack = &stack[..stack.len() - 1];
    }
    let last_letter = stack.chars().last().unwrap();
    stack = &stack[..stack.len() - 1];
    if !last_letter.is_alphanumeric() {
        negate_string(stack, c + 1) + last_letter.to_string().as_str()
    } else {
        negate_string(stack, c - 1) + last_letter.to_string().as_str() + "!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
    }
}
