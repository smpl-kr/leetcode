// 1889. Minimum Space Wasted From Packaging
// https://leetcode.com/problems/minimum-space-wasted-from-packaging/
// Solved.

pub(self) mod first_try {
    pub struct Solution;

    impl Solution {
        pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
            const MOD: i64 = 10i64.pow(9) + 07i64;
            let mut minimum_space = std::i64::MAX;

            let sum_of_pkgs = packages.iter().fold(0i64, |acc, &n| (acc + n as i64) % MOD);

            let mut pkgs = packages;
            pkgs.sort();

            let mut boxes = boxes;
            for each_boxes in boxes.iter_mut() {
                each_boxes.sort();

                if pkgs.last().unwrap() > each_boxes.last().unwrap() {
                    continue;
                }

                let mut space = 0i64;
                let mut prev = 0usize;

                for b in each_boxes.iter() {
                    let idx = Self::binary_search(&pkgs, b);

                    space += *b as i64 * (idx - prev) as i64;
                    prev = idx;
                }

                if minimum_space > space {
                    minimum_space = space;
                }
            }

            if minimum_space != std::i64::MAX { ((minimum_space - sum_of_pkgs) % MOD) as i32 } else { -1 }
        }
        
        fn binary_search(v: &[i32], n: &i32) -> usize {
            let mut start = 0i32;
            let mut end = v.len() as i32 - 1;

            while start <= end {
                let mid = (start + end) / 2;

                if v[mid as usize] <= *n {
                    start = mid + 1;
                }
                else {
                    end = mid - 1;
                }
            }

            start as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_space_wasted_from_packaging::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::min_wasted_space(vec![2,3,5], vec![vec![4,8], vec![2,8]]), 6);
        assert_eq!(Solution::min_wasted_space(vec![2,3,5], vec![vec![1,4], vec![2,3], vec![3,4]]), -1);
        assert_eq!(Solution::min_wasted_space(vec![3,5,8,10,11,12], vec![vec![12], vec![11,9], vec![10,5,14]]), 9);
        assert_eq!(Solution::min_wasted_space(vec![7,6,5,3,4], vec![vec![2,7], vec![6], vec![10,5]]), 10);
    }
}