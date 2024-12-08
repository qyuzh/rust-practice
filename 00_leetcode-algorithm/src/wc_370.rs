pub fn find_champion_a(grid: Vec<Vec<i32>>) -> i32 {
    for (i, v) in grid.iter().enumerate() {
        let mut flag = true;
        for (j, &x) in v.iter().enumerate() {
            if x == 0 && i != j {
                flag = false;
            }
        }
        if flag {
            return i as i32;
        }
    }
    0
}

pub fn find_champion_b(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut indegs = vec![0; n as usize];
    let mut ans = -1;
    for e in edges.iter() {
        indegs[e[1] as usize] += 1;
    }
    for (idx, &x) in indegs.iter().enumerate() {
        if x == 0 {
            if ans != -1 {
                return -1;
            }
            ans = idx as i32;
        }
    }
    ans
}

pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
    let mut g = vec![vec![]; values.len()];
    for e in edges.iter() {
        g[e[0] as usize].push(e[1] as usize);
        g[e[1] as usize].push(e[0] as usize);
    }
    values.iter().map(|&v| v as i64).sum::<i64>() - dfs(&g, &values, 0, 0)
}

fn dfs(g: &Vec<Vec<usize>>, values: &Vec<i32>, node: usize, fa: usize) -> i64 {
    let mut ans = 0;
    for &x in g[node].iter() {
        if x != fa {
            ans += dfs(g, values, x, node);
        }
    }
    if ans == 0 || ans > values[node] as i64 {
        return values[node] as i64;
    }
    ans
}

pub fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
    let mut b = nums
        .iter()
        .enumerate()
        .map(|(idx, &v)| v - idx as i32)
        .collect::<Vec<_>>();
    b.sort();
    b.dedup();

    let mut bit = Bit::new(nums.len() + 1);
    let mut ans = i64::MIN;
    for (i, &num) in nums.iter().enumerate() {
        let j = b.partition_point(|&v| v < (num - i as i32)) + 1;
        let f = 0.max(bit.pre_max(j as i32)) + num as i64;
        ans = ans.max(f);
        bit.update(j as i32, f);
    }

    ans
}

struct Bit {
    tree: Vec<i64>,
}

impl Bit {
    pub fn new(n: usize) -> Bit {
        Bit { tree: vec![0; n] }
    }

    pub fn update(&mut self, mut i: i32, val: i64) {
        while (i as usize) < self.tree.len() {
            self.tree[i as usize] = self.tree[i as usize].max(val);
            i += i & -i;
        }
    }

    pub fn pre_max(&mut self, mut i: i32) -> i64 {
        let mut ans = i64::MIN;
        while i > 0 {
            ans = ans.max(self.tree[i as usize]);
            i -= i & -i; // i &= i - 1
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::wc_370::{max_balanced_subsequence_sum, maximum_score_after_operations};

    #[test]
    fn test_a() {}

    #[test]
    fn test_b() {}

    #[test]
    fn test_c() {
        assert_eq!(
            maximum_score_after_operations(vec![vec![0, 1]], vec![2, 1]),
            2,
            "C 1"
        );
    }

    #[test]
    fn test_d() {
        assert_eq!(max_balanced_subsequence_sum(vec![3, 3, 5, 6]), 14, "d 1");
        assert_eq!(max_balanced_subsequence_sum(vec![5, -1, -3, 8]), 13, "d 2");
        assert_eq!(max_balanced_subsequence_sum(vec![-2, -1]), -1, "d 3");
        assert_eq!(
            max_balanced_subsequence_sum(vec![34, 34, 32, 33]),
            65,
            "d 4"
        );
        assert_eq!(
            max_balanced_subsequence_sum(vec![2, 9, -4, 4, 2]),
            11,
            "d 5"
        );
        assert_eq!(max_balanced_subsequence_sum(vec![1, 2, 3, 4, 5]), 15, "d 6");
    }

    #[test]
    fn test_binary_representation() {
        println!("---test_binary_representation---");
        print!("6\n{:#034b}\n{:#34b}\n{:#034b}\n", 6, -6, 6 & -6);
        print!("7\n{:#034b}\n{:#34b}\n{:#034b}\n", 7, -7, 7 & -7);
        print!("8\n{:#034b}\n{:#34b}\n{:#034b}\n", 8, -8, 8 & -8);
        println!("---test_binary_representation---");
    }
}
