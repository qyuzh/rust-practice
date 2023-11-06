/// binary search in O(logn)
pub fn h_index(citations: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, citations.len()); // Attention of right bound
    while l < r {
        let mid = (l + r) >> 1;
        if citations[mid] as usize >= citations.len() - mid {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    (citations.len() - l) as i32
}

#[cfg(test)]
mod test {
    use crate::p275::h_index;

    #[test]
    fn test_normal() {
        assert_eq!(h_index(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(h_index(vec![1, 2, 100]), 2);
    }

    #[test]
    fn test_0() {
        assert_eq!(h_index(vec![0]), 0);
        assert_eq!(h_index(vec![0, 0, 0]), 0);
    }
}
