/// runs in O(n)/O(n)
pub fn remove_stars(s: String) -> String {
    let mut stack: String = "".into();
    for &c in s.as_bytes() {
        if c == b'*' {
            stack.pop();
        } else {
            stack.push(c as char);
        }
    }
    stack
}
