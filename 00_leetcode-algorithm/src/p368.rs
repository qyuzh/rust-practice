pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut f = vec![1; nums.len()];
    let mut max_size = 1;
    let mut max_val = f[0];

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] % nums[j] == 0 {
                f[i] = f[i].max(f[j] + 1);
            }
        }
        if max_size < f[i] {
            max_size = f[i];
            max_val = nums[i];
        }
    }

    let mut result = Vec::with_capacity(max_size as usize);

    if max_size == 1 {
        result.push(nums[0]);
        return result;
    }

    for i in (0..nums.len()).rev() {
        if f[i] == max_size && max_val % nums[i] == 0 {
            result.push(nums[i]);
            max_size -= 1;
            max_val = nums[i];
        }
    }

    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_divisible_subset_basic() {
        let nums = vec![1, 2, 3];
        let result = largest_divisible_subset(nums);
        assert!(result == vec![1, 2] || result == vec![1, 3]);
    }

    #[test]
    fn test_largest_divisible_subset_single_element() {
        let nums = vec![10];
        let result = largest_divisible_subset(nums);
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_largest_divisible_subset_all_divisible() {
        let nums = vec![1, 2, 4, 8];
        let result = largest_divisible_subset(nums);
        assert_eq!(result, vec![1, 2, 4, 8]);
    }

    #[test]
    fn test_largest_divisible_subset_no_divisible() {
        let nums = vec![5, 7, 11];
        let result = largest_divisible_subset(nums);
        assert_eq!(result.len(), 1); // Only one element can be in the subset
    }

    #[test]
    fn test_largest_divisible_subset_unsorted_input() {
        let nums = vec![4, 1, 8, 2];
        let result = largest_divisible_subset(nums);
        assert_eq!(result, vec![1, 2, 4, 8]);
    }

    #[test]
    fn test_largest_divisible_subset_large_input() {
        let nums = vec![3, 6, 9, 18, 36];
        let result = largest_divisible_subset(nums);
        assert!(result == vec![3, 6, 18, 36] || result == vec![3, 9, 18, 36]);
    }
}
