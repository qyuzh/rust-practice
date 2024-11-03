/// dfs: enumeration --> Time Limit Exceeded
pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    let mut ans = i32::MAX;
    dfs(&price, &special, &needs, 0, &mut ans);
    ans
}

fn dfs(price: &[i32], special: &[Vec<i32>], needs: &[i32], cur_price: i32, ans: &mut i32) {
    if needs.iter().all(|&x| x == 0) {
        *ans = cur_price.min(*ans);
        return;
    }

    // buy a special
    for s in special.iter() {
        let mut new_needs = needs.to_vec();
        let mut i = 0;
        while i < needs.len() {
            if new_needs[i] < s[i] {
                break;
            }
            new_needs[i] -= s[i];
            i += 1;
        }
        if i == needs.len() {
            dfs(
                price,
                special,
                &new_needs,
                cur_price + s.last().unwrap(),
                ans,
            );
        }
    }

    // buy a single item
    for i in 0..needs.len() {
        if needs[i] > 0 {
            let mut new_needs = needs.to_vec();
            new_needs[i] -= 1;
            dfs(price, special, &new_needs, cur_price + price[i], ans);
        }
    }
}
