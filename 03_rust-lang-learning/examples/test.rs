#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::atomic::{
    fence, AtomicBool, AtomicI32, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize,
};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn scoped_thread() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}

fn thread_parking() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn condition_variables() {
    use std::sync::Condvar;

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn stop_flag() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work.
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            println!("thread is doing some work...");
            thread::sleep(Duration::from_secs(5))
        }
        println!("received stop signal");
    });

    // Use the main thread to listen for user input.
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    // Inform the background thread it needs to stop.
    STOP.store(true, Relaxed);

    // Wait until the background thread finishes.
    background_thread.join().unwrap();
}

fn progress_report() {
    let num_done = AtomicUsize::new(0);

    let main = thread::current();

    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                println!("{:?}: process item...", thread::current().id()); // Assuming this takes some time.
                thread::sleep(Duration::from_millis(100));
                num_done.store(i + 1, Relaxed);
                main.unpark();
            }
        });

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!(
                "{:?}: Working.. {n}/100 done",
                thread::current().name().unwrap()
            );
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("All Done!");
}

fn lazy_init() {
    fn get_x() -> u64 {
        println!("get x");
        static X: AtomicU64 = AtomicU64::new(0);
        let mut x = X.load(Relaxed);
        if x == 0 {
            x = calculate_x();
            X.store(x, Relaxed);
        }
        x
    }

    fn calculate_x() -> u64 {
        println!("calculating...");
        thread::sleep(Duration::from_secs(1));
        123
    }

    dbg!(get_x());
    dbg!(get_x());
}

fn lazy_init2() {
    fn get_x() -> u64 {
        println!("get x");
        static X: AtomicU64 = AtomicU64::new(0);
        let mut x = X.load(Relaxed);
        if x == 0 {
            x = calculate_x();
            // this can guarantee that X only be set once.
            match X.compare_exchange(0, x, Relaxed, Relaxed) {
                Ok(_) => x,
                Err(k) => k,
            }
        } else {
            x
        }
    }

    fn calculate_x() -> u64 {
        println!("calculating...");
        thread::sleep(Duration::from_secs(1));
        123
    }

    dbg!(get_x());
    dbg!(get_x());
}

fn progress_report2() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    println!("{:?} {}", thread::current().id(), t * 25 + i); // Assuming this takes some time.
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1000, "too many IDs!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

fn total_modification_order() {
    static X: AtomicI32 = AtomicI32::new(0);

    fn a() {
        for _ in 0..10000 {
            X.fetch_add(5, Relaxed);
        }
        for _ in 0..10000 {
            X.fetch_add(5, Relaxed);
        }
    }

    fn b() {
        let a = X.load(Relaxed);
        let b = X.load(Relaxed);
        let c = X.load(Relaxed);
        let d = X.load(Relaxed);
        println!("{a} {b} {c} {d}");
    }

    thread::scope(|s| {
        s.spawn(a);
        s.spawn(b);
    });
}

struct Data([u8; 100]);
fn get_data() -> &'static Data {
    fn generate_data() -> Data {
        Data([123; 100])
    }

    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    // When init, there maybe are two threads both enter here.
    // That means Thread A holds p that's null, and so do Thread B.
    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            // Safety: p comes from Box::into_raw right above,
            // and wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Something important here is that we must guarantee p is valid.
    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}

fn fences() {
    static mut DATA: [u64; 10] = [0; 10];

    const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
    static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(500));

    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe { DATA[i] });
            }
        }
    }

    fn some_calculation(i: usize) -> u64 {
        thread::sleep(Duration::from_millis(400 + i as u64 % 3 * 100));
        123
    }
}

fn main() {
    // scoped_thread();
    // thread_parking();
    // condition_variables()

    // stop_flag();
    // progress_report();
    // lazy_init();
    // progress_report2();

    // total_modification_order();
    // println!("{:p}", get_data());
    // println!("{:p}", get_data()); // Same address as before.

    fences();
}
