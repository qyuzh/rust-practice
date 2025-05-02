/// Runs in `O(n)` time and `O(n)` space.
pub fn push_dominoes(dominoes: String) -> String {
    let n = dominoes.len();
    let bytes = dominoes.as_bytes();

    let mut rights = vec![-1; n];
    let mut last_right = -1;
    for i in 0..n {
        if bytes[i] == b'R' {
            last_right = i as i32;
        } else if bytes[i] == b'L' {
            last_right = -1;
        }
        rights[i] = last_right;
    }

    let mut lefts = vec![-1; n];
    let mut last_left = -1;
    for i in (0..n).rev() {
        if bytes[i] == b'L' {
            last_left = i as i32;
        } else if bytes[i] == b'R' {
            last_left = -1;
        }
        lefts[i] = last_left;
    }

    let mut result = vec![b'.'; n];
    for i in 0..n {
        if bytes[i] == b'R' {
            result[i] = b'R';
        } else if bytes[i] == b'L' {
            result[i] = b'L';
        } else if rights[i] != -1 && (lefts[i] == -1 || i as i32 - rights[i] < lefts[i] - i as i32)
        {
            result[i] = b'R';
        } else if lefts[i] != -1
            && (rights[i] == -1 || lefts[i] - (i as i32) < i as i32 - rights[i])
        {
            result[i] = b'L';
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}
