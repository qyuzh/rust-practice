use std::str::Split;

/// Consider "9,3,4,#,#,1,#,#,2,#,6,#,#"
/// runs in O(n)/O(n)
pub fn is_valid_serialization(preorder: String) -> bool {
    let mut nodes = preorder.split(",");
    preorder_traverse(&mut nodes) && nodes.next().is_none()
}

fn preorder_traverse<'a>(nodes: &mut Split<'a, &str>) -> bool {
    if let Some(s) = nodes.next() {
        if s == "#" {
            return true;
        }

        if !preorder_traverse(nodes) {
            return false;
        }

        if !preorder_traverse(nodes) {
            return false;
        }

        return true;
    }
    false
}

/// runs in O(n)/O(n)
pub fn is_valid_serialization2(preorder: String) -> bool {
    let bytes = preorder.as_bytes();
    let n = bytes.len();

    let mut stk = vec![1];

    let mut i = 0;
    while i < n {
        if stk.is_empty() {
            return false;
        }

        if bytes[i] == b',' {
            i += 1; // skip
        } else if bytes[i] == b'#' {
            *stk.last_mut().unwrap() -= 1;
            if *stk.last().unwrap() == 0 {
                stk.pop();
            }
            i += 1;
        } else {
            // It's a number

            // read a number
            while i < n && bytes[i] != b',' {
                i += 1;
            }

            *stk.last_mut().unwrap() -= 1;
            if *stk.last().unwrap() == 0 {
                stk.pop();
            }

            stk.push(2);
        }
    }

    stk.is_empty()
}

/// runs in O(n)/O(1)
pub fn is_valid_serialization3(preorder: String) -> bool {
    let bytes = preorder.as_bytes();
    let n = bytes.len();

    let mut slots = 1;

    let mut i = 0;
    while i < n {
        if slots == 0 {
            return false;
        }

        if (bytes[i] == b',') {
            i += 1;
        } else if bytes[i] == b'#' {
            slots -= 1;
            i += 1;
        } else {
            // read a number
            while i < n && bytes[i] != b',' {
                i += 1;
            }
            slots += 1;
        }
    }

    slots == 0
}

#[test]
fn test_is_valid_serialization() {
    let ret = is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".into());
    assert_eq!(ret, true);

    let ret = is_valid_serialization("1,#".into());
    assert_eq!(ret, false);

    let ret = is_valid_serialization("9,#,#,1".into());
    assert_eq!(ret, false);
}
