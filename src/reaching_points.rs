// 780. Reaching Points
// https://leetcode.com/problems/reaching-points/
// Solved. Discuss를 좀 컨닝함...

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
            let mut s = Vec::<(i32, i32, i32, i32)>::new();
            s.reserve(1024);
            s.push((sx, sy, tx, ty));

            loop {
                if let Some((x0, y0, x1, y1)) = s.pop() {
                    if x0 > x1 || y0 > y1 {
                        continue
                    }

                    //println!("sx = {}, sy = {}, tx = {}, ty = {}, stack size = {}", x0, y0, x1, y1, s.len());

                    if x0 == x1 && y0 == y1 { 
                        return true
                    }

                    if x0 == x1 {
                        // ty == sy + sx * N 의 경우를 검사.
                        return (y1 - y0) % x0 == 0
                    }

                    if y0 == y1 {
                        // tx == sx + sy * N 의 경우를 검사.
                        return (x1 - x0) % y0 == 0
                    }

                    // source -> target으로 올라가도 되지만, tx, ty가 훨씬 크므로 target -> source로 하면 빠르게 진행 가능.
                    if x1 >= y1 {
                        s.push((x0, y0, x1 - y1, y1));
                    }
                    else {
                        s.push((x0, y0, x1, y1 - x1));
                    }
                }
                else {
                    return false
                }
            }

            false
        }
    }   
}

#[cfg(test)]
mod tests {
    use crate::reaching_points::first_try::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
        assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
        assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
        assert_eq!(Solution::reaching_points(1, 1, 1128944, 6345634), false);
    }
}