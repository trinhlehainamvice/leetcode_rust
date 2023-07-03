use std::ops::Deref;

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
    pub fn merge_two_lists(mut head1: Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (head1, head2) {
            (None, Some(node)) | (Some(node), None) => {
                return Some(node);
            }
            (Some(node1), Some(node2)) => {
                head1 = Some(node1);
                head2 = Some(node2);
            }
            _ => {
                return None;
            }
        }
        
        // Need to create pre_head ListNode instance to able to return head as pre_head.next
        let mut pre_head = ListNode::new(0);
        // Use tail to trace along two linked lists
        let mut tail = &mut pre_head;
        
        while head1.is_some() && head2.is_some() {
            let node1 = head1.as_ref().unwrap();
            let node2 = head2.as_ref().unwrap();
            
            if node1.val < node2.val {
                tail.next = head1;
                tail = tail.next.as_mut().unwrap();
                head1 = tail.next.take();
            }
            else {
                tail.next = head2;
                tail = tail.next.as_mut().unwrap();
                head2 = tail.next.take();
            }
        }

        match (head1.take(), head2.take()) {
            (Some(node), None) | (None, Some(node)) => {
                tail.next = Some(node);
            }
            _ => {}
        }

        pre_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Create a [1, 2, 3] linked list
        let mut head1 = Some(Box::new(ListNode::new(1)));
        let mut current1 = &mut head1;
        for num in [2, 3] {
            // To reference inside element of an [&mut Option], use as_mut()
            let Some(mut node) = current1.as_mut() else {
                break;
            };
            node.next = Some(Box::new(ListNode::new(num)));
            current1 = &mut node.next;
        }

        let mut head2 = Some(Box::new(ListNode::new(1)));
        let mut current2 = &mut head2;
        for num in [3, 4, 4, 5, 6] {
            // To reference inside element of an [&mut Option], use as_mut()
            let Some(mut node) = current2.as_mut() else {
                break;
            };
            node.next = Some(Box::new(ListNode::new(num)));
            current2 = &mut node.next;
        }

        let mut head = Solution::merge_two_lists(head1, head2);
        let mut result = vec![];
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }

        let expected = vec![1, 1, 2, 3, 3, 4, 4, 5, 6];
        assert_eq!(result, expected);
    }
}