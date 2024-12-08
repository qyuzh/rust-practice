/// # Lemmas
/// 1. After topological sort, the remaining must be circles of size of either 2 or more.
///    Every circle must be of single hole.
///
/// # Complexity
/// Get answer in O(n)/O(n)
pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    // 1. construct the graph
    let mut g = vec![vec![]; favorite.len()];
    let mut g_r = vec![vec![]; favorite.len()];
    let mut indegs = vec![0; favorite.len()];
    for (i, &x) in favorite.iter().enumerate() {
        g[i].push(x as usize);
        indegs[x as usize] += 1;
        g_r[x as usize].push(i);
    }

    let removed = topological_sort(&g, &mut indegs);
    let mut len = 0;
    let mut len2 = 0;

    let mut vis = vec![false; g.len()];
    for (i, &r) in removed.iter().enumerate() {
        if !r && !vis[i] {
            let size = get_circle_size(&g, &removed, &mut vis, i, true);
            if size <= 2 {
                len2 += get_size_for_circle_size_of_2(&g_r, &removed, &mut vis, i);
            } else {
                len = len.max(size);
                get_circle_size(&g, &removed, &mut vis, i, false);
            }
        }
    }

    len.max(len2) as i32
}

fn topological_sort(g: &[Vec<usize>], indegs: &mut [i32]) -> Vec<bool> {
    let mut removed = vec![false; indegs.len()];
    let mut q = std::collections::VecDeque::new();
    for (i, &x) in indegs.iter().enumerate() {
        if x == 0 {
            q.push_back(i);
            removed[i] = true;
        }
    }

    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        for &nxt in g[node].iter() {
            indegs[nxt] -= 1;
            if indegs[nxt] == 0 {
                removed[nxt] = true;
                q.push_back(nxt);
            }
        }
    }

    removed
}

/// get circle size and set visited state
/// # Params
/// - *reset*, true for resetting `vis[node]` to `false`, otherwise set it to `true`.
fn get_circle_size(
    g: &[Vec<usize>],
    removed: &[bool],
    vis: &mut Vec<bool>,
    node: usize,
    reset: bool,
) -> usize {
    let mut size = 1;
    vis[node] = true;
    for &nxt in g[node].iter() {
        if !removed[nxt] && !vis[nxt] {
            size += get_circle_size(g, removed, vis, nxt, reset);
        }
    }
    if reset {
        vis[node] = false;
    }
    size
}

fn get_size_for_circle_size_of_2(
    g: &[Vec<usize>],
    removed: &[bool],
    vis: &mut Vec<bool>,
    node: usize,
) -> usize {
    let mut another = usize::MAX;
    for &nxt in g[node].iter() {
        if !removed[nxt] {
            another = nxt;
        }
    }
    dfs(g, vis, node, another) + dfs(g, vis, another, node)
}

fn dfs(g: &[Vec<usize>], vis: &mut Vec<bool>, node: usize, another: usize) -> usize {
    vis[node] = true;
    let mut max = 0;
    for &nxt in g[node].iter() {
        if !vis[nxt] && nxt != another {
            max = max.max(dfs(g, vis, nxt, usize::MAX));
        }
    }
    max + 1
}

#[cfg(test)]
mod test {
    use crate::p2127::maximum_invitations;

    #[test]
    fn test_maximum_invitations() {
        assert_eq!(
            maximum_invitations(vec![2, 2, 1, 2]),
            3,
            "test_maximum_invitations 1"
        );
        assert_eq!(
            maximum_invitations(vec![1, 2, 0]),
            3,
            "test_maximum_invitations 2"
        );
        assert_eq!(
            maximum_invitations(vec![3, 0, 1, 4, 1]),
            4,
            "test_maximum_invitations 3"
        );
    }
}
