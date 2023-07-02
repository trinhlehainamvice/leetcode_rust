struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                _ => {
                    if stack.is_empty() {
                        return false;
                    }
                    let top = stack.pop().unwrap();
                    match (top, c) {
                        ('(', ')') | ('[', ']') | ('{', '}') => continue,
                        _ => return false
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