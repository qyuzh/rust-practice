pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0, 0];
    for x in nums1.iter() {
        if nums2.contains(x) {
            ans[0] += 1;
        }
    }
    for x in nums2.iter() {
        if nums1.contains(x) {
            ans[1] += 1;
        }
    }
    ans
}

pub fn remove_almost_equal_characters(word: String) -> i32 {
    let mut ans = 0;
    let bytes = word.as_bytes();
    let mut i = 1;
    while i < bytes.len() {
        let p = (bytes[i - 1] - b'a') as isize;
        let c = (bytes[i] - b'a') as isize;
        if (p - c).abs() <= 1 {
            ans += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    ans.min(bytes.len() as i32 - ans)
}

/// sliding window
pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;

    let mut ht = std::collections::HashMap::new();
    let (mut l, mut r) = (0, 0); // [l,r)
    while r < nums.len() {
        while r < nums.len() && *ht.entry(nums[r]).and_modify(|v| *v += 1).or_insert(1) <= k {
            r += 1;
        }

        ans = ans.max(r as i32 - l as i32);

        if r == nums.len() {
            break;
        }

        while l < r && nums[l] != nums[r] {
            ht.entry(nums[l]).and_modify(|v| *v -= 1);
            l += 1;
        }
        ht.entry(nums[l]).and_modify(|v| *v -= 1); // nums[l] == nums[r]

        r += 1;
        l += 1;
    }

    ans
}

pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
    let g = Graph::new(roads, n);
    let mut ans = 1;
    let mut mask = 1;
    while mask < (1 << n) {
        if g.check(mask, n as usize, max_distance) {
            ans += 1;
        }
        mask += 1;
    }

    ans
}

struct Graph(Vec<Vec<i32>>);

impl Graph {
    fn new(roads: Vec<Vec<i32>>, n: i32) -> Self {
        let mut g = vec![vec![i32::MAX >> 2; n as usize]; n as usize];
        for r in roads.iter() {
            g[r[0] as usize][r[1] as usize] = g[r[0] as usize][r[1] as usize].min(r[2]);
            g[r[1] as usize][r[0] as usize] = g[r[1] as usize][r[0] as usize].min(r[2]);
        }
        Self(g)
    }

    /// check if it's ok under mask
    fn check(&self, mask: usize, n: usize, max_distance: i32) -> bool {
        let total = mask.count_ones();
        for i in 0..n {
            if (mask >> i) & 1 == 1 && self.max_dist_from(mask, n, i, total) > max_distance {
                return false;
            }
        }
        true
    }

    /// max distance from node to others
    fn max_dist_from(&self, mask: usize, n: usize, mut node: usize, total: u32) -> i32 {
        let mut d = vec![i32::MAX >> 2; n];
        let mut vis = vec![false; n];

        d[node] = 0;
        vis[node] = true;
        let mut cnt = 1;
        while cnt < total {
            let mut t_node = node;
            let mut min = i32::MAX;
            for nxt in 0..n {
                if (mask >> nxt) & 1 == 1 && !vis[nxt] {
                    if d[nxt] > d[node] + self.0[node][nxt] {
                        d[nxt] = d[node] + self.0[node][nxt];
                    }
                    if min > d[nxt] {
                        min = d[nxt];
                        t_node = nxt;
                    }
                }
            }

            node = t_node;
            vis[node] = true;
            cnt += 1;
        }

        d[node]
    }
}
