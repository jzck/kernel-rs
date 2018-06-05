use alloc::VecDeque;

use super::*;

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
    fn add_process(&mut self, ip: u32) {
        let p = Process::new(self.next_pid, ip);
        self.list.push_back(p);
        self.next_pid += 1;
    }

    fn next(&mut self) -> Option<Process> {
        self.list.pop_front()
    }
}
