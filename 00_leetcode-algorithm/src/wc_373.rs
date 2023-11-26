pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    for (idx, row) in mat.iter().enumerate() {
        let right = if idx % 2 == 0 { 1 } else { -1 };
        for i in 0..row.len() as i32 {
            let idx = (row.len() as i32 * 55 + i + right * k) as usize % row.len();
            if row[i as usize] != row[idx] {
                return false;
            }
        }
    }
    true
}

pub fn beautiful_substrings(s: String, k: i32) -> i32 {
    const VOWELS: &str = "aeiou";

    let bytes = s.as_bytes();
    let mut prefix_sum = vec![0; bytes.len() + 1];
    for i in 1..=bytes.len() {
        prefix_sum[i] = prefix_sum[i - 1]
            + if VOWELS.contains(bytes[i - 1] as char) {
                1
            } else {
                0
            };
    }

    let mut ans = 0;
    for i in 0..bytes.len() {
        for j in i + 1..=bytes.len() {
            let cnt = prefix_sum[j] - prefix_sum[i];
            if cnt * 2 == j - i && (cnt * cnt) % k as usize == 0 {
                ans += 1
            }
        }
    }

    ans
}

pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let mut p: Vec<usize> = (0..nums.len()).collect();
    p.sort_unstable_by(|&x, &y| nums[x].cmp(&nums[y]));

    let mut ans = vec![0; nums.len()];

    let mut i = 0;
    while i < nums.len() {
        let mut j = i + 1;
        while j < nums.len() && nums[p[j]] - nums[p[j - 1]] <= limit {
            j += 1;
        }

        let mut ids = vec![0; j - i];
        for k in i..j {
            ids[k - i] = p[k];
        }
        ids.sort();

        for k in i..j {
            ans[ids[k - i]] = nums[p[k]];
        }

        i = j;
    }

    ans
}

///   baeyh
/// v:01222
/// c:11123
///
/// if vi - ci = vj - cj, then s(i..j] is beautiful when just consider condition 1.
///
/// version 1, get answer in $O(n * \sqrt(k))/O(n)$
pub fn beautiful_substrings_d(s: String, k: i32) -> i64 {
    let n = s.as_bytes().len();
    let mut valid = vec![];
    for i in 0..k {
        if i * i % k == 0 {
            valid.push(i);
        }
    }

    let mut ans = 0;

    let mut ht = std::collections::HashMap::new();
    ht.insert((0, 0), 1);
    let (mut det, mut cnt) = (0, 0);
    for i in 0..n {
        if is_vowel(s.as_bytes()[i]) {
            det += 1;
            cnt += 1;
        } else {
            det -= 1;
        }

        for &x in valid.iter() {
            let t = (cnt - x + k) % k;
            if let Some(&t) = ht.get(&(det, t)) {
                ans += t as i64;
            }
        }

        ht.entry((det, cnt % k))
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    ans
}

#[inline]
fn is_vowel(bit: u8) -> bool {
    // 1065233 = (1 << 4) + (1 << 8) + (1 << 14) + (1 << 20)
    ((1065233 >> (bit - b'a')) & 1) == 1
}

#[cfg(test)]
mod test {
    use crate::wc_373::{beautiful_substrings_d, lexicographically_smallest_array};

    #[test]
    fn test_c() {
        assert_eq!(
            lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn test_d() {
        let t = beautiful_substrings_d("baeyh".to_string(), 2);
        assert_eq!(t, 2, "Test d: case 1");
    }
}
