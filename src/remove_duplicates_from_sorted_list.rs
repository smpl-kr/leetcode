// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// Solved

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn generate(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut next_node: Option<Box<ListNode>> = None;
        for each in nums.into_iter().rev() {
            let mut cur_node = ListNode::new(each);
            cur_node.next = next_node;
            let mut cur_node = Some(Box::new(cur_node));

            next_node = cur_node;
        }

        next_node
    }
}

// recursive approach
pub(self) mod first_try {
    use crate::remove_duplicates_from_sorted_list::ListNode;

    pub struct Solution;
    impl Solution {
        fn rec(prev_val: i32, cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if cur.is_none() {
                return None
            }

            let mut cur = cur.unwrap();
            if prev_val == cur.val {
                return Self::rec(prev_val, cur.next)
            }

            cur.next = Self::rec(cur.val, cur.next);
            Some(cur)
        }

        pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() {
                return None
            }

            let mut head = head.unwrap();
            head.next = Self::rec(head.val, head.next);
            Some(head)
        }
    }
}

// procedural approach
pub(self) mod second_try {
    use crate::remove_duplicates_from_sorted_list::ListNode;
    pub struct Solution;
    impl Solution {
        pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() {
                return None
            }

            let mut head = head;
            
            let mut prev = head.as_mut();
            while let Some(p) = prev {
                if let Some(cur) = p.next.as_mut() {
                    if p.val == cur.val {
                        p.next = cur.next.take();
                        prev = Some(p);
                        continue
                    }
                }

                prev = p.next.as_mut();
            }

            head
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates_from_sorted_list::ListNode;

    #[test]
    fn example_first_try() {
        use crate::remove_duplicates_from_sorted_list::first_try::Solution;
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![])), ListNode::generate(vec![]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1])), ListNode::generate(vec![1]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 1])), ListNode::generate(vec![1]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 2])), ListNode::generate(vec![1, 2]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 2, 3, 3])), ListNode::generate(vec![1, 2, 3]));
    }

    #[test]
    fn example_second_try() {
        use crate::remove_duplicates_from_sorted_list::second_try::Solution;
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![])), ListNode::generate(vec![]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1])), ListNode::generate(vec![1]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 1])), ListNode::generate(vec![1]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 2])), ListNode::generate(vec![1, 2]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 2, 3, 3])), ListNode::generate(vec![1, 2, 3]));
    }
}