pub struct Solution;

impl Solution {
    pub fn remove_duplicates_02(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        if len <= 1 {
            return len as i32;
        }

        let mut unique_count = 1;

        let mut current = 0;
        for next in 1..nums.len() {
            if nums[next] > nums[current] {
                unique_count += 1;
                nums[current + 1] = nums[next];
                current += 1;
            }
        }
        
        unique_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![1, 1, 2, 3, 3, 3, 4, 4];
        let expected_nums = vec![1, 2, 3, 4];
        let unique_count = Solution::remove_duplicates_02(&mut nums);
        assert_eq!(unique_count, expected_nums.len() as i32);
        assert_eq!(nums.into_iter().take(unique_count as usize).collect::<Vec<i32>>(), expected_nums);
    }
}