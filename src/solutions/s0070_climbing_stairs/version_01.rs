struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // Use Fibonacci solution with base case are 1 or 2
        match n {
            1 | 2 => n,
            ..=0 => 0,
            _ => Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        println!("{}", Solution::climb_stairs(44));
        println!("{}", Solution::climb_stairs(45));
    }
}