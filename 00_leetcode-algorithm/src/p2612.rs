use std::collections::{BTreeSet, HashSet, VecDeque};

pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
    let banned: HashSet<i32> = banned.into_iter().collect();
    bfs(n, p, &banned, k)
}

// TLE, O(n*k)
pub fn bfs(n: i32, p: i32, banned: &HashSet<i32>, k: i32) -> Vec<i32> {
    let mut visited = vec![false; n as usize];
    let mut queue = VecDeque::new();
    let mut res = vec![-1; n as usize];
    queue.push_back((p, 0));
    visited[p as usize] = true;
    res[p as usize] = 0;
    while let Some((cur, cnt)) = queue.pop_front() {
        for i in 0..k {
            if cur - i < 0 {
                break;
            }

            if cur + k - i > n {
                continue;
            }

            let p = k - 2 * i - 1 + cur;
            if p >= 0 && p < n && !visited[p as usize] && !banned.contains(&p) {
                visited[p as usize] = true;
                queue.push_back((p, cnt + 1));
                res[p as usize] = cnt + 1;
            }
        }
    }
    res
}

/// O(nlogn)
pub fn min_reverse_operations2(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ts = {
        let mut ts = [BTreeSet::new(), BTreeSet::new()];
        for i in 0..n {
            ts[i as usize % 2].insert(i);
        }
        for b in banned.iter() {
            ts[*b as usize % 2].remove(b);
        }
        ts
    };

    {
        let mut ans = vec![-1; n as usize];
        let mut deleted = Vec::new();

        let mut q = VecDeque::new();
        q.push_back(p);
        ans[p as usize] = 0;
        ts[p as usize % 2].remove(&p);

        while let Some(i) = q.pop_front() {
            let mi = (i - k + 1).max(k - i - 1);
            let mx = (i + k - 1).min(2 * n - k - i - 1);
            let s = &mut ts[mi as usize % 2];

            for &j in s.range(mi..=mx) {
                ans[j as usize] = ans[i as usize] + 1;
                q.push_back(j);
                deleted.push(j);
            }

            for j in deleted.drain(..) {
                s.remove(&j);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_reverse_operations() {
        assert_eq!(min_reverse_operations(4, 0, vec![], 4), vec![0, -1, -1, 1]);
    }
}
