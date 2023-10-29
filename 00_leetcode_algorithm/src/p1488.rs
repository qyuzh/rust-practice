use std::collections::{BTreeSet, HashMap};

/// P1488, Avoid Flood in The City
pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut gmap: HashMap<i32, usize> = HashMap::new();
    let mut res = vec![-1; rains.len()];

    let mut que: BTreeSet<usize> = BTreeSet::new();
    for (idx, i) in rains.iter().enumerate() {
        if *i == 0 {
            que.insert(idx);
        } else {
            if let Some(x) = gmap.get(i) {
                if que.is_empty() {
                    return vec![];
                }
                let tidx = que.range((x)..).next();
                if tidx.is_none() {
                    return vec![];
                }
                let tidx = *tidx.unwrap();
                que.remove(&tidx);
                res[tidx] = *i;
                gmap.insert(*i, idx);
            } else {
                gmap.insert(*i, idx);
            }
        }
    }

    que.iter().for_each(|x| {
        res[*x] = 1;
    });

    res
}
