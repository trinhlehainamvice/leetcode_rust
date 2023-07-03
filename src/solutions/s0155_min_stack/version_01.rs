struct MinStack {
    top: i32,
    min: i32,
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
            top: -1,
            min: i32::MAX,
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.top += 1;
        self.stack.push(val);
        if val <= self.min {
            self.min = val;
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        if self.top == -1 {
            return;
        }
        self.top -= 1;
        let last = self.stack.pop().unwrap();
        if last == self.min {
            self.min_stack.pop();
            if let Some(min) = self.min_stack.last() {
                self.min = *min;
            } else {
                self.min = i32::MAX;
            }
        }
    }

    fn top(&self) -> i32 {
        if self.top == -1 {
            return 0;
        }
        self.stack[self.top as usize].clone()
    }

    fn get_min(&self) -> i32 {
        self.min
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
        assert_eq!(obj.top(), 0);
        obj.push(0);
        obj.push(1);
        obj.push(0);
        assert_eq!(obj.get_min(), 0);
        obj.pop();
        assert_eq!(obj.top(), 1);
        assert_eq!(obj.get_min(), 0);
    }
}