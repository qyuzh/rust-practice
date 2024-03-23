//! the score of subarray: min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1)
//! a good subarray: where i <= k <= j

/// runs in O(n^2)/O(1) -> TLE
pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;

    let mut ans = 0;
    for idx in 0..nums.len() {
        // suppose that nums[idx] is the min
        let (l, r) = find_boundary(&nums, idx);
        if k < l || k > r {
            continue;
        }
        let score = (r - l + 1) as i32 * nums[idx];
        ans = ans.max(score);
    }

    ans
}

/// runs in O(n)/O(1)
fn find_boundary(nums: &Vec<i32>, idx: usize) -> (usize, usize) {
    let mut l = idx;
    while l > 0 && nums[l] >= nums[idx] {
        l -= 1;
    }
    // edge case when l == 0
    if l > 0 || l == 0 && nums[l] < nums[idx] {
        l += 1;
    }

    let mut r = idx;
    while r < nums.len() && nums[r] >= nums[idx] {
        r += 1;
    }
    r -= 1;

    (l, r)
}

/// runs in O(n)/O(n), in which n = nums.len()
/// mono-stack
pub fn maximum_score2(nums: Vec<i32>, k: i32) -> i32 {
    let left = find_boundarys(&nums, nums.iter().enumerate(), -1);
    let right = find_boundarys(&nums, nums.iter().enumerate().rev(), nums.len() as i32);

    let mut ans = 0;

    for (idx, (&l, &r)) in left.iter().zip(right.iter()).enumerate() {
        if l < k && k < r {
            ans = ans.max((r - l - 1) * nums[idx]);
        }
    }

    ans
}

/// runs in O(n)/O(n), in which n = nums.len()
fn find_boundarys<'a, T>(nums: &Vec<i32>, iter: T, default: i32) -> Vec<i32>
where
    T: Iterator<Item = (usize, &'a i32)>,
{
    let mut b = vec![default; nums.len()];
    let mut st = vec![];
    for (i, &x) in iter {
        while !st.is_empty() && x <= nums[*st.last().unwrap()] {
            st.pop();
        }
        if let Some(&k) = st.last() {
            b[i] = k as i32;
        }
        st.push(i)
    }
    b
}

/// runs in O(n)/O(1)
fn maximum_score3(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;

    let (mut ans, mut min_h) = (nums[k], nums[k]);

    let (mut i, mut j) = (k, k);
    for _ in 0..(n - 1) {
        if j == n - 1 || (i > 0 && nums[i - 1] > nums[j + 1]) {
            i -= 1;
            min_h = min_h.min(nums[i]);
        } else {
            j += 1;
            min_h = min_h.min(nums[j]);
        }
        ans = ans.max(min_h * (j - i + 1) as i32);
    }

    ans
}

#[cfg(test)]
mod test {

    #[test]
    fn test_maximum_score2() {
        use super::maximum_score2;
        assert_eq!(maximum_score2([1, 4, 3, 7, 4, 5].into(), 3), 15);
    }
}
