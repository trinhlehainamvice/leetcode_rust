struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut current: i32 = 0;
        for next in 0..nums.len() {
            if nums[next] != val {
                nums[current as usize] = nums[next];
                current += 1;
            }
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let expected_nums = vec![0, 1, 3, 0, 4];
        let result = Solution::remove_element(&mut nums, val);
        assert_eq!(result, expected_nums.len() as i32);
        assert_eq!(nums.into_iter().take(result as usize).collect::<Vec<i32>>(), expected_nums);
    }
}