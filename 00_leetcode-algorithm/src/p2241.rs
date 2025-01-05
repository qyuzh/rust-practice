const MONEY: [i32; 5] = [20, 50, 100, 200, 500];
const LEN: usize = MONEY.len();

#[allow(clippy::upper_case_acronyms)]
struct ATM {
    banknotes: [i32; LEN],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    fn new() -> Self {
        Self {
            banknotes: [0; LEN],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        (0..LEN).for_each(|i| {
            self.banknotes[i] += banknotes_count[i];
        });
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut amount = amount;
        let mut res = vec![0; LEN];
        for i in (0..LEN).rev() {
            let count = amount / MONEY[i];
            if count > self.banknotes[i] {
                res[i] = self.banknotes[i];
            } else {
                res[i] = count;
            }
            amount -= res[i] * MONEY[i];
        }
        if amount == 0 {
            (0..LEN).for_each(|i| {
                self.banknotes[i] -= res[i];
            });
            res
        } else {
            vec![-1]
        }
    }
}
