// runs in O(nlogn)/O(1)
pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    nums.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    let mut ans = 0;
    let mut l = nums[0][0];
    let mut r = nums[0][1];
    for car in nums.iter().skip(1) {
        if car[0] > r {
            ans += (r - l) + 1;
            l = car[0];
            r = car[1];
        } else {
            r = car[1].max(r);
        }
    }
    ans += (r - l) + 1; // post-process
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_points() {
        let ret = number_of_points(
            [
                [2, 3].into(),
                [3, 9].into(),
                [5, 7].into(),
                [4, 10].into(),
                [9, 10].into(),
            ]
            .into(),
        );
        assert_eq!(ret, 9)
    }
}
