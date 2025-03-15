const MAPPINGS: [&str; 10] = [
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    fn dfs(bytes: &[u8], i: usize, path: &mut String, res: &mut Vec<String>) {
        if i == bytes.len() {
            res.push(path.clone());
            return;
        }
        for c in MAPPINGS[(bytes[i] - b'0') as usize].chars() {
            path.push(c);
            dfs(bytes, i + 1, path, res);
            path.pop();
        }
    }

    let bytes = digits.as_bytes();
    let mut res = vec![];
    let mut s = String::new();
    dfs(bytes, 0, &mut s, &mut res);

    res
}
