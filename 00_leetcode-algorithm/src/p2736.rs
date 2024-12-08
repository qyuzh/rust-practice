/// 1. Brute-Force in O(nm), where n = nums.length, m = queries.length
/// 2. Sort `nums[i] = nums1[i] + nums1[i]`
///
pub fn maximum_sum_queries_bf(
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    queries: Vec<Vec<i32>>,
) -> Vec<i32> {
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries.iter() {
        let mut max = -1;
        for i in 0..nums1.len() {
            if nums1[i] >= q[0] && nums2[i] >= q[1] {
                max = max.max(nums1[i] + nums2[i]);
            }
        }
        ans.push(max);
    }
    ans
}

pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut a: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
    a.sort_by(|x, y| y.0.cmp(&x.0)); // 按照nums1, 大到小

    let mut qid: Vec<usize> = (0..queries.len()).collect();
    qid.sort_by(|&i, &j| queries[j][0].cmp(&queries[i][0])); // 按照xi, 大到小

    let mut ans = vec![-1; queries.len()];
    let mut st: Vec<(i32, i32)> = Vec::new();
    let mut j = 0;
    for &i in &qid {
        let x = queries[i][0];
        let y = queries[i][1];
        while j < a.len() && a[j].0 >= x {
            while !st.is_empty() && st.last().unwrap().1 <= a[j].0 + a[j].1 {
                st.pop();
            }
            if st.is_empty() || st.last().unwrap().0 < a[j].1 {
                st.push((a[j].1, a[j].0 + a[j].1));
            }
            j += 1;
        }
        let p = st.partition_point(|&p| p.0 < y);
        if p < st.len() {
            ans[i] = st[p].1;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::p2736::maximum_sum_queries;

    #[test]
    fn test_1() {
        let queries = [[4, 1], [1, 3], [2, 5]]
            .into_iter()
            .map(|v| v.to_vec())
            .collect();
        maximum_sum_queries(vec![4, 3, 1, 2], vec![2, 4, 9, 5], queries);
    }
}
