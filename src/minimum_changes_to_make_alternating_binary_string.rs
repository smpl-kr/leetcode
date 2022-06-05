// 1758. Minimum Changes To Make Alternating Binary String
// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/
// Solved

pub(self) mod first_try {
    pub struct Solution;

    impl Solution {
        pub fn min_operations(s: String) -> i32 {
            let zero = Self::check_sequence(&s, true);
            let one = Self::check_sequence(&s, false);

            if zero < one { zero } else { one }
        }

        fn check_sequence(s: &str, start_with_zero: bool) -> i32 {
            let mut flip = 0i32;
            let mut zero = start_with_zero;

            for c in s.chars() {
                if c != if zero { '0' } else { '1' } {
                    flip += 1;
                }

                zero = !zero;
            }

            flip
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_changes_to_make_alternating_binary_string::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::min_operations("0100".to_owned()), 1);
        assert_eq!(Solution::min_operations("10".to_owned()), 0);
        assert_eq!(Solution::min_operations("1111".to_owned()), 2);
        assert_eq!(Solution::min_operations("1".to_owned()), 0);
        assert_eq!(Solution::min_operations("0".to_owned()), 0);
    }
}