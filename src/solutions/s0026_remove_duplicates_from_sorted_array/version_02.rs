pub struct Solution;

impl Solution {
    pub fn remove_duplicates_02(nums: &mut Vec<i32>) -> i32 {
        let mut current = 0;
        for next in 0..nums.len() {
            if nums[next] > nums[current] {
                current += 1;
                nums[current] = nums[next];
            }
        }
        current += 1;
        current as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![1,2,2,3,3,4,4,5,5];
        let expected_nums = vec![1,2,3,4,5];
        let unique_count = Solution::remove_duplicates_02(&mut nums);
        assert_eq!(unique_count, expected_nums.len() as i32);
        assert_eq!(nums.into_iter().take(unique_count as usize).collect::<Vec<i32>>(), expected_nums);
    }
}