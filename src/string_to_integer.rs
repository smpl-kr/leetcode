// https://leetcode.com/problems/string-to-integer-atoi/
// Solved

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn my_atoi(s: String) -> i32 {
            enum State {
                LeadingSpaces,
                PosOrNeg,
                Numbers,
            }

            let mut positive = true;
            let mut number = 0i64;
            let mut state = State::LeadingSpaces;

            for c in s.chars() {
                if c == ' ' {
                    match state {
                        State::LeadingSpaces => (),
                        _ => break,
                    }
                } else if c == '-' || c == '+' {
                    match state {
                        State::LeadingSpaces => {
                            positive = c == '+';
                            state = State::PosOrNeg;
                        }
                        _ => break,
                    }
                } else if let Some(n) = c.to_digit(10) {
                    match state {
                        State::LeadingSpaces | State::PosOrNeg | State::Numbers => {
                            number = std::cmp::min(
                                number * 10 + n as i64,
                                if positive {
                                    i32::MAX as i64
                                } else {
                                    -1 * i32::MIN as i64
                                },
                            );
                            state = State::Numbers;
                        }
                    }
                } else {
                    break;
                }
            }

            let number = if positive { number } else { -number };
            number as i32
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
        assert_eq!(Solution::my_atoi("+-42".to_owned()), 0); // assumes
        assert_eq!(Solution::my_atoi("-2147483647".to_owned()), -2147483647);
        assert_eq!(Solution::my_atoi("-2147483648".to_owned()), -2147483648);
        assert_eq!(Solution::my_atoi("-2147483649".to_owned()), -2147483648);
        assert_eq!(Solution::my_atoi("2147483646".to_owned()), 2147483646);
        assert_eq!(Solution::my_atoi("2147483647".to_owned()), 2147483647);
        assert_eq!(Solution::my_atoi("2147483648".to_owned()), 2147483647);
        assert_eq!(
            Solution::my_atoi("9223372036854775808".to_owned()),
            2147483647
        );
        assert_eq!(Solution::my_atoi("".to_owned()), 0);
        assert_eq!(Solution::my_atoi("-".to_owned()), 0);
        assert_eq!(Solution::my_atoi("+".to_owned()), 0);
        assert_eq!(Solution::my_atoi("0".to_owned()), 0);
        assert_eq!(Solution::my_atoi("000".to_owned()), 0);
    }
}
