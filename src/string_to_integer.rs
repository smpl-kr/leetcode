// https://leetcode.com/problems/string-to-integer-atoi/
// todo

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn my_atoi(s: String) -> i32 {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string_to_integer::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::my_atoi("42".to_owned()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_owned()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_owned()), 4193);
    }
}