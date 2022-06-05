pub fn binary_search<T: PartialOrd + Copy>(v: &[T], n: T) -> Option<usize> {
    let mut start = 0isize;
    let mut end = v.len() as isize - 1;

    while start <= end {
        let mid = (start + end) / 2;
        let midv = v[mid as usize];

        if midv == n {
            return Some(mid as usize)
        }
        else if midv < n {
            start = mid + 1;
        }
        else {
            end = mid - 1;
        }
    }

    None
}

pub fn binary_search_recursive<T: PartialOrd + Copy>(v: &[T], n: T) -> Option<usize> {
    let start = 0isize;
    let end = v.len() as isize - 1;

    binary_search_recursive_impl(v, n, start, end)
}

fn binary_search_recursive_impl<T: PartialOrd + Copy>(v: &[T], n: T, start: isize, end: isize) -> Option<usize> {
    if start > end {
        return None
    }

    let mid = (start + end) / 2;
    let midv = v[mid as usize];

    if midv == n {
        Some(mid as usize)
    }
    else if midv < n {
        binary_search_recursive_impl(v, n, mid + 1, end)
    }
    else {
        binary_search_recursive_impl(v, n, start, mid - 1)
    }
}

// n보다 더 큰 값이 올 수 있는 위치.
fn binary_search_upper<T: PartialOrd + Copy>(v: &[T], n: T) -> usize {
    let mut start = 0isize;
    let mut end = v.len() as isize- 1;

    while start <= end {
        let mid = (start + end) / 2;
        let midv = v[mid as usize];

        if midv <= n {
            start = mid + 1;
        }
        else {
            end = mid - 1;
        }
    }

    start as usize
}

#[cfg(test)]
mod tests {

    use super::{binary_search, binary_search_recursive, binary_search_upper};

    #[test]
    fn example() {
        let v = vec![17, 28, 43, 67, 88, 92, 100, 150];
        
        for (i, &n) in v.iter().enumerate() {
            assert_eq!(binary_search(&v, n), Some(i));
        }
        assert_eq!(binary_search(&v, 10), None);
        assert_eq!(binary_search(&v, 55), None);
        assert_eq!(binary_search(&v, 200), None);

        for (i, &n) in v.iter().enumerate() {
            assert_eq!(binary_search_recursive(&v, n), Some(i));
        }
        assert_eq!(binary_search_recursive(&v, 10), None);
        assert_eq!(binary_search_recursive(&v, 55), None);
        assert_eq!(binary_search_recursive(&v, 200), None);

        for (i, n) in v.iter().enumerate() {
            assert_eq!(binary_search_upper(&v, *n), i + 1, "{}, {}", i, n);
        }
        assert_eq!(binary_search_upper(&v, 10), 0);
        assert_eq!(binary_search_upper(&v, 55), 3);
        assert_eq!(binary_search_upper(&v, 200), 8);
    }
}