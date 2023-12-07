//! Use `OnceLock` to implement initialization when first usage

use std::sync::OnceLock;

use regex::Regex;

pub fn log_file_regex() -> &'static Regex {
    static LOG_FILE_REGEX: OnceLock<Regex> = OnceLock::new();
    LOG_FILE_REGEX.get_or_init(|| Regex::new(r"^[0-9]{3,4}$").unwrap())
}

fn main() {
    println!("{:?}", log_file_regex().is_match("123"));
    println!("{:?}", log_file_regex().is_match("1231"));
    println!("{:?}", log_file_regex().is_match("12312"));

    let t = r"123qyuzh";
    let t1 = r##"123"#qyuzh"##;
    println!("{t} {}", t.len());
    println!("{t1} {}", t1.len());
}
