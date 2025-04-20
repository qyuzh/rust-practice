/// LeetCode 781. Rabbits in Forest
/// https://leetcode.com/problems/rabbits-in-forest/
/// if there are 10 answers of 10, then there are 11 rabbits at least
/// if there are 11 answers of 10, then there are 11 rabbits at least
/// if there are 12 answers of 10, then there are (11 + 11) rabbits at least
/// if there are 22 answers of 10, then there are (11 + 11) rabbits at least
/// if there are 23 answers of 10, then there are (11 + 11 + 11) rabbits at least
pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut count = vec![0; 1001];
    for &answer in &answers {
        count[answer as usize] += 1;
    }
    let mut total = 0;
    for (i, &c) in count.iter().enumerate() {
        if c > 0 {
            total += (i + 1) * ((c + i) / (i + 1));
        }
    }
    total as i32
}
