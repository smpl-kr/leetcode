// https://leetcode.com/problems/integer-to-roman/

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn int_to_roman(num: i32) -> String {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string_to_integer::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}