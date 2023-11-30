pub fn close_strings(word1: String, word2: String) -> bool {
    let mut feat1 = get_feat(word1);
    let mut feat2 = get_feat(word2);

    for (&x, &y) in feat1.iter().zip(feat2.iter()) {
        if x == 0 && y != 0 || x != 0 && y == 0 {
            return false;
        }
    }

    feat1.sort();
    feat2.sort();

    feat1 == feat2
}

fn get_feat(s: String) -> [i32; 26] {
    let mut feat1 = [0; 26];
    for &x in s.as_bytes() {
        feat1[(x - b'a') as usize] += 1;
    }
    feat1
}
