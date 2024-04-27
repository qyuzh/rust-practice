pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
    let mut ans = i32::MAX;
    dfs(0, hours_before as f32, 0, &mut ans, speed as f32, &dist);
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn dfs(i: usize, hours_before: f32, skips: i32, min_skips: &mut i32, speed: f32, dist: &Vec<i32>) {
    if hours_before < 0f32 {
        return;
    }

    if i == dist.len() && hours_before >= 0f32 {
        *min_skips = skips.min(*min_skips)
    }

    if i >= dist.len() {
        return;
    }

    let t = (dist[i] as f32) / speed;

    // skip
    dfs(i + 1, hours_before - t, skips + 1, min_skips, speed, dist);

    // not skip
    dfs(
        i + 1,
        (hours_before - t).floor(),
        skips,
        min_skips,
        speed,
        dist,
    );
}

const FACTOR: i64 = 1e7 as i64;

pub fn min_skips2(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
    let mut f = std::collections::HashMap::new();
    let ans = dfs2(0, hours_before as i64 * FACTOR, speed as i64, &dist, &mut f);
    println!("len: {}", f.len());
    if ans >= i32::MAX >> 2 {
        -1
    } else {
        ans
    }
}

fn dfs2(
    i: usize,
    hours_before: i64,
    speed: i64,
    dist: &Vec<i32>,
    f: &mut std::collections::HashMap<(usize, i64), i32>,
) -> i32 {
    if i == dist.len() && hours_before >= 0 {
        return 0;
    }

    if i >= dist.len() || hours_before < 0 {
        return i32::MAX >> 2;
    }

    let hours = hours_before;
    if f.contains_key(&(i, hours)) {
        return *f.get(&(i, hours)).unwrap();
    }

    let t = (dist[i] as i64 * FACTOR) / speed;

    if t > hours_before {
        return i32::MAX >> 2;
    }

    // skip
    let num1 = dfs2(i + 1, hours_before - t, speed, dist, f) + 1;

    // not skip
    let num2 = dfs2(i + 1, (hours_before - t) / FACTOR * FACTOR, speed, dist, f);

    f.insert((i, hours), num1.min(num2));

    num1.min(num2)
}

#[test]
fn test_min_skip2() {
    let ans = min_skips2(
        [
            68, 49, 41, 23, 59, 69, 44, 70, 46, 87, 99, 93, 88, 83, 89, 34, 79, 62, 25, 32, 36, 72,
            39, 29, 39, 99, 37, 91, 27, 89, 91, 63, 41, 62, 45, 72, 93, 36, 37, 16, 74, 87, 70, 49,
            51, 45, 99, 19, 38, 6, 37, 92, 11, 45, 57, 84, 33, 96, 62, 28, 26, 69, 35, 72, 90, 35,
            18, 14, 79, 95, 76, 31, 32, 7, 86, 68, 29, 82, 30, 7, 13, 53, 43, 92, 61, 13, 88, 99,
            18, 1, 71, 21, 53, 84, 61, 62, 84, 26, 97, 37, 42, 37, 83, 8, 99, 12, 84, 74, 30, 69,
            95, 93, 5, 23, 54, 23, 5, 51, 64, 24, 96, 38, 5, 2, 39, 73, 78, 14, 43, 37, 59, 83, 70,
            68, 64, 18, 65, 60, 35, 25, 15, 43, 21, 94, 59, 83, 33, 92, 24, 2, 95, 44, 38, 73, 87,
            97, 25, 23, 79, 82, 52, 55, 66, 42, 12, 52, 86, 13, 24, 18, 62, 42, 32, 1, 34, 50, 31,
            58, 97, 35, 43, 39, 75, 93, 74, 2, 34, 84, 47, 89, 31, 8, 86, 16, 60, 40, 21, 52, 88,
            65, 91, 99, 40, 89, 16, 82, 38, 48, 79, 29, 29, 18, 62, 20, 59, 96, 78, 48, 54, 90, 71,
            61, 97, 69, 86, 28, 98, 32, 88, 97, 77, 11, 68, 58, 68, 91, 47, 10, 73, 15, 8, 71, 65,
            26, 74, 69, 20, 22, 42, 67, 57, 15, 88, 27, 92, 75, 63, 12, 66, 58, 8, 59, 17, 66, 54,
            59, 73, 8, 72, 60, 87, 40, 93, 3, 51, 33, 1, 96, 28, 75, 10, 6, 55, 45, 90, 9, 70, 8,
            9, 61, 5, 37, 53, 24, 99, 18, 99, 83, 94, 83, 54, 48, 49, 58, 49, 49, 11, 88, 85, 70,
            18, 84, 45, 16, 23, 69, 51, 68, 17, 94, 29, 1, 97, 3, 39, 60, 87, 93, 70, 55, 69, 49,
            30, 32, 59, 45, 20, 36, 25, 93, 98, 15, 5, 85,
        ]
        .into(),
        56,
        858,
    );

    assert_eq!(ans, 0);
}

pub fn min_skips3(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
    let n = dist.len();
    let mut f = vec![vec![0; n]; n];

    for i in (0..n).rev() {
        for s in (0..n - i) {
            f[i][s] = f[i][s - 1].min(f[i + 1][s - 1])
        }
    }

    todo!()
}
