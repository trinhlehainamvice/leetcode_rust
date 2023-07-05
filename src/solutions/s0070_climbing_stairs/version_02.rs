struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 | 2 => {
                return n;
            },
            ..=0 => {
                return 0;
            }
            _ => {}
        };
        
        // Use Fibonacci iterative solution
        // Calculate from base case to n
        // Base case [1,2]:
        // F(1) = 1, F(2) = 2
        // F(3) = F(2) + F(1) = 2 + 1 = 3
        // F(4) = F(3) + F(2) = 3 + 2 = 5
        // F(5) = F(4) + F(3) = 5 + 3 = 8
        // ...
        
        let mut a = 1;
        let mut b = 2;
        let mut c = 0;
        
        for _ in 3..=n {
            c = a + b;
            a = b;  
            b = c;
        }
        
        c
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