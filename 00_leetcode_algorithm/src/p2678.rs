/// array of strings `details`
pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut cnt = 0;
    for v in details.iter() {
        let v = v.as_bytes();
        let num = (v.get(11).unwrap() - b'0') * 10 + v.get(12).unwrap() - b'0';
        if num > 60 {
            cnt += 1
        }
    }
    cnt
}
