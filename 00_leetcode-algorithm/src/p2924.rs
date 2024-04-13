// runs in O(n + m)/O(n), where m = edges.len()
pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut indegs = vec![0; n as usize];
    for e in edges.iter() {
        indegs[e[1] as usize] += 1;
    }

    let mut ans = -1;
    for (idx, c) in indegs.into_iter().enumerate() {
        if c == 0 {
            if ans == -1 {
                ans = idx as i32;
            } else {
                return -1;
            }
        }
    }

    ans
}
