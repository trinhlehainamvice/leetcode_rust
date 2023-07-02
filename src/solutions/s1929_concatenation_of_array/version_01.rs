struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![0; len * 2];
        for i in 0..len {
            ans[i] = nums[i];
            ans[i + len] = nums[i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        let expected_nums = vec![1, 2, 3, 1, 2, 3];
        assert_eq!(Solution::get_concatenation(nums), expected_nums);
    }
}