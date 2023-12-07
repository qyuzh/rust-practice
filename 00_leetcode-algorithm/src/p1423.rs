/// get ans in O(n)/O(1)
/// keywords: sliding window
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    for &x in card_points.iter().take(k as usize) {
        sum += x;
    }
    let mut ans = sum;
    let (mut i, mut j) = (k as usize - 1, card_points.len() - 1);
    loop {
        sum -= card_points[i];
        sum += card_points[j];
        ans = ans.max(sum);
        if i == 0 {
            break ans;
        }
        j -= 1;
        i -= 1;
    }
}
