use std::collections::VecDeque;

#[derive(Debug)]
struct MyStack {
    queue: VecDeque<i32>
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: VecDeque::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        match self.queue.pop_back() {
            Some(x) => x,
            _ => -1
        }
    }

    fn top(&self) -> i32 {
        match self.queue.back() {
            Some(x) => *x,
            _ => -1
        }
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        println!("{:?}", obj);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.empty(), false);
    }
}