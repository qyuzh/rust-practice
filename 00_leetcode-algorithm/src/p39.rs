/// brute-force
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut select = vec![];
    let mut res = vec![];
    dfs(0, target, &mut select, &candidates, &mut res);
    res
}

fn dfs(
    i: usize,
    left_target: i32,
    select: &mut Vec<i32>,
    candidates: &Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if left_target == 0 {
        res.push(select.clone());
        return;
    }

    if left_target < 0 {
        return;
    }

    if i >= candidates.len() {
        return;
    }

    dfs(i + 1, left_target, select, candidates, res);

    let num = left_target / candidates[i];
    let mut sum = 0;
    for _ in (1..=num) {
        select.push(candidates[i]);
        sum += candidates[i];
        dfs(i + 1, left_target - sum, select, candidates, res);
    }
    for _ in (1..=num) {
        select.pop();
    }
}
