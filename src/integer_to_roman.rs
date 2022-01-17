// https://leetcode.com/problems/integer-to-roman/
// Solved

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn int_to_roman(num: i32) -> String {
            assert!(1 <= num && num <= 3999);

            Self::int_to_roman_recursive(num, 0)
        }

        fn int_to_roman_recursive(num: i32, cipher: i32) -> String {
            let (high, low) = (num / 10, num % 10);

            let (i, v, x) = Self::roman_by_cipher(cipher);
            let low_string = Self::digit_to_roman(low, i, v, x);

            if high == 0 {
                low_string
            } else {
                format!(
                    "{}{}",
                    Self::int_to_roman_recursive(high, cipher + 1),
                    low_string
                )
            }
        }

        fn digit_to_roman(num: i32, one: char, five: char, ten: char) -> String {
            match num {
                1 | 2 | 3 => std::iter::repeat(one).take(num as usize).collect(),
                4 => format!("{}{}", one, five),
                5 => format!("{}", five),
                6 | 7 | 8 => format!(
                    "{}{}",
                    five,
                    std::iter::repeat(one)
                        .take(num as usize - 5)
                        .collect::<String>()
                ),
                9 => format!("{}{}", one, ten),
                _ => "".to_owned(),
            }
        }

        fn roman_by_cipher(cipher: i32) -> (char, char, char) {
            match cipher {
                0 => ('I', 'V', 'X'),
                1 => ('X', 'L', 'C'),
                2 => ('C', 'D', 'M'),
                3 => ('M', '@', '#'),
                _ => panic!("unsupported cipher"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::integer_to_roman::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
    }
}
