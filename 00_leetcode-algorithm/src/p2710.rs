use std::mem::ManuallyDrop;

pub fn remove_trailing_zeros(num: String) -> String {
    if num.len() <= 1 {
        return num;
    }
    let mut t = num.into_bytes();
    while let Some(n) = t.last() {
        if *n == b'0' && t.len() > 1 {
            t.pop();
        } else {
            break;
        }
    }
    // SAFETY: t is a valid String, and we just pop some chars from it
    unsafe {
        // avoid to double-free
        let mut t = std::mem::ManuallyDrop::new(t);
        String::from_raw_parts(t.as_mut_ptr(), t.len(), t.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_trailing_zeros() {
        assert_eq!(remove_trailing_zeros("512301200".into()), "5123012");
        assert_eq!(remove_trailing_zeros("0".into()), "0");
        assert_eq!(remove_trailing_zeros("00".into()), "0");
    }
}
