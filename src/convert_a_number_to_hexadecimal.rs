// 405. Convert a Number to Hexadecimal
// https://leetcode.com/problems/convert-a-number-to-hexadecimal/
// Solved

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn to_hex(num: i32) -> String {
            if num == 0 {
                return "0".to_owned()
            }

            let hex: Vec<char> = "0123456789abcdef".chars().collect();

            let mut unum = num as u32;

            let mut v = vec![];

            loop {
                if unum == 0 {
                    break
                }

                let c = hex[(unum & 0xf) as usize];
                v.push(c);

                unum >>= 4;
            }
            
            v.into_iter().rev().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::convert_a_number_to_hexadecimal::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::to_hex(26), "1a");
        assert_eq!(Solution::to_hex(-1), "ffffffff");
        assert_eq!(Solution::to_hex(0), "0");
    }
}