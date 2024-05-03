//! job_scheduling2 is the right dp methods that run in O(nlogn)/O(n)

/// brute-force by emulation
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut selected = std::collections::HashMap::new();

    let mut ids: Vec<usize> = (0..start_time.len()).collect();
    ids.sort_unstable_by(|&i, &j| start_time[i].cmp(&start_time[j]));

    dfs(&start_time, &end_time, &profit, &mut selected, &ids, 1, 0)
}

/// brute-force by emulation
fn dfs(
    start_time: &[i32],
    end_time: &[i32],
    profit: &[i32],
    selected: &mut std::collections::HashMap<usize, i32>,
    ids: &[usize],
    time: usize,
    idx: usize,
) -> i32 {
    let n = start_time.len();

    if idx >= n {
        return 0;
    }

    if selected.contains_key(&time) {
        return *selected.get(&time).unwrap();
    }

    let mut max_v = 0;

    for i in idx..n {
        if start_time[ids[i]] as usize >= time {
            let v = dfs(
                start_time,
                end_time,
                profit,
                selected,
                ids,
                end_time[ids[i]] as usize,
                i + 1,
            ) + profit[ids[i]];
            max_v = max_v.max(v);
        }
    }

    selected.insert(time, max_v);

    max_v
}

/// dp in O(nlogn)/O(n)
pub fn job_scheduling2(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let n = start_time.len();

    let mut jobs = Vec::with_capacity(n);

    for i in 0..n {
        jobs.push((end_time[i] as usize, start_time[i] as usize, profit[i]));
    }

    jobs.sort_unstable();

    let mut f = vec![0; n + 1];
    f[0] = 0;
    for i in 0..n {
        let j = upper_bound(&jobs[0..i], jobs[i].1);
        f[i + 1] = f[i].max(f[j] + jobs[i].2);
    }

    f[n]
}

/// run in O(logn)
fn upper_bound(jobs: &[(usize, usize, i32)], time: usize) -> usize {
    use std::cmp::Ordering::{Greater, Less};
    match jobs.binary_search_by(|i| if i.0 <= time { Less } else { Greater }) {
        Ok(idx) => unreachable!(), // we are not return Equal in the closure
        Err(idx) => idx,
    }
}

#[test]
fn test_upper_bound() {
    assert_eq!(upper_bound(&[(1, 0, 0), (3, 0, 0)], 4), 2);
    assert_eq!(upper_bound(&[(1, 0, 0), (3, 0, 0)], 0), 0);
    assert_eq!(upper_bound(&[(1, 0, 0), (3, 0, 0)], 2), 1);
    assert_eq!(upper_bound(&[(1, 0, 0), (3, 0, 0)], 1), 1);
    assert_eq!(upper_bound(&[(1, 0, 0), (1, 0, 0), (3, 0, 0)], 1), 2);
}
