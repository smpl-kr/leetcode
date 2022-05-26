// 347. Top K Frequent Elements
// https://leetcode.com/problems/top-k-frequent-elements/
// Solved

mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
            let mut m = std::collections::HashMap::<i32, u32>::new();
            
            for n in nums {
                if let Some(v) = m.get_mut(&n) {
                    *v += 1;
                }
                else {
                    m.insert(n, 1);
                }
            }

            let mut ret = m.iter().map(|(&k, &v)| (k, v)).collect::<Vec<(i32, u32)>>();
            ret.sort_by(|left, right| { left.1.cmp(&right.1).reverse() });
            
            ret.iter().map(|(num, count)| *num).take(k as usize).collect()
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::top_k_frequent_elements::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::top_k_frequent(vec![1,1,1,2,2,3], 2), [1,2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), [1]);
        assert_eq!(Solution::top_k_frequent(vec![4,1,-1,2,-1,2,3], 2), [-1,2]);
    }
}