// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/
// Solved

pub(self) mod first_try {

    pub struct Solution;
    impl Solution {
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let nums_len = nums.len();

            let mut cursor = 0usize;
            let mut next_place = 0usize;
            let mut zero_count = 0usize;

            while cursor < nums_len {
                if nums[cursor] != 0 {
                    nums[next_place] = nums[cursor];
                    next_place += 1;
                }
                else {
                    zero_count += 1;
                }

                cursor += 1;
            }

            while zero_count > 0 {
                nums[nums_len - zero_count] = 0;
                zero_count -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::move_zeroes::first_try::Solution;

    #[test]
    fn example() {
        {
            let mut input = vec![0, 1, 0, 3, 12];
            let expected = vec![1, 3, 12, 0, 0];
            Solution::move_zeroes(&mut input);
            assert_eq!(input, expected);
        }

        {
            let mut input = vec![0];
            let expected = vec![0];
            Solution::move_zeroes(&mut input);
            assert_eq!(input, expected);
        }
    }
}