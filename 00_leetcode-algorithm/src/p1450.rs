/// runs in O(n)/O(1)
pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    start_time
        .iter()
        .zip(end_time.iter())
        .fold(0, |acc, (l, r)| {
            acc + (*l <= query_time && query_time <= *r) as i32
        })
}
