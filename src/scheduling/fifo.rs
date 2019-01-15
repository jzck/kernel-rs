//! simple first come first serve scheduling algorithm
//! unsound for everyday use, a process can decide to
//! hijack the cpu, also it only allows for terminating
//! processes...
//! however it's stupid simple to implement!

use alloc::collections::vec_deque::VecDeque;

use super::*;
// use super::process::*;

pub struct Fifo {
    list: VecDeque<Process>,
    next_pid: i32,
}

impl Fifo {
    pub fn new() -> Fifo {
        Fifo {
            list: VecDeque::new(),
            next_pid: 1,
        }
    }
}

impl Scheduler for Fifo {
    fn add_task(&mut self, ip: u32) {
        let p = Process::new(self.next_pid, ip);
        self.list.push_back(p);
        self.next_pid += 1;
    }

    fn next(&mut self) -> Option<Process> {
        self.list.pop_front()
    }
}
