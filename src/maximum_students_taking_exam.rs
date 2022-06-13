// 1349. Maximum Students Taking Exam
// https://leetcode.com/problems/maximum-students-taking-exam/
// Solved

pub(self) mod first_try {
    pub struct Solution;
    impl Solution {
        pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
            let mut possible_rows = Vec::<std::collections::HashMap<u32, u32>>::new();

            // 각 줄 별로 모든 가능한 자리배치를 계산
            for s in seats.iter() {
                possible_rows.push(Solution::possible_rows(s));
            }

            // 첫 줄의 가능한 자리배치들의 학생이 앉은 자리 갯수를 계산
            let mut row = possible_rows.drain(..1).take(1).next().unwrap();
            for (num, count) in row.iter_mut() {
                *count = Solution::count_of_bit(*num);
            }

            loop {
                // 모든 줄을 탐색할 때까지...
                if possible_rows.is_empty() {
                    break
                }

                // 다음줄로 넘어가면서 계산. 전 줄까지 계산된 누적된 학생이 앉아 있는 자리 갯수 정보를 활용한다.
                let prior_row = row;
                row = possible_rows.drain(..1).take(1).next().unwrap();

                // 전 줄의 자리배치와 이번 줄의 자리배치가 문제 조건에 부합하는지를 확인하고, 부합한다면,
                // 전 줄까지의 자리배치를 계산하면서, 학생이 최대로 앉아있을 수 있는 자리갯수를 얻었을텐데,
                // 그 최대 자리 갯수를 이번 줄의 각 배치의 자리 갯수에 합산한다. (누적)
                // 그러면 이번 줄까지의 각 자리배치에서 최대로 학생들이 앉을 수 있는 자리갯수를 도출할 수 있다.
                for (cur_num, cur_count) in row.iter_mut() {
                    let count_of_bit = Solution::count_of_bit(*cur_num);

                    for (prior_num, prior_count) in prior_row.iter() {
                        if Solution::compare_rows(prior_num, cur_num) {
                            if count_of_bit + *prior_count > *cur_count {
                                *cur_count = count_of_bit + *prior_count;
                            }
                        }
                    }
                }
            }

            // 마지막 줄의 배치들 중 자리 갯수가 가장 큰 값을 리턴
            let mut largest = 0u32;
            for (_, count) in row.iter() {
                if *count > largest {
                    largest = *count;
                }
            }

            largest as i32
        }

        // 현재 줄의 빈 자리 정보로부터, 배치 가능한 모든 경우의 수를 구함.
        // 해시맵의 키는 배치의 모습을 8bit 비트필드로 표현한 것이고, 밸류는 해당 배치에서 나올 수 있는 자리 갯수이다.
        // 이 함수에서는 아직 자리 갯수가 계산되지 않았으므로 0으로 지정한다.
        fn possible_rows(row: &Vec<char>) -> std::collections::HashMap<u32, u32> {
            let mut ret = Vec::<u32>::new();
            
            for c in row.iter() {
                if *c == '#' {
                    if ret.is_empty() {
                        ret.push(0);
                    }
                    else {
                        for each in ret.iter_mut() {
                            *each <<= 1;
                        }
                    }
                }
                else if *c == '.' {
                    if ret.is_empty() {
                        ret.push(1);
                        ret.push(0);
                    }
                    else {
                        let mut temp = Vec::<u32>::new();
                        for each in ret.iter() {
                            if each & 1 == 0 {
                                temp.push(*each << 1);
                                temp.push((*each << 1) | 1);
                            }
                            else {
                                temp.push(*each << 1);
                            }
                        }
                        ret = temp;
                    }
                }
            }

            ret.iter().map(|n| (*n, 0u32)).collect()
        }

        // 입력된 줄에서 지정된 열이 학생이 앉아있는 자리인지를 얻는다.
        fn get_bit(num: &u32, bit: u8) -> bool {
            num & (1 << bit) != 0
        }

        // 입력된 줄에서 학생이 앉아있는 자리 갯수를 구한다.
        fn count_of_bit(mut num: u32) -> u32 {
            let mut count = 0;
            while num > 0 {
                if num & 1 == 1 {
                    count += 1;
                }
                num >>= 1;
            }
            count
        }

        // 이전 줄과 현재 줄을 비교해서, 문제의 조건에 부합하는지 확인한다.
        fn compare_rows(prior_num: &u32, cur_num: &u32) -> bool {
            for i in 0..8 {
                if Solution::get_bit(cur_num, i) {
                    if i > 0 && Solution::get_bit(prior_num, i-1) {
                        return false
                    }

                    if i < 7 && Solution::get_bit(prior_num, i+1) {
                        return false
                    }
                }
            }

            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maximum_students_taking_exam::first_try::Solution;

    #[test]
    fn example() {
        {
            let seats = vec![
                vec!['#','.','#','#','.','#'],
                vec!['.','#','#','#','#','.'],
                vec!['#','.','#','#','.','#']
            ];
            let expected = 4;
            assert_eq!(Solution::max_students(seats), expected);
        }

        {
            let seats = vec![
                vec!['.','#'],
                vec!['#','#'],
                vec!['#','.'],
                vec!['#','#'],
                vec!['.','#'],
            ];
            let expected = 3;
            assert_eq!(Solution::max_students(seats), expected);
        }

        {
            let seats = vec![
                vec!['#','.','.','.','#'],
                vec!['.','#','.','#','.'],
                vec!['.','.','#','.','.'],
                vec!['.','#','.','#','.'],
                vec!['#','.','.','.','#'],
            ];
            let expected = 10;
            assert_eq!(Solution::max_students(seats), expected);
        }
    }
}