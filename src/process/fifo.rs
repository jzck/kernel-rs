use spin::Mutex;
use alloc::VecDeque;

use super::*;

pub struct Fifo {
    list: VecDeque<Process>,
}

impl Fifo {
    pub fn new() -> Fifo {
        Fifo {
            list: VecDeque::new(),
        }
    }
}

impl Scheduler for Fifo {
    fn add_process(&mut self, ip: u32) {
        let p = Process::new(ip);
        self.list.push_back(p);
    }

    fn next(&mut self) -> Option<Process> {
        self.list.pop_front()
    }
}
