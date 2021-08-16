// 43. Multiply Strings
// https://leetcode.com/problems/multiply-strings/
// Solved

// first dirty solution
pub(self) mod first_try {
    use std::iter::Iterator;

    pub struct Solution;
    impl Solution {
        pub fn multiply(num1: String, num2: String) -> String {
            if num1 == "0" || num2 == "0" {
                return "0".to_owned()
            }

            const MAX_SIZE: usize = 200 * 200;
            let mut result = [0u8; MAX_SIZE];
            let mut begin = MAX_SIZE - 1;

            for (j, jv) in num2.chars().rev().enumerate() {
                for (i, iv) in num1.chars().rev().enumerate() {
                    let iv = iv.to_digit(10).unwrap();
                    let jv = jv.to_digit(10).unwrap();
                    let mulv = iv * jv;

                    let (mut high, low) = (mulv / 10, mulv % 10);
                    let mut pos = MAX_SIZE - (i + j) - 1;

                    let tmp = result[pos] as u32 + low;
                    high += if tmp >= 10 {
                        result[pos] = (tmp % 10) as u8;
                        tmp / 10
                    } else {
                        result[pos] = (tmp % 10) as u8;
                        0
                    };

                    while high > 0 {
                        pos -= 1;
                        let low = high % 10;

                        high /= 10;

                        let tmp = result[pos] as u32 + low;
                        high += if tmp >= 10 {
                            result[pos] = (tmp % 10) as u8;
                            tmp / 10
                        } else {
                            result[pos] = (tmp % 10) as u8;
                            0
                        };
                    }

                    begin = std::cmp::min(begin, pos);
                }
            }

            result[begin..]
                .iter()
                .map(|n| std::char::from_digit(*n as u32, 10).unwrap())
                .collect()
        }
    }
}


// second little-bit better solution
pub(self) mod second_try {
    use std::iter::Iterator;
    use std::char;
    use std::cmp::min;

    pub struct Solution;
    impl Solution {
        pub fn multiply(num1: String, num2: String) -> String {
            if num1 == "0" || num2 == "0" {
                return "0".to_owned()
            }

            const MAX_SIZE: usize = 200 * 200;
            let mut ret = [0u8; MAX_SIZE];
            let mut begin = MAX_SIZE - 1;
            
            for (j, jv) in num2.chars().rev().enumerate() {
                for (i, iv) in num1.chars().rev().enumerate() {
                    let mulv = iv.to_digit(10).unwrap() * jv.to_digit(10).unwrap();
                    let mut pos = MAX_SIZE - (i + j) - 1;

                    // add v to res
                    let adder = |res: &mut [u8], v: u32, i: usize| -> u32 {
                        let low = res[i] as u32 + (v % 10);
                        res[i] = (low % 10) as u8;
                        let carry = v / 10 + low / 10;

                        carry
                    };

                    let mut carry = adder(&mut ret, mulv, pos);

                    while carry > 0 {
                        pos -= 1;
                        carry = adder(&mut ret, carry, pos);
                    }

                    begin = min(begin, pos);
                }
            }

            ret[begin..]
                .iter()
                .map(|&n| char::from_digit(n as u32, 10).unwrap())
                .collect()
        }
    }
}

// solution using iterator
pub(self) mod third_try {
    use std::iter::Iterator;
    use std::char;
    use std::cmp::min;

    pub struct Solution;
    impl Solution {
        pub fn multiply(num1: String, num2: String) -> String {
            if num1 == "0" || num2 == "0" {
                return "0".to_owned()
            }

            const MAX_SIZE: usize = 200 * 200;
            let mut ret = [0u8; MAX_SIZE];
            let mut begin = MAX_SIZE - 1;
            
            for (j, jv) in num2.chars().rev().enumerate() {
                for (i, iv) in num1.chars().rev().enumerate() {

                    let multiplied = iv.to_digit(10).unwrap() * jv.to_digit(10).unwrap();
                    let adder = Adder{
                        res: &mut ret,
                        pos: MAX_SIZE - (i + j) - 1,
                        value: multiplied
                    };
                    for pos in adder {
                        begin = min(begin, pos);
                    }
                }
            }

            ret[begin..]
                .iter()
                .map(|&n| char::from_digit(n as u32, 10).unwrap())
                .collect()
        }
    }

    struct Adder<'a> {
        res: &'a mut [u8],
        pos: usize,
        value: u32,
    }

    impl<'a> std::iter::Iterator for Adder<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.value == 0 {
                return None
            }

            let lowpart = self.res[self.pos] as u32 + (self.value % 10);
            self.res[self.pos] = (lowpart % 10) as u8;
            let carry = self.value / 10 + lowpart / 10;

            let position = self.pos;
            self.pos -= 1;
            self.value = carry;

            Some(position)
        }
    }
}

#[cfg(test)]
mod tests {
    //use crate::multiply_strings::first_try::Solution;
    //use crate::multiply_strings::second_try::Solution;
    use crate::multiply_strings::third_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::multiply("0".to_owned(), "0".to_owned()), "0".to_owned());
        assert_eq!(Solution::multiply("2".to_owned(), "0".to_owned()), "0".to_owned());
        assert_eq!(Solution::multiply("0".to_owned(), "3".to_owned()), "0".to_owned());
        assert_eq!(Solution::multiply("2".to_owned(), "3".to_owned()), "6".to_owned());
        assert_eq!(Solution::multiply("123".to_owned(), "456".to_owned()), "56088".to_owned());
        assert_eq!(Solution::multiply("57986882437".to_owned(), "24378".to_owned()), "1413604220049186".to_owned());
    }
}