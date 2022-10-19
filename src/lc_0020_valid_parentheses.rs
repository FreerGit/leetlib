pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let mut stack: Vec<char> = vec![];
    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '}' | ']' | ')' => {
                if let Some(n) = stack.pop() {
                    if n != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(is_valid("()".to_string()), true);
    }
    #[test]
    fn ex2() {
        assert_eq!(is_valid("()[]{{}}".to_string()), true);
    }

    #[test]
    fn ex3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
    #[test]
    fn ex4() {
        assert_eq!(is_valid("(".to_string()), false);
    }
    #[test]
    fn ex5() {
        assert_eq!(is_valid("((".to_string()), false);
    }
}
