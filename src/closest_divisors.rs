// 1362. Closest Divisors
// https://leetcode.com/problems/closest-divisors/
// Solved

pub(self) mod first_try {
    pub struct Solution;

    impl Solution {
        pub fn closest_divisors(num: i32) -> Vec<i32> {
            let plus_one = Self::most_big_two_divisors(num + 1);
            let plus_two = Self::most_big_two_divisors(num + 2);

            if (plus_one.0 - plus_one.1).abs() < (plus_two.0 - plus_two.1).abs() {
                vec![plus_one.0, plus_one.1]
            }
            else {
                vec![plus_two.0, plus_two.1]
            }
        }

        fn most_big_two_divisors(num: i32) -> (i32, i32) {
            let mut divisor = 1i32;

            for n in (1..={(num as f32).sqrt().floor() as i32}).rev() {
                if num % n == 0 {
                    divisor = n;
                    break;
                }
            }

            (divisor, num / divisor)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::closest_divisors::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::closest_divisors(8), vec![3, 3]);
        assert_eq!(Solution::closest_divisors(123), vec![5, 25]);
        assert_eq!(Solution::closest_divisors(999), vec![25, 40]);
        assert_eq!(Solution::closest_divisors(1), vec![1, 2]);
        assert_eq!(Solution::closest_divisors(2), vec![2, 2]);
        assert_eq!(Solution::closest_divisors(3), vec![2, 2]);
        assert_eq!(Solution::closest_divisors(4), vec![2, 3]);

    }
}