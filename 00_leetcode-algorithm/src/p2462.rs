use std::{cmp::Reverse, collections::BinaryHeap};

/// total cost to hire k workers
/// Solution: min-heap
pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let candidates = candidates as usize;
    let k = k as usize;

    assert!(
        costs.len() >= k && costs.len() >= candidates,
        "break constrains"
    );

    let mut mh = std::collections::BinaryHeap::new();

    if candidates * 2 <= n {
        for (i, &x) in costs.iter().enumerate().take(candidates) {
            mh.push(Reverse((x, i)));
        }
        for (i, &x) in costs.iter().enumerate().rev().take(candidates) {
            mh.push(Reverse((x, i)));
        }
    } else {
        for (i, &x) in costs.iter().enumerate() {
            mh.push(Reverse((x, i)));
        }
    }

    let mut ret = 0i64;

    let mut l = candidates - 1;
    let mut r = n - candidates;
    for i in 0..k {
        let Reverse((v, idx)) = mh.pop().unwrap();
        ret += v as i64;

        // println!("choose {idx} value {v}");

        if l >= r || l + 1 == r {
            continue;
        }

        if idx <= l {
            l += 1;
            mh.push(Reverse((costs[l], l)));
        } else {
            r -= 1;
            mh.push(Reverse((costs[r], r)));
        }
    }

    ret
}

/// Solution: min-heap
pub fn total_cost2(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let candidates = candidates as usize;
    let k = k as usize;

    assert!(n >= k && n >= candidates, "break constraints");

    // min heap
    let mut mh = std::collections::BinaryHeap::new();

    // double ended iter
    let mut di = costs.iter().enumerate();

    while let Some((i, &x)) = di.next() {
        mh.push(Reverse((x, i, true))); // means (cost, idx, is_left)

        if let Some((i, &x)) = di.next_back() {
            mh.push(Reverse((x, i, false)));
        }

        if i + 1 == candidates {
            break;
        }
    }

    let mut ret: i64 = 0;
    for i in 0..k {
        let Reverse((v, i, is_left)) = mh.pop().unwrap();
        ret += v as i64;

        if is_left {
            if let Some((i, &x)) = di.next() {
                mh.push(Reverse((x, i, true)));
            }
        } else if let Some((i, &x)) = di.next_back() {
            mh.push(Reverse((x, i, false)));
        }
    }

    ret
}

/// 2 min heap
pub fn total_cost3(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let c = candidates as usize;
    let k = k as usize;

    assert!(n >= k && n >= c, "break constraints");

    if c * 2 + k >= n {
        costs.sort_unstable();
        return costs.iter().take(k).map(|&x| x as i64).sum();
    }

    let mut pre = BinaryHeap::new();
    let mut suf = BinaryHeap::new();

    for i in 0..c {
        pre.push(-costs[i]);
        suf.push(-costs[n - 1 - i]);
    }

    let mut ret = 0;

    let mut i = c;
    let mut j = n - 1 - c;
    for _ in 0..k {
        if pre.peek().unwrap() >= suf.peek().unwrap() {
            ret -= pre.pop().unwrap() as i64;
            pre.push(-costs[i]);
            i += 1;
        } else {
            ret -= suf.pop().unwrap() as i64;
            suf.push(-costs[j]);
            j -= 1;
        }
    }

    ret
}
