/// Get answer in O(nm)
/// Optimization: We could only keep one word for those words that has the same feature.
pub fn max_product(words: Vec<String>) -> i32 {
    let mut features = vec![0; words.len()];
    for (idx, w) in words.iter().enumerate() {
        let mut feature = 0;
        for &c in w.as_bytes() {
            feature |= 1 << (c - b'a');
        }
        features[idx] = feature;
    }

    let mut ans = 0;
    for (idx, f) in features.iter().enumerate() {
        for j in 0..idx {
            if (f & features[j]) == 0 {
                ans = ans.max(words[idx].len() * words[j].len())
            }
        }
    }

    ans as i32
}
