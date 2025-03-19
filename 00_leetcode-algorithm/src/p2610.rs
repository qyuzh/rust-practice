pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ht = std::collections::HashMap::new();
    for &x in nums.iter() {
        let count = ht.entry(x).or_insert(0);
        *count += 1;
    }
    let mut res = vec![];
    for (&k, &v) in ht.iter() {
        while res.len() < v {
            res.push(vec![]);
        }
        for row in res.iter_mut().take(v) {
            row.push(k);
        }
    }
    res
}
