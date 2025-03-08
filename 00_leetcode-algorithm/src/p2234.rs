/// Steps:
/// 1. 如果所有的花园已经种满花, 那么直接返回`n * full`
/// 2. 如果new_flowers可以种满所有的花园, 那么直接返回 `max(n * full, (n - 1) * full + (target - 1) * partial)`
/// 3. 如果new_flowers不足以种满所有的花园, 先把所有的花园的花都变成target的花, 计算剩余的花的数量, 记为`left_flowers`
///    1. 先按照花园已有的花的数量从小到大排序
///    2. 枚举i(从1开始), 把到第i(不包含)个花园的花的数量**不**变成target朵
///       1. 更新`left_flowers += (target - flowers[i - 1])`
///       2. 前i个花园能够得到的最大的最小值是`avg = (pre_sum + left_flowers) / j`, 其中pre_sum是前j个花园的花的数量之和, `j < i`
///       3. 计算`total_beauty = avg * partial + (n - i) * full`, 更新 `ans`
pub fn maximum_beauty(
    mut flowers: Vec<i32>,
    new_flowers: i64,
    target: i32,
    full: i32,
    partial: i32,
) -> i64 {
    let n = flowers.len() as i64;
    let full = full as i64;
    let partial = partial as i64;

    // let mut left_flowers = new_flowers;
    // for flower in flowers.iter_mut() {
    //     left_flowers -= target as i64 - target.min(*flower) as i64;
    //     *flower = target.min(*flower);
    // }

    // 种满所有花园后剩余的花
    let mut left_flowers = new_flowers - target as i64 * n;
    for flower in flowers.iter_mut() {
        *flower = target.min(*flower);
        left_flowers += *flower as i64;
    }

    if left_flowers == new_flowers {
        return n * full;
    }

    if left_flowers >= 0 {
        return (n * full).max((n - 1) * full + (target as i64 - 1) * partial);
    }

    flowers.sort_unstable();

    let mut ans = 0;
    let mut pre_sum = 0;
    let mut j = 0;

    for i in 1..=n as usize {
        left_flowers += (target - flowers[i - 1]) as i64;
        if left_flowers < 0 {
            continue;
        }

        while j < i && flowers[j] as i64 * j as i64 <= pre_sum + left_flowers {
            pre_sum += flowers[j] as i64;
            j += 1;
        }

        let avg = (pre_sum + left_flowers) / j as i64;
        let total_beauty = avg * partial + (n - i as i64) * full;
        ans = ans.max(total_beauty);
    }

    ans
}
