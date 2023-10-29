/// P2512, Reward Top K Students
pub fn top_students(
    positive_feedback: Vec<String>,
    negative_feedback: Vec<String>,
    report: Vec<String>,
    student_id: Vec<i32>,
    k: i32,
) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    use std::cmp::Ordering;
    let n = student_id.len();
    let mut rank = vec![0; n];


    let positive_feedback: HashSet<String> = positive_feedback.into_iter().collect();
    let negative_feedback: HashSet<String> = negative_feedback.into_iter().collect();
    let mut hm = HashMap::with_capacity(n);
    student_id.iter().enumerate().for_each(|(i, &v)| { hm.insert(v, i); });

    for (i, v) in report.iter().enumerate() {
        v.split(' ').for_each(|v| {
            if positive_feedback.contains(v) {
                rank[i] += 3
            } else if negative_feedback.contains(v) {
                rank[i] -= 1
            }
        });
    }

    let mut student_id = student_id;
    student_id.sort_by(|a, b|
        if rank[*hm.get(b).unwrap()].cmp(&rank[*hm.get(a).unwrap()]) == Ordering::Equal {
            (*a).cmp(b)
        } else {
            rank[*hm.get(b).unwrap()].cmp(&rank[*hm.get(a).unwrap()])
        }
    );

    student_id.into_iter().take(k as usize).collect()
}