/// runs in O(n)/O(n), in which n = days.len()
pub fn min_cost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let n = days.len();
    let mut f = vec![i32::MAX; n + 1]; // i + 1 -> days[i]
    f[0] = 0;
    for i in 1..=n {
        f[i] = f[i - 1] + costs[0];

        let k = find_latest_day::<7>(&days, i);
        f[i] = f[i].min(f[k] + costs[1]);

        let k = find_latest_day::<30>(&days, i);
        f[i] = f[i].min(f[k] + costs[2]);
    }
    f[n]
}

#[inline]
fn find_latest_day<const N: i32>(days: &[i32], i: usize) -> usize {
    if i <= 1 {
        return 0;
    }

    macro_rules! cmp {
        ($ty: expr) => {
            days[$ty] + N <= days[i - 1]
        };
    }

    let mut j = i - 2;
    while j > 0 {
        if cmp!(j) {
            return j + 1; // map: j -> f[j + 1]
        }
        j -= 1;
    }

    cmp!(0) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cost_tickets() {
        let ret = min_cost_tickets([1, 4, 6, 7, 8, 20].into(), [2, 7, 15].into());
        assert_eq!(ret, 11)
    }
}
