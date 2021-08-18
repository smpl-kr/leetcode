// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// todo

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

pub(self) mod first_try {
    use crate::remove_duplicates_from_sorted_list::ListNode;

    pub struct Solution;
    impl Solution {
        pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates_from_sorted_list::first_try::Solution;
    use crate::remove_duplicates_from_sorted_list::ListNode;

    #[test]
    fn example() {
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 2])), ListNode::generate(vec![1, 2]));
        assert_eq!(Solution::delete_duplicates(ListNode::generate(vec![1, 1, 2, 3, 3])), ListNode::generate(vec![1, 2, 3]));
    }
}