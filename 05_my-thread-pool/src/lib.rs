use std::thread;

struct Task {
    work: Box<dyn FnOnce(&str) + Send + Sync>,
}

impl Task {
    fn new<F>(f: F) -> Self
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        Task { work: Box::new(f) }
    }
}

pub struct Threadpool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: crossbeam_channel::Sender<Task>,
}

impl Threadpool {
    pub fn new(worker_num: usize, max_task_num: usize) -> Self {
        let mut workers = Vec::with_capacity(worker_num);
        let (sender, recv) = crossbeam_channel::bounded::<Task>(max_task_num);

        for i in 0..worker_num {
            let worker = format!("worker {i}");
            let r = recv.clone();
            workers.push(thread::spawn(move || loop {
                match r.recv() {
                    Ok(task) => (task.work)(&worker),
                    Err(e) => {
                        println!("{worker} exit with {}", e);
                        break;
                    }
                }
            }));
        }

        Self { workers, sender }
    }

    /// Execute a task that may not execute immediately because no available worker
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        let task = Task::new(f);
        self.sender.send(task).unwrap();
    }

    /// Wait all task finished
    pub fn join(self) {
        // 1. notify all workers that it could exit if no task is executing
        drop(self.sender);

        // 2. wait all workers finish its task and exit
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn it_works() {
        let pool = Threadpool::new(6, 10);
        for i in 0..6 {
            pool.execute(move |message| {
                thread::sleep(Duration::from_millis(100));
                println!("Task: {} prints in {}", i, message);
            });
        }
        thread::sleep(Duration::from_secs(1));
        pool.join();
    }
}
