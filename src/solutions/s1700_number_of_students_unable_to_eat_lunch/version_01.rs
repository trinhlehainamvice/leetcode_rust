struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students;
        let mut sandwiches = sandwiches;

        loop {
            let front_student = students.remove(0);
            if front_student != sandwiches[0] {
                students.push(front_student);
            } else {
                sandwiches.remove(0);
            }

            if students.is_empty() {
                break;
            }

            if students
                .iter()
                .all(|student| *student != sandwiches[0])
            {
                break;
            }
        }

        students.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        assert_eq!(Solution::count_students(students, sandwiches), 3);
    }
}