///
pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![i32::MAX >> 3; n]; n];
    for e in edges.iter() {
        g[e[0] as usize][e[1] as usize] = e[2];
        g[e[1] as usize][e[0] as usize] = e[2];
    }
    let mut ans = n - 1;
    let mut min = n;
    for i in (0..n).rev() {
        let t = dfs(i, &g, n, distance_threshold);
        if t < min {
            min = t;
            ans = i;
        }
    }
    ans as i32
}

fn dfs(node: usize, g: &Vec<Vec<i32>>, n: usize, th: i32) -> usize {
    let mut vis = vec![false; n];
    let mut dis = vec![i32::MAX >> 3; n];
    dis[node] = 0;
    vis[node] = true;
    let mut nxt = node;
    for _ in 1..n {
        let mut min = i32::MAX;
        let mut t_next = 0;
        for i in 0..n {
            if vis[i] {
                continue;
            }
            if dis[i] > dis[nxt] + g[nxt][i] {
                dis[i] = dis[nxt] + g[nxt][i];
            }
            if min > dis[i] {
                min = dis[i];
                t_next = i;
            }
        }
        nxt = t_next;
        vis[nxt] = true;
    }
    dis.iter()
        .fold(0, |acc, &v| acc + if v <= th { 1 } else { 0 })
}

#[cfg(test)]
mod test {
    use crate::p1334::find_the_city;

    #[test]
    fn test() {
        let edges: Vec<Vec<i32>> = [
            [0, 1, 10],
            [0, 2, 1],
            [2, 3, 1],
            [1, 3, 1],
            [1, 4, 1],
            [4, 5, 10],
        ]
        .into_iter()
        .map(|v| v.to_vec())
        .collect();
        assert_eq!(find_the_city(6, edges, 20), 5, "case 1");
    }

    #[test]
    fn test_2() {
        let edges: Vec<Vec<i32>> = [
            [0, 1, 2],
            [0, 4, 8],
            [1, 2, 3],
            [1, 4, 2],
            [2, 3, 1],
            [3, 4, 1],
        ]
        .into_iter()
        .map(|v| v.to_vec())
        .collect();
        assert_eq!(find_the_city(5, edges, 2), 0, "case 1");
    }
}
