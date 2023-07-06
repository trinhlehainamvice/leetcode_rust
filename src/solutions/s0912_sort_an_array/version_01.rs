struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let end = nums.len() - 1;
        Solution::merge_sort(&mut nums, 0, end);
        nums
    }

    fn merge_sort(nums: &mut Vec<i32>, start: usize, end: usize) {
        if start >= end {
            return;
        }

        let mid = (start + end) / 2;
        Solution::merge_sort(nums, start, mid);
        Solution::merge_sort(nums, mid + 1, end);

        Solution::merge(nums, start, mid, end);
    }

    fn merge(nums: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
        // Use merge two sorted lists or arrays algorithm
        // Reference: s0021_merge_two_sorted_lists
        
        let mut result = vec![0; end - start + 1];
        
        let mut i = start;
        let mut j = mid + 1;
        let mut k = 0;
        while i <= mid && j <= end {
            if nums[i] <= nums[j] {
                result[k] = nums[i];
                i += 1;
            } else {
                result[k] = nums[j];
                j += 1;
            }
            k += 1;
        }
        
        // Fill remaining elements
        while i <= mid {
            result[k] = nums[i];
            i += 1;
            k += 1;
        }
        
        // Fill remaining elements
        while j <= end {
            result[k] = nums[j];
            j += 1;
            k += 1;
        }
        
        nums[start..=end].copy_from_slice(&result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::sort_array(vec![2, 3, 1, 4, 5]), vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::sort_array(vec![5, 2, 3]), vec![2, 3, 5]);
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }
}