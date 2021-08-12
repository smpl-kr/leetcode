// solved

pub(self) mod first_try {
    use std::cmp::{Ord, Ordering};
    pub struct Solution;
    impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let (mut mi, mut ni) = (m - 1, n - 1);
            let mut mr = nums1.len() - 1;

            loop {
                //println!("nums1[{:?} nums2[{:?}] mi[{}] ni[{}] mr[{}]", nums1, nums2, mi, ni, mr);

                if ni < 0 {
                    //println!("break1");
                    return
                }

                if mi < 0 {
                    for (i, v) in nums2.into_iter().enumerate().take(ni as usize + 1) {
                        nums1[i] = *v;
                    }
                    //println!("break2");
                    return
                }

                match nums1[mi as usize].cmp(&nums2[ni as usize]) {
                    Ordering::Less => {
                        nums1[mr] = nums2[ni as usize];
                        ni -= 1;
                    },
                    Ordering::Equal => {
                        nums1[mr] = nums1[mi as usize];
                        mi -= 1;
                    },
                    Ordering::Greater => {
                        nums1[mr] = nums1[mi as usize];
                        mi -= 1;
                    }
                }

                mr -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_sorted_array::first_try::Solution;

    #[test]
    fn example1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, [1]);
    }

    #[test]
    fn example3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, [1]);
    }

   #[test]
    fn example4() {
        let mut nums1 = vec![1, 2, 4, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 3, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, [1, 2, 2, 3, 4, 6]);
    }

    #[test]
    fn example5() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![1, 2, 3];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, [1, 2, 3, 4, 5, 6]);
        //nums1 1 
        //nums2 4
        //mi 1 
        //ni 0 
    }

}