struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        // '+'.Record a new score that is the sum of the previous two scores.
        // 'D'.Record a new score that is the double of the previous score.
        // 'C'.Invalidate the previous score, removing it from the record. 
        let mut records = vec![0];
        for op in operations {
            if let Ok(num) = op.parse::<i32>() {
                records.push(num);
            }
            else if op == "+".to_string() {
                records.push(records[records.len() - 1] + records[records.len() - 2]);
            }
            else if op == "D".to_string() {
                records.push(records[records.len() - 1] * 2);
            }
            else if op == "C".to_string() {
                records.pop();
            }
        }
        
        let mut sum = 0;
        for num in records {
            sum += num;
        }
        
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ops = vec!["5".to_string(),"2".to_string(),"C".to_string(),"D".to_string(),"+".to_string()];
        let expected = 30;
        assert_eq!(Solution::cal_points(ops), expected);
    }
}