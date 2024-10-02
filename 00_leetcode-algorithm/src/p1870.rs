/// binary search runs in O(nlogn)/O(1)
pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    const MAX: i32 = 1e7 as i32;
    let (mut l, mut r) = (1, MAX + 1);
    while l < r {
        let s = (l + r) >> 1;
        let mut duration: i32 = dist
            .iter()
            .take(dist.len() - 1)
            .map(|&x| x / s + (x % s != 0) as i32)
            .sum();
        let last = dist[dist.len() - 1] as f64 / s as f64 + duration as f64;
        if hour >= last || hour + f64::EPSILON >= last {
            r = s;
        } else {
            l = s + 1;
        }
    }
    if l > MAX {
        -1
    } else {
        l
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_min_speed_on_time() {
        let ans = min_speed_on_time([1, 1, 100000].into(), 2.01);
        assert_eq!(ans, 10000000);

        let t = 2.1f64;
        let tt = 2.1 + f64::EPSILON;
        println!("{t} {tt} {}", f64::EPSILON);
        test(t);

        fn test(t: f64) {
            println!("{t}");
        }
    }
}
