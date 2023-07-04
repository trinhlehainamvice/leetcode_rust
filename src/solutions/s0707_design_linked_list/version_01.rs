use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct MyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
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
        if self.len == 0 {
            return -1;
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

        return match current.clone() {
            Some(node) => {
                node.borrow().val
            }
            None => {
                -1
            }
        };
    }

    fn add_at_head(&mut self, val: i32) {
        let head = Some(Rc::new(RefCell::new(Node {
            val,
            next: self.head.clone(),
            prev: None,
        })));
        
        self.head = head;
        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let tail = Some(Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: self.tail.clone(),
        })));
        
        self.tail = tail;
        self.len += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        match index {
            _ if index == 0 => {
                self.add_at_head(val);
                return;
            }
            _ if index == self.len => {
                self.add_at_tail(val);
                return;
            }
            _ => ()
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            if let Some(node) = current.clone() {
                current = node.borrow().next.clone();
            }
        }

        if let Some(current_node) = current {
            let prev_node = current_node.borrow().prev.clone();
            let new_node = Some(Rc::new(RefCell::new(Node {
                val,
                next: current_node.borrow().next.clone(),
                prev: prev_node.clone(),
            })));
            current_node.borrow_mut().prev = new_node.clone();
            prev_node.unwrap().borrow_mut().next = new_node;
        }
        
        self.len += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        match index {
            _ if index == 0 => {
                self.delete_at_head();
                return;
            }
            _ if index == self.len => {
                self.delete_at_tail();
                return;
            }
            _ => ()
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            if let Some(node) = current.clone() {
                current = node.borrow().next.clone();
            }
        }

        if let Some(current_node) = current{
            let mut prev_node = current_node.borrow().prev.clone();
            let mut next_node = current_node.borrow().next.clone();
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
        self.head.clone().unwrap().borrow_mut().prev = None;
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
    fn test() {
        let mut obj = MyLinkedList::new();
        let ret_1: i32 = obj.get(2);
        assert_eq!(ret_1, -1);
        obj.add_at_head(1);
        println!("{:?}", obj);
        obj.add_at_tail(2);
        println!("{:?}", obj);
        obj.add_at_index(1, 3);
        println!("{:?}", obj);
        obj.delete_at_index(1);
        println!("{:?}", obj);
    }
}