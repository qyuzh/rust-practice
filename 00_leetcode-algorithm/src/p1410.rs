pub fn entity_parser(text: String) -> String {
    let ht: std::collections::HashMap<&str, &str> = [
        ("quot", "\""),
        ("apos", "'"),
        ("amp", "&"),
        ("gt", ">"),
        ("lt", "<"),
        ("frasl", "/"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut ans = String::new();
    let mut i = 0;
    let bytes = text.as_bytes();

    // case 1: &xxx;
    // case 2: &&xxx;
    // case 3: &&&
    while i < bytes.len() {
        if bytes[i] == b'&' {
            let mut j = i + 1;
            while i < bytes.len() && bytes[i] != b';' {
                // case: &&gt; -> &>
                if bytes[i] == b'&' {
                    ans.push_str(unsafe { std::str::from_utf8_unchecked(&bytes[j - 1..i]) });
                    j = i + 1;
                }
                i += 1;
            }
            let str = unsafe { std::str::from_utf8_unchecked(&bytes[j..i]) };
            if ht.contains_key(str) {
                ans.push_str(ht.get(str).unwrap());
            } else {
                ans.push('&');
                ans.push_str(str);
                if i != bytes.len() {
                    // case: last char is Not ';'
                    ans.push(';');
                }
            }
        } else {
            ans.push(bytes[i] as char);
        }
        i += 1;
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::p1410::entity_parser;

    #[test]
    fn test() {
        let t = entity_parser("&&&gt;".to_string());
        assert_eq!(t, "&&>");

        let t = entity_parser("&&&".to_string());
        assert_eq!(t, "&&&");
    }
}
