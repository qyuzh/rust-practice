pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    score.sort_by(|a, b| b[k as usize].cmp(&a[k as usize]));
    score
}
