use std::arch::x86_64::_mm_sha1nexte_epu32;
use std::mem::swap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node: Option<Box<ListNode>> = None;

        // Assign current as head
        // Move head to next
        // Swap process:
        // - current.next -> node
        // - node -> current

        while let Some(mut current) = head {
            // Move head to next
            head = current.next.take();
            // Move next to node
            current.next = match node {
                Some(node) => Some(node),
                None => None,
            };
            // Move node to current
            node = Some(current);
        }

        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut first = Box::new(ListNode::new(1));
        let mut second = Box::new(ListNode::new(2));
        let mut third = Box::new(ListNode::new(3));
        let mut fourth = Box::new(ListNode::new(4));
        let mut fifth = Box::new(ListNode::new(5));
        fourth.next = Some(fifth);
        third.next = Some(fourth);
        second.next = Some(third);
        first.next = Some(second);

        let mut head = Some(first);
        let mut reserved_head = Solution::reverse_list(head);

        let mut result = vec![];
        while let Some(node) = reserved_head {
            result.push(node.val);
            reserved_head = node.next;
        }

        let expected = vec![5, 4, 3, 2, 1];
        assert_eq!(result, expected);
    }
}