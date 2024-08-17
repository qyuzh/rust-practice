pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
    use std::collections::HashMap;
    let bytes = word.as_bytes();
    let mut ht = HashMap::<&[u8], usize>::new();
    bytes.chunks(k as usize).for_each(|c| {
        ht.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });
    let max_cnt = *ht.values().max().unwrap();
    (word.len() / k as usize - max_cnt) as i32
}
