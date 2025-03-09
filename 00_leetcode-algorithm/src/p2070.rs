/// 1. sort queries_index
/// 2. sort items by price
/// 3. for each query, find the item with max beauty and price >= query
pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut queries_index: Vec<usize> = queries.iter().enumerate().map(|(idx, _)| idx).collect();
    queries_index.sort_unstable_by(|&a, &b| queries[a].cmp(&queries[b]));

    items.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    let mut ans = vec![0; queries.len()];

    let mut i = 0;
    let mut max_beauty = 0;
    for idx in queries_index.into_iter() {
        while i < items.len() && queries[idx] >= items[i][0] {
            max_beauty = max_beauty.max(items[i][1]);
            i += 1;
        }
        ans[idx] = max_beauty;
    }

    ans
}
