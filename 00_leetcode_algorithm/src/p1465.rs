/// 
pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut hc = horizontal_cuts;
    hc.sort_unstable();
    let mut h_max = hc[0];
    for i in 0..hc.len() - 1 {
        h_max = h_max.max(hc[i + 1] - hc[i]);
    }
    h_max = h_max.max(h - hc[hc.len() - 1]);

    let mut vc = vertical_cuts;
    vc.sort_unstable();
    let mut v_max = vc[0];
    for i in 0..vc.len() - 1 {
        v_max = v_max.max(vc[i + 1] - vc[i]);
    }
    v_max = v_max.max(w - vc[vc.len() - 1]);

    ((h_max as i64 * v_max as i64) % (1e9 as i64 + 7)) as i32
}