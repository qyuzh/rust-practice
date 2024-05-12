pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut ids_s = [0; 26];
    let mut ids_t = [0; 26];

    for i in 0..s.len() {
        ids_s[(s.as_bytes()[i] - b'a') as usize] = i as i32;
        ids_t[(t.as_bytes()[i] - b'a') as usize] = i as i32;
    }

    let mut ret = 0;

    for i in 0..26 {
        ret += (ids_s[i] - ids_t[i]).abs()
    }

    ret
}

pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let n = energy.len();
    let k = k as usize;

    let mut f = energy.clone();

    for i in (0..n).rev() {
        if i + k < n {
            f[i] += f[i + k];
        }
    }

    *f.iter().max().unwrap()
}

pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    let mut f = vec![vec![(-1, i32::MIN); m]; n];

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dfs(i, j, &mut f, &grid);
        }
    }

    let t = f
        .iter()
        .map(|v| v.iter().map(|v| v.0).max().unwrap())
        .max()
        .unwrap();

    let step = f
        .iter()
        .map(|v| v.iter().map(|v| v.1).max().unwrap())
        .max()
        .unwrap();

    if t == 0 {
        step
    } else {
        t
    }
}

fn dfs(x: usize, y: usize, f: &mut Vec<Vec<(i32, i32)>>, grid: &Vec<Vec<i32>>) -> i32 {
    if f[x][y].0 != -1 {
        return f[x][y].0;
    }

    let n = grid.len();
    let m = grid[0].len();

    let mut ret = 0;

    let mut step1 = i32::MIN;

    for i in x + 1..n {
        ret = ret.max(dfs(i, y, f, grid) + grid[i][y] - grid[x][y]);
        step1 = step1.max(grid[i][y] - grid[x][y]);
    }

    for j in y + 1..m {
        ret = ret.max(dfs(x, j, f, grid) + grid[x][j] - grid[x][y]);
        step1 = step1.max(grid[x][j] - grid[x][y]);
    }

    f[x][y] = (ret, step1);

    ret
}

pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}
