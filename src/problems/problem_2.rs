use super::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current1 = l1;
        let mut current2 = l2;
        let mut result: Option<Box<ListNode>> = None;
        let mut tail = &mut result;
        let mut carry = 0;

        while current1.is_some() || current2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(node) = current1 {
                sum += node.val;
                current1 = node.next;
            };

            if let Some(node) = current2 {
                sum += node.val;
                current2 = node.next;
            }

            carry = sum / 10;
            sum %= 10;

            let new_node = Some(Box::new(ListNode::new(sum)));

            if let Some(node) = tail {
                node.next = new_node;
            } else {
                *tail = new_node;
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }
}

pub fn example() {
    // [2,4,3]
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    // [5,6,4]
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let result = Solution::add_two_numbers(l1, l2);

    let expected = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 8, next: None })),
        })),
    }));

    assert_eq!(result, expected);
}
