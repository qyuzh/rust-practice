///
pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
    let mut k = 0;
    let mut nums = vec![];
    let mut ans = vec![];
    for w in words.iter() {
        if b'0' <= w.as_bytes()[0] && w.as_bytes()[0] <= b'9' {
            nums.push(w);
            k = 0;
        } else {
            k += 1;
            if nums.len() as i32 - k >= 0 {
                ans.push(nums[nums.len() - k as usize].parse::<i32>().unwrap());
            } else {
                ans.push(-1);
            }
        }
    }
    ans
}

///
pub fn get_words_in_longest_subsequence(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let mut ans = vec![];
    let mut pre = groups[0];
    ans.push(words[0].clone());
    for (idx, &v) in groups.iter().enumerate().skip(1) {
        if v != pre {
            ans.push(words[idx].clone());
        }
        pre = v;
    }
    ans
}

/// 1. the same idea to the problem B
/// 2. dfs
pub fn get_words_in_longest_subsequence_c(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    todo!()
}

const MOD: i32 = 1e9 as i32 + 7;

/// 1. sort ascending
/// 2. 
pub fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut ans = 0;
    let mut k = l;
    while k <= r {
        ans = (ans + helper_d(&nums, k)) % MOD;
        k += 1;
    }
    ans
}

fn helper_d(nums: &Vec<i32>, val: i32) -> i32 {
    let mut cnt = 0;
    let mut sum = 0;
    let mut i = 0;
    let mut j = 0;
    while i < j && j < nums.len() {
        if sum < val {
            sum += nums[j];
            j += 1;
            continue;
        }
        if sum > val {
            sum -= nums[i];
            i += 1;
            continue;
        }
        cnt += 1
    }
    cnt
}