struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // use stack for solve [last open first close] or [inside out] related problem
        // or match requirement of inputs in reserved order
        let mut required_close = vec![];

        for c in s.chars() {
            match c {
                // Push match close parentheses to stack
                '(' => required_close.push(')'),
                '[' => required_close.push(']'),
                '{' => required_close.push('}'),
                // Compare close parentheses to that match required close parentheses in stack
                _ => {
                    match required_close.pop() {
                        None => return false,
                        Some(last) if last != c => return false,
                        _ => {},
                    }
                }
            }
        }

        required_close.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "([)]".to_string();
        assert_eq!(Solution::is_valid(s), false);
        let s = "()".to_string();
        assert_eq!(Solution::is_valid(s), true);
        let s = "()[]{}".to_string();
        assert_eq!(Solution::is_valid(s), true);
    }

    #[test]
    fn test_2() {
        let s = "{[()]}".to_string();
        assert_eq!(Solution::is_valid(s), true);
        let s = "(}[]{)".to_string();
        assert_eq!(Solution::is_valid(s), false);
        let s = "[[[]".to_string();
        assert_eq!(Solution::is_valid(s), false);
        let s = "{()[]}".to_string();
        assert_eq!(Solution::is_valid(s), true);
        let s = "[])".to_string();
        assert_eq!(Solution::is_valid(s), false);
        let s = "(([]){})".to_string();
        assert_eq!(Solution::is_valid(s), true);
    }
}