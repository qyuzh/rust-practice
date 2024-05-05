pub fn is_valid(word: String) -> bool {
    let bytes = word.as_bytes();

    let mut flag1 = false;
    let mut flag2 = false;

    for &c in bytes.iter() {
        if !c.is_ascii_alphanumeric() {
            return false;
        }

        if c.is_ascii_digit() {
            continue;
        }

        if [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'].contains(&c) {
            flag1 = true;
        } else {
            flag2 = true;
        }
    }

    flag1 && flag2 && bytes.len() >= 3
}

pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut ht = HashMap::new();

    for i in (0..word.len()).step_by(k as usize) {
        let t = &word[i..(i + k as usize)];
        ht.entry(t).and_modify(|v| *v += 1).or_insert(1);
    }

    let max_v = *ht.values().max().unwrap();

    word.len() as i32 / k - max_v
}

pub fn min_anagram_length(s: String) -> i32 {
    let n = s.len();
    let mut hts = vec![vec![0; s.len() + 1]; 26];

    for (idx, &b) in s.as_bytes().iter().enumerate() {
        let c = (b - b'a') as usize;
        for ht in hts.iter_mut() {
            ht[idx + 1] = ht[idx];
        }
        hts[c][idx + 1] += 1;
    }

    for k in 1..=n {
        if n % k != 0 {
            continue;
        }

        let mut flag1 = true;
        for j in (k * 2..=n).step_by(k) {
            let mut flag = true;
            for ht in hts.iter() {
                if ht[j] * k != ht[k] * j {
                    flag = false;
                    break;
                }
            }
            if !flag {
                flag1 = false;
                break;
            }
        }

        if flag1 {
            return k as i32;
        }
    }

    unreachable!()
}

pub fn min_cost_to_equalize_array(mut nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
    unimplemented!("refer to cpp.cc#minCostToEqualizeArray2")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_anagram_length() {
        let t = min_anagram_length("abba".into());
        assert_eq!(t, 2)
    }
}
