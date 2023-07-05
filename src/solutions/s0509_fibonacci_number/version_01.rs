struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            ..=-1 => -1,
            _ => Solution::fib(n - 1) + Solution::fib(n - 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
    }
}