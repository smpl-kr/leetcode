// 830. Positions of Large Groups
// https://leetcode.com/problems/positions-of-large-groups/
// Solved

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
            let mut ret: Vec<Vec<i32>> = Vec::new();

            let mut prev: char = ' ';
            let mut start = 0usize;
            let mut count = 0usize;
            for (idx, c) in s.chars().enumerate() {
                if prev != c {
                    if count >= 3 {
                        ret.push(vec![start as i32, idx as i32 - 1]);
                    }

                    start = idx;
                    count = 1;
                    prev = c;
                } else {
                    count += 1;
                }
            }

            if count >= 3 {
                ret.push(vec![start as i32, s.len() as i32 - 1]);
            }

            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::positions_of_large_groups::first_try::Solution;

    #[test]
    fn example() {
        let empty: Vec<Vec<i32>> = Vec::new();

        assert_eq!(Solution::large_group_positions("abbxxxxzzy".to_string()), vec![vec![3, 6]]);
        assert_eq!(Solution::large_group_positions("abc".to_string()), empty);
        assert_eq!(Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()), vec![vec![3, 5], vec![6, 9], vec![12, 14]]);
        assert_eq!(Solution::large_group_positions("aaabbbccc".to_string()), vec![vec![0, 2], vec![3, 5], vec![6, 8]]);
    }
}