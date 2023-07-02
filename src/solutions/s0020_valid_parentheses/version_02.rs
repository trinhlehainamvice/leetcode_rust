struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                // Push match close parentheses to stack
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                // Compare close parentheses to that match required close parentheses in stack
                _ => {
                    match stack.pop() {
                        None => return false,
                        Some(last) if last != c => return false,
                        _ => {},
                    }
                }
            }
        }

        stack.is_empty()
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