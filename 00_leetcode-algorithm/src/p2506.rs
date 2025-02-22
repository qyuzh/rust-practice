pub fn similar_pairs(words: Vec<String>) -> i32 {
    let masks = words.into_iter().map(|w| {
        let mut t = 0u32;
        w.as_bytes().iter().for_each(|&c| {
            t |= 1 << (c - b'a');
        });
        t
    });
    let mut ans = 0;
    let mut ht = std::collections::HashMap::<u32, i32>::new();
    for m in masks {
        let s = ht.entry(m).or_insert(0);
        ans += *s;
        *s += 1;
    }
    ans
}
