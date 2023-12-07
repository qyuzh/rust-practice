/// get answer in O(n)/O(n)
pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut ht = vec![(0, 0); arr.len() + 1];
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            ht[mat[i][j] as usize] = (i, j);
        }
    }
    let mut row = vec![0; mat.len()];
    let mut col = vec![0; mat[0].len()];
    for (idx, &x) in arr.iter().enumerate() {
        let (i, j) = ht[x as usize];
        row[i] += 1;
        col[j] += 1;
        if row[i] == mat[0].len() || col[j] == mat.len() {
            return idx as i32;
        }
    }
    0
}
