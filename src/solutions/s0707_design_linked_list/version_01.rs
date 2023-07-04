use std::cell::RefCell;
use std::rc::Rc;

struct DoubleLinkedNode {
    val: i32,
    prev: Option<Rc<RefCell<DoubleLinkedNode>>>,
    next: Option<Rc<RefCell<DoubleLinkedNode>>>,
}

struct MyLinkedList {
    head: Option<Rc<RefCell<DoubleLinkedNode>>>,
    tail: Option<Rc<RefCell<DoubleLinkedNode>>>,
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
            tail: None,
            len: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if self.len == 0 || index > self.len {
            return -1;
        } else if index == 0 && self.len > 0 {
            return self.head.clone().unwrap().borrow().val;
        } else if index == self.len - 1 {
            return self.tail.clone().unwrap().borrow().val;
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            match current.clone() {
                Some(node) => {
                    current = node.borrow().next.clone();
                }
                None => {
                    return -1;
                }
            }
        }

        return match current {
            Some(node) => {
                node.borrow().val
            }
            None => {
                -1
            }
        };
    }

    fn add_at_head(&mut self, val: i32) {
        let head = Some(Rc::new(RefCell::new(DoubleLinkedNode {
            val,
            next: self.head.clone(),
            prev: None,
        })));

        if self.len == 0 {
            self.head = head.clone();
            self.tail = head;
            self.len += 1;
            return;
        }

        self.head.clone().unwrap().borrow_mut().prev = head.clone();
        self.head = head;
        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let tail = Some(Rc::new(RefCell::new(DoubleLinkedNode {
            val,
            next: None,
            prev: self.tail.clone(),
        })));

        if self.len == 0 {
            self.head = tail.clone();
            self.tail = tail;
            self.len += 1;
            return;
        }

        self.tail.clone().unwrap().borrow_mut().next = tail.clone();
        self.tail = tail;
        self.len += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.len {
            return;
        } else if index == 0 {
            self.add_at_head(val);
            return;
        } else if index == self.len {
            self.add_at_tail(val);
            return;
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            if let Some(node) = current.clone() {
                current = node.borrow().next.clone();
            }
        }

        if let Some(current_node) = current {
            let prev_node = current_node.borrow().prev.clone();
            let new_node = Some(Rc::new(RefCell::new(DoubleLinkedNode {
                val,
                next: Some(current_node.clone()),
                prev: prev_node.clone(),
            })));
            current_node.borrow_mut().prev = new_node.clone();
            prev_node.unwrap().borrow_mut().next = new_node;
        }

        self.len += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if self.len == 0 || index > self.len - 1 {
            return;
        } else if index == 0 {
            self.delete_at_head();
            return;
        } else if index == self.len - 1 {
            self.delete_at_tail();
            return;
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            if let Some(node) = current.clone() {
                current = node.borrow().next.clone();
            }
        }

        if let Some(current_node) = current {
            let prev_node = current_node.borrow().prev.clone();
            let next_node = current_node.borrow().next.clone();
            prev_node.clone().unwrap().borrow_mut().next = next_node.clone();
            next_node.unwrap().borrow_mut().prev = prev_node;
        }

        self.len -= 1;
    }

    fn delete_at_head(&mut self) {
        if self.len == 0 {
            return;
        }
        self.head = self.head.clone().unwrap().borrow().next.clone();
        if let Some(node) = self.head.clone() {
            node.borrow_mut().prev = None;
        }
        self.len -= 1;
    }

    fn delete_at_tail(&mut self) {
        if self.len == 0 {
            return;
        }
        self.tail = self.tail.clone().unwrap().borrow().prev.clone();
        self.tail.clone().unwrap().borrow_mut().next = None;
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