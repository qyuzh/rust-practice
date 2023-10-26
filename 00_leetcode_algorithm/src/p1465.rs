/// sort
pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut h_max = get_max_span(horizontal_cuts, h);
    let mut v_max = get_max_span(vertical_cuts, w);
    ((h_max as i64 * v_max as i64) % (1e9 as i64 + 7)) as i32
}

fn get_max_span(mut cuts: Vec<i32>, upper: i32) -> i32 {
    cuts.sort_unstable();
    let mut max = cuts[0];
    for i in 0..cuts.len() - 1 {
        max = max.max(cuts[i + 1] - cuts[i]);
    }
    max =max.max(upper - cuts[cuts.len() - 1]);
    max
}