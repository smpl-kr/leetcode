// solved

pub(self) mod first_try {
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            let (mut max , mut tmp) = (i32::MIN, 0);
            for n in nums.iter() {
                tmp = std::cmp::max(tmp + n, *n);
                max = std::cmp::max(tmp, max);
            }
            max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}

