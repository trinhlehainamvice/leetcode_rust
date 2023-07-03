struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_stack.last() {
            // Handle duplicates min
            Some(min) if val <= *min => {
                self.min_stack.push(val);
            }
            None => self.min_stack.push(val),
            _ => (),
        }
    }

    fn pop(&mut self) {
        match (self.stack.pop(), self.min_stack.last()) {
            (Some(val), Some(min)) if val == *min => {
                self.min_stack.pop(); 
            },
            _ => {}
        }
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().clone()
    }

    fn get_min(&self) -> i32 {
        self.min_stack.last().unwrap().clone()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = MinStack::new();
        obj.push(0);
        obj.push(1);
        obj.push(0);
        assert_eq!(obj.get_min(), 0);
        obj.pop();
        assert_eq!(obj.top(), 1);
        assert_eq!(obj.get_min(), 0);
    }
}