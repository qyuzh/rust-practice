use std::collections::HashMap;

pub fn max_coins(nums: Vec<i32>) -> i32 {
    Solution312::new(nums).max_coins()
}

struct Solution312 {
    memo: HashMap<(usize, usize), i32>,
    values: Vec<i32>,
}

impl Solution312 {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut values = Vec::with_capacity(nums.len() + 2);

        values.push(1);
        nums.into_iter().for_each(|v| values.push(v));
        values.push(1);

        Self {
            memo: HashMap::new(),
            values,
        }
    }

    pub fn max_coins(&mut self) -> i32 {
        self.solve(0, self.values.len() - 1)
    }

    fn solve(&mut self, l: usize, r: usize) -> i32 {
        if l + 1 >= r {
            return 0;
        }

        if let Some(t) = self.memo.get(&(l, r)) {
            return *t;
        }

        let mut ret = 0;

        (l + 1..r).for_each(|mid| {
            let mut sum = self.values[l] * self.values[r] * self.values[mid];
            sum += self.solve(l, mid) + self.solve(mid, r);
            ret = ret.max(sum);
        });

        self.memo.insert((l, r), ret);

        ret
    }
}

mod solution1 {
    use std::collections::HashMap;

    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        dfs(&nums[0..nums.len()], &nums[nums.len()..], &mut memo)
    }

    fn dfs(left: &[i32], right: &[i32], memo: &mut HashMap<Vec<i32>, i32>) -> i32 {
        let now = Vec::from_iter(left.iter().chain(right).copied());

        if now.is_empty() {
            return 0;
        }

        let n = now.len();

        if n == 1 {
            return now[0];
        }

        if n == 2 {
            return now[0] * now[1] + now[0].max(now[1]);
        }

        if let Some(t) = memo.get(&now) {
            return *t;
        }

        let mut ret = (now[0] * now[1] + dfs(&now[0..0], &now[1..], memo))
            .max(now[n - 1] * now[n - 2] + dfs(&now[0..n - 1], &now[n..], memo));

        for i in 1..n - 1 {
            ret = ret.max(now[i] * now[i - 1] * now[i + 1] + dfs(&now[0..i], &now[i + 1..], memo));
        }

        memo.insert(now, ret);

        ret
    }
}

mod solution2 {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let values = new_values(nums);

        let n = values.len();

        let mut dp = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            for j in i + 2..n {
                for mid in i + 1..j {
                    let sum = values[mid] * values[i] * values[j] + dp[i][mid] + dp[mid][j];
                    dp[i][j] = dp[i][j].max(sum);
                }
            }
        }

        dp[0][n - 1]
    }

    fn new_values(nums: Vec<i32>) -> Vec<i32> {
        let mut values = Vec::with_capacity(nums.len() + 2);

        values.push(1);
        nums.into_iter().for_each(|v| values.push(v));
        values.push(1);

        values
    }
}
