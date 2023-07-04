struct SinglyLinkedNode {
    val: i32,
    next: Option<Box<SinglyLinkedNode>>,
}

struct MyLinkedList {
    head: Option<Box<SinglyLinkedNode>>,
    len: i32,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            len: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if self.len == 0 || index > self.len - 1 {
            return -1;
        }

        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref().unwrap().next;
        }

        current.as_ref().unwrap().val
    }

    fn add_at_head(&mut self, val: i32) {
        let head = Some(Box::new(SinglyLinkedNode {
            val,
            next: self.head.take(),
        }));
        self.head = head;
        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        if self.head.is_none() {
            self.add_at_head(val);
            return;
        }

        let mut tail = &mut self.head;
        for _ in 0..(self.len - 1) {
            tail = &mut tail.as_mut().unwrap().next;
        }

        let node = Some(Box::new(SinglyLinkedNode {
            val,
            next: None,
        }));

        tail.as_mut().unwrap().next = node;
        self.len += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
            return;
        } else if index == self.len {
            self.add_at_tail(val);
            return;
        } else if index > self.len {
            return;
        }

        let mut prev = &mut self.head;
        for _ in 0..(index - 1) {
            prev = &mut prev.as_mut().unwrap().next;
        }
        let next = &mut prev.as_mut().unwrap().next;
        let node = Some(Box::new(SinglyLinkedNode {
            val,
            next: next.take(),
        }));
        prev.as_mut().unwrap().next = node;
        self.len += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 && self.len > 0 {
            self.head = self.head.take().unwrap().next;
            self.len -= 1;
            return;
        } else if index > self.len - 1 {
            return;
        }

        let mut prev = &mut self.head;
        for _ in 0..(index - 1) {
            prev = &mut prev.as_mut().unwrap().next;
        }
        let current = &mut prev.as_mut().unwrap().next;
        prev.as_mut().unwrap().next = current.take().unwrap().next;

        self.len -= 1;
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = MyLinkedList::new();
        let ret_1: i32 = obj.get(2);
        assert_eq!(ret_1, -1);
        obj.add_at_head(1);
        let ret_2: i32 = obj.get(0);
        assert_eq!(ret_2, 1);
        obj.add_at_tail(2);
        assert_eq!(obj.get(1), 2);
        obj.add_at_index(1, 3);
        assert_eq!(obj.get(1), 3);
        assert_eq!(obj.get(2), 2);
        obj.delete_at_index(1);
        assert_eq!(obj.get(1), 2);
        obj.delete_at_index(1);
        obj.delete_at_index(0);
    }

    #[test]
    fn test_2() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(7);
        obj.add_at_head(2);
        obj.add_at_head(1);
        obj.add_at_index(3, 0);
        obj.delete_at_index(2);
        obj.add_at_head(6);
        obj.add_at_tail(4);
        assert_eq!(obj.get(4), 4);
    }

    #[test]
    fn test_03() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(2);
        obj.delete_at_index(1);
        obj.add_at_head(2);
        obj.add_at_head(7);
        obj.add_at_head(3);
        obj.add_at_head(2);
        obj.add_at_head(5);
        obj.add_at_tail(5);
        assert_eq!(obj.get(5), 2);
        obj.delete_at_index(6);
        obj.delete_at_index(4);
    }

    #[test]
    fn test_04() {
        let mut obj = MyLinkedList::new();
        obj.add_at_index(1, 0);
        assert_eq!(obj.get(0), -1);
    }
}