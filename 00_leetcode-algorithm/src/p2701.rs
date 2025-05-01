/// Runs in O(nlogn + mlogm + (n+m)log(min(n,m))/O(n+m).
pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, mut pills: i32, strength: i32) -> i32 {
    let mut tasks = tasks;
    let mut workers = workers;
    tasks.sort_unstable();
    workers.sort_unstable();

    let mut left = 0;
    let mut right = tasks.len().min(workers.len()) as i32;

    while left < right {
        let mid = (left + right + 1) / 2;
        if can_assign(&tasks, &workers, mid as usize, pills, strength) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

/// Runs in O(n+m).
fn can_assign(tasks: &[i32], workers: &[i32], mid: usize, pills: i32, strength: i32) -> bool {
    let mut pills = pills;
    let mut valid_tasks = std::collections::VecDeque::new();
    let mut i = 0;
    let m = workers.len();
    for &w in workers[m - mid..m].iter() {
        while i < tasks.len() && tasks[i] <= w + strength {
            valid_tasks.push_back(tasks[i]);
            i += 1;
        }
        if valid_tasks.is_empty() {
            return false;
        }
        if w >= *valid_tasks.front().unwrap() {
            valid_tasks.pop_front();
        } else if pills > 0 {
            pills -= 1;
            valid_tasks.pop_back();
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::arch::x86_64;

    use super::*;

    #[test]
    fn test_max_task_assign_basic() {
        let tasks = vec![3, 2, 1];
        let workers = vec![3, 2, 1];
        let pills = 0;
        let strength = 0;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 3);
    }

    #[test]
    fn test_max_task_assign_with_pills() {
        let tasks = vec![5, 4, 3];
        let workers = vec![3, 3, 3];
        let pills = 2;
        let strength = 2;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 3);
    }

    #[test]
    fn test_max_task_assign_not_enough_pills() {
        let tasks = vec![5, 4, 3];
        let workers = vec![3, 3, 3];
        let pills = 1;
        let strength = 2;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 2);
    }

    #[test]
    fn test_max_task_assign_no_pills_needed() {
        let tasks = vec![1, 1, 1];
        let workers = vec![2, 2, 2];
        let pills = 0;
        let strength = 0;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 3);
    }

    #[test]
    fn test_max_task_assign_no_tasks() {
        let tasks = vec![];
        let workers = vec![1, 2, 3];
        let pills = 1;
        let strength = 1;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 0);
    }

    #[test]
    fn test_max_task_assign_no_workers() {
        let tasks = vec![1, 2, 3];
        let workers = vec![];
        let pills = 1;
        let strength = 1;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 0);
    }

    #[test]
    fn test_max_task_assign_edge_case() {
        let tasks = vec![10];
        let workers = vec![5];
        let pills = 1;
        let strength = 5;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 1);
    }

    #[test]
    fn test_max_task_assign_edge_case2() {
        let tasks = vec![5, 9, 8, 5, 9]; // 9, 9, 8, 5, 5
        let workers = vec![1, 6, 4, 2, 6]; // 6, 6, 4, 2, 1
        let pills = 1;
        let strength = 5;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 3);
    }

    // #[test]
    // fn test1() {
    //     let mut tt = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //     let x = tt.partition_point(|x| *x < 5);
    //     assert_eq!(x, 4);
    //     let x = tt.partition_point(|x| *x > 11);
    //     assert_eq!(x, 10);

    //     tt.sort_unstable_by_key(|x| -x);
    //     println!("{:?}", tt);
    //     let x = tt.partition_point(|x| *x > 10);
    //     assert_eq!(x, 0);
    //     let x = tt.partition_point(|x| *x > 5);
    //     assert_eq!(x, 5);
    //     let x = tt.partition_point(|x| *x < 1);
    //     assert_eq!(x, 10);
    // }
}
