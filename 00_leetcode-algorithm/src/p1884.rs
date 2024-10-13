/// p1884 - egg drop with 2 eggs and n floors
///
/// runs in O(1)/O(1)
///
/// # Inspiration
///
/// 最值问题的解题思路
/// 1. dp
/// 2. 给出一个可行的操作步骤(给定n, 求最小的操作数 -反向思考-> 给定操作数, 可以验证的最大n)
pub fn two_egg_drop(n: i32) -> i32 {
    ((n * 8 + 1) as f64).sqrt().ceil() as i32 / 2
}
