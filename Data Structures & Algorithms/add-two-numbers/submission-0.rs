// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = false;

        let mut l1 = l1;
        let mut l2 = l2;

        let mut head = Box::new(ListNode::new(0));
        let mut current = head.as_mut();

        while l1.is_some() || l2.is_some() || carry {
            let mut sum = carry as i32;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            current.val = sum % 10;
            carry = sum >= 10;
            current.next = if l1.is_some() || l2.is_some() || carry {
                Some(Box::new(ListNode::new(0)))
            } else {
                None
            };
            current = match current.next.as_mut() {
                Some(node) => node,
                None => break,
            };
        }

        Some(head)
    }
}