/// runs in O(n)/O(n)
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut k = None;
    for (i, &x) in nums.iter().enumerate() {
        if x >= 0 {
            break;
        }
        k = Some(i);
    }

    if k.is_none() {
        return nums.into_iter().map(|x| x * x).collect();
    }

    if k.is_some_and(|k| k == nums.len() - 1) {
        return nums.into_iter().rev().map(|x| x * x).collect();
    }

    let mut i = k;
    let mut j = k.map(|x| x + 1);

    let mut ans = Vec::with_capacity(nums.len());

    'a: while let (Some(mut ii), Some(mut jj)) = (i, j) {
        while nums[ii] * nums[ii] <= nums[jj] * nums[jj] {
            ans.push(nums[ii] * nums[ii]);
            if ii == 0 {
                i = None;
                break 'a;
            }
            ii -= 1;
        }
        i = Some(ii);

        while nums[ii] * nums[ii] > nums[jj] * nums[jj] {
            ans.push(nums[jj] * nums[jj]);
            if jj == nums.len() - 1 {
                j = None;
                break 'a;
            }
            jj += 1;
        }
        j = Some(jj);
    }

    while let Some(ii) = i {
        ans.push(nums[ii] * nums[ii]);
        if ii > 0 {
            i = Some(ii - 1);
        } else {
            i = None;
        }
    }

    while let Some(jj) = j {
        ans.push(nums[jj] * nums[jj]);
        if jj < nums.len() - 1 {
            j = Some(jj + 1);
        } else {
            j = None;
        }
    }

    ans
}

pub fn sorted_squares2(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut k = 0;
    for (i, &x) in nums.iter().enumerate() {
        if x >= 0 {
            break;
        }
        k = i + 1;
    }

    let lb = 0; // left_bound
    let rb = n + 1; // right_bound

    let mut i = k;
    let mut j = k + 1;
    let mut ans = Vec::with_capacity(n);
    while i > lb && j < rb {
        while i > lb && nums[i - 1] * nums[i - 1] <= nums[j - 1] * nums[j - 1] {
            ans.push(nums[i - 1] * nums[i - 1]);
            i -= 1;
        }
        if i == 0 {
            break;
        }
        while j < rb && nums[i - 1] * nums[i - 1] > nums[j - 1] * nums[j - 1] {
            ans.push(nums[j - 1] * nums[j - 1]);
            j += 1;
        }
    }

    while i > lb {
        ans.push(nums[i - 1] * nums[i - 1]);
        i -= 1;
    }

    while j < rb {
        ans.push(nums[j - 1] * nums[j - 1]);
        j += 1;
    }

    ans
}
