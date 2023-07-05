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
        Solution::reserve(head, None)
    }
    
    fn reserve(mut head: Option<Box<ListNode>>, mut prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut current) = head {
            let next = current.next.take();
            current.next = prev;
            prev = Some(current);
            head = next;

            // prev -> head
            // head -> next of head
            // continue this process until head (aka next of head) is None
            // return base condition of FINAL process
            return Solution::reserve(head, prev);
        }
        
        // Base condition
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Create a [1, 2, 3, 4, 5] linked list
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut current = &mut head;
        for num in 2..=5 {
            // To reference inside element of an [&mut Option], use as_mut()
            let Some(mut node) = current.as_mut() else {
                break;
            };
            node.next = Some(Box::new(ListNode::new(num)));
            current = &mut node.next;
        }
        //

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