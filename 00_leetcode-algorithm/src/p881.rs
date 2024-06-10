/// runs in O(nlogn)/O(1)
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();

    let mut cnt = 0;

    let (mut l, mut r) = (0, people.len() - 1);
    // Less not LessOrEqual is to avoid to subtract with overflow to `r`
    while l < r {
        if people[l] + people[r] <= limit {
            l += 1;
        }
        r -= 1;
        cnt += 1;
    }

    // Special case: Equal
    if l == r {
        cnt += 1
    }

    cnt
}
