pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    let mut max_diagonal = 0;
    let mut max_area = 0;
    dimensions.iter().for_each(|v| {
        let diagonal = v[0] * v[0] + v[1] * v[1];
        if diagonal > max_diagonal {
            max_diagonal = diagonal;
            max_area = v[0] * v[1];
        } else if diagonal == max_diagonal {
            max_area = max_area.max(v[0] * v[1]);
        }
    });
    max_area
}

/// (a, b) rook (c, d) bishop (e, f) queen
pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    // rook和queen在同一条横线上, 且bishop不在这条横线上或者bishop不在两者中间
    if a == e && (c != e || is_not_in_middle(d, b, f)) {
        return 1;
    }

    // rook和queen在同一条纵线上, 且bishop不在这条纵线上或者bishop不在两者中间
    if b == f && (d != f || is_not_in_middle(c, a, e)) {
        return 1;
    }

    // bishop和queen在同一条斜线上, 且rook不再这条斜线上或者rook不在两者中间
    if c + d == e + f && (a + b != e + f || is_not_in_middle(a, c, e)) {
        return 1;
    }

    // bishop和queen在同一条反斜线上, 且rook不再这条反斜线上或者rook不在两者中间
    if c - d == e - f && (a - b != e - f || is_not_in_middle(a, c, e)) {
        return 1;
    }

    2
}

fn is_not_in_middle(z: i32, x: i32, y: i32) -> bool {
    z < x.min(y) || z > x.max(y)
}

pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ht1 = std::collections::HashSet::new();
    let mut ht2 = std::collections::HashSet::new();
    nums1.iter().for_each(|v| {
        ht1.insert(*v);
    });
    nums2.iter().for_each(|v| {
        ht2.insert(*v);
    });

    remove(&mut ht1, &ht2, nums1.len() / 2);
    remove(&mut ht2, &ht1, nums1.len() / 2);

    ht1.union(&ht2).count() as i32
}

/// Greedy: 首先移除ht1中在ht2中存在的元素, 然后移除多余的元素, 使得ht1中的元素数量小于等于n
fn remove(
    ht1: &mut std::collections::HashSet<i32>,
    ht2: &std::collections::HashSet<i32>,
    n: usize,
) {
    if ht1.len() <= n {
        return;
    }

    // 移除ht1中在ht2中存在的元素至ht1中元素的数量小于等于n
    let mut removes = Vec::with_capacity(ht1.len() - n);
    for v in ht1.iter() {
        if ht2.contains(v) {
            removes.push(*v);
            if removes.len() + n == ht1.len() {
                break;
            }
        }
    }
    removes.iter().for_each(|v| {
        ht1.remove(v);
    });

    if ht1.len() <= n {
        return;
    }

    // 经过上述的操作后, ht1中的元素数量仍然大于n, 移除多余的
    removes.clear();
    for v in ht1.iter() {
        removes.push(*v);
        if removes.len() + n == ht1.len() {
            break;
        }
    }
    removes.iter().for_each(|v| {
        ht1.remove(v);
    });
}
