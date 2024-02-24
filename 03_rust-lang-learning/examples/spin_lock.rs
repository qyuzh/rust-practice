use std::thread;

use rust_lang_learning::spin_lock::SpinLock;

fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock2().push(1));
        s.spawn(|| {
            let mut g = x.lock2();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    println!("g: {g:?}");
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
