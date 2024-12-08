pub fn maximum_length_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut ans = 0;
    for i in 0..bytes.len() {
        for j in i..bytes.len() {
            let mut cnts = [0; 26];
            for k in i..=j {
                cnts[(bytes[k] - b'a') as usize] += 1;
            }
            let mut valid = true;
            for &count in &cnts {
                if count > 2 {
                    valid = false;
                    break;
                }
            }
            if valid {
                ans = ans.max(j - i + 1)
            }
        }
    }
    ans as i32
}

pub fn min_operations(k: i32) -> i32 {
    let mut max_v = 1;
    let mut sum = 1;

    let mut ans = 0;
    loop {
        if sum >= k {
            break;
        }
        let left = k - sum;
        let cnt_add = left / (max_v + 1) + if left % (max_v + 1) == 0 { 0 } else { 1 };
        let cnt_dup = left / (max_v) + if left % (max_v) == 0 { 0 } else { 1 };
        if cnt_dup <= cnt_add {
            sum += max_v
        } else {
            sum += 1;
            max_v += 1;
        }
        ans += 1;
    }

    ans
}

/// runs in O(nlogn)/O(n) using multiset
///
/// `nums[i]` := the i-th ID \
/// `freq[i]` := the freq of ID `nums[i]`, add if `freq[i]` is positive, subtract otherwise \
/// `ans[i]` := the count of most frequent ID
pub fn most_frequent_ids(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
    use std::collections::{BTreeMap, HashMap};

    let mut cnt: HashMap<i32, i64> = HashMap::new();

    let mut ms: BTreeMap<i64, i32> = BTreeMap::new(); // simulate multiset in C++

    let mut ans = vec![0; nums.len()];

    for (idx, (&id, &fr)) in nums.iter().zip(freq.iter()).enumerate() {
        // delete old cnt_x
        if let Some(total_fr) = cnt.get(&id) {
            if let Some(t) = ms.get_mut(total_fr) {
                if *t == 1 {
                    ms.remove(total_fr);
                } else {
                    *t -= 1;
                }
            }
        }

        let fr = fr as i64;

        // add new new_cnt_x
        let new_cnt_x = cnt.entry(id).and_modify(|v| *v += fr).or_insert(fr);
        ms.entry(*new_cnt_x).and_modify(|v| *v += 1).or_insert(1);

        ans[idx] = *ms.last_key_value().unwrap().0; // SAFETY: we had inserted a kv before(line-2)
    }

    ans
}

/// runs in O(nlogn)/O(n) using lazy-deleted max heap
pub fn most_frequent_ids2(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
    use std::collections::{BinaryHeap, HashMap};

    let mut cnt: HashMap<i32, i64> = HashMap::new(); // id -> total frequency

    let mut max_h = BinaryHeap::new(); // (total frequency, id)

    let mut ans = vec![0; nums.len()];

    for (idx, (&id, &fr)) in nums.iter().zip(freq.iter()).enumerate() {
        let fr = fr as i64;

        let new_total_fr = cnt.entry(id).and_modify(|v| *v += fr).or_insert(fr);

        max_h.push((*new_total_fr, id));

        // find the max total_fr that is "valid"
        while let Some((total_fr, id)) = max_h.peek() {
            // determines if it's valid
            // SAFETY: we had inserted kv before
            if *total_fr == *cnt.get(id).unwrap() {
                break;
            }
            max_h.pop();
        }

        ans[idx] = max_h.peek().unwrap().0; // SAFETY:
    }

    ans
}
