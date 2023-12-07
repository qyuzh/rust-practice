/// get answer in O(nm)/O(n), in which m = price.len()
pub fn minimum_total_price(
    n: i32,
    edges: Vec<Vec<i32>>,
    price: Vec<i32>,
    trips: Vec<Vec<i32>>,
) -> i32 {
    // 1. construct tree
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in edges.iter() {
        g[e[0] as usize].push(e[1] as usize);
        g[e[1] as usize].push(e[0] as usize);
    }

    // 2. update the price of every node on tree in O(nm)/O(n)
    let mut cnt = vec![0; n];
    for t in trips.iter() {
        dfs(t[0] as usize, t[1] as usize, t[0] as usize, &mut cnt, &g);
    }

    // 3. dp in O(n)/O(n)
    let (o, h) = dp(0, 0, &cnt, &g, &price);
    o.min(h)
}

fn dfs(x: usize, end: usize, fa: usize, cnt: &mut Vec<i32>, g: &Vec<Vec<usize>>) -> bool {
    if x == end {
        cnt[x] += 1;
        return true;
    }
    for &y in g[x].iter() {
        if y != fa && dfs(y, end, x, cnt, g) {
            cnt[x] += 1;
            return true;
        }
    }
    false
}

fn dp(x: usize, fa: usize, cnt: &Vec<i32>, g: &Vec<Vec<usize>>, prices: &Vec<i32>) -> (i32, i32) {
    let mut origin = prices[x] * cnt[x];
    let mut halved = origin >> 1;
    for &y in g[x].iter() {
        if y != fa {
            let (o, h) = dp(y, x, cnt, g, prices);
            origin += o.min(h);
            halved += o;
        }
    }
    (origin, halved)
}
