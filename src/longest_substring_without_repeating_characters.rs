// solved

#![allow(dead_code)]
pub(self) mod structural_try {
    // 절차적으로 처음 짠 버전..

    use std::cmp::PartialEq;
    use std::collections::VecDeque;

    pub struct Solution;
    impl Solution {
        fn find<T: std::cmp::PartialEq + Copy>(vd: &VecDeque<T>, fv: &T) -> Option<usize> {
            for (i, v) in vd.iter().enumerate() {
                if v == fv {
                    return Some(i);
                }
            }

            None
        }

        pub fn length_of_longest_substring(s: String) -> i32 {
            let mut subchars = VecDeque::new();
            let mut longest = 0i32;

            for c in s.chars() {
                if let Some(pos) = Self::find::<char>(&subchars, &c) {
                    subchars.drain(0..pos + 1);
                }

                subchars.push_back(c);

                if longest < subchars.len() as i32 {
                    longest = subchars.len() as i32;
                }
            }

            return longest;
        }
    }
}

#[allow(dead_code)]
pub(self) mod functional_try {
    // 조금이라도 더 함수형으로 짜려고 한 버전

    use std::cmp::max;
    use std::collections::VecDeque;

    pub struct Solution;
    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            s.chars()
                .fold((VecDeque::new(), 0), |(mut subchars, mut longest), c| {
                    if let Some(i) = subchars.iter().position(|v| v == &c) {
                        subchars.drain(0..i + 1);
                    }

                    subchars.push_back(c);

                    longest = max(longest, subchars.len());

                    (subchars, longest)
                })
                .1 as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_substring_without_repeating_characters::functional_try::Solution;

    #[test]
    fn length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_owned()), 0);
        assert_eq!(Solution::length_of_longest_substring("So every time you do vector.iter(), you get a new iterator that starts from the beginning, thus all the position() called on that will just search from the beginning, thus getting only the first ‘\r’. What I would normally do(there should be more idiomatic ways) would just be".to_owned()), 10);
        assert_eq!(Solution::length_of_longest_substring("aab".to_owned()), 2);
    }
}
