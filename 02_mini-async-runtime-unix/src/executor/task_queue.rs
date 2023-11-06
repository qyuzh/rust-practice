use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use chrono::prelude::*;

use crate::executor::task::Task;

pub struct TaskQueue {
    queue: RefCell<VecDeque<Rc<Task>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        const DEFAULT_TASK_QUEUE_SIZE: usize = 4096;
        Self::new_with_capacity(DEFAULT_TASK_QUEUE_SIZE)
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            queue: RefCell::new(VecDeque::with_capacity(capacity)),
        }
    }

    pub fn push(&self, runnable: Rc<Task>) {
        println!("{}: [Task Queue] add task", Local::now());

        self.queue.borrow_mut().push_back(runnable);
    }

    pub fn pop(&self) -> Option<Rc<Task>> {
        println!("{}: [Task Queue] try getting task", Local::now());

        self.queue.borrow_mut().pop_front()
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}
