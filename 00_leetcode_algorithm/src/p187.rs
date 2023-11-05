pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let chars = s.as_bytes();
    if chars.len() < 10 {
        return vec![];
    }
    let mut ans = vec![];
    let mut ht = std::collections::HashMap::new();
    for i in 0..=chars.len() - 10 {
        let s = &chars[i..i + 10];
        match ht.get(s) {
            Some(&t) if t == 1 => {
                ans.push(unsafe { String::from_utf8_unchecked(s.to_vec()) });
            }
            _ => {}
        }
        ht.entry(s).and_modify(|v| *v += 1).or_insert(1);
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::p187::find_repeated_dna_sequences;

    #[test]
    fn test() {
        assert_eq!(
            find_repeated_dna_sequences("AAAAAAAAAAA".to_string()),
            vec!["AAAAAAAAAA".to_string()],
            "test 1"
        );
    }
}
