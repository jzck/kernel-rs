//! sleeping processing are stored in a delta queue
//! separate from other scheduling structures: this
//! way the scheduling algorithms don't have to worry about
//! managing these
//!
//! inspired from https://wiki.osdev.org/Blocking_Process

use alloc::collections::vec_deque::VecDeque;
use super::*;

struct Sleeper {
    process: Process,
    // ms
    ticks: u32,
}

/// osdev calls this a delta queue but google doesnt know
struct DeltaQueue {
    queue: VecDeque<Sleeper>,
}

impl DeltaQueue {
    pub fn insert(&mut self, process: Process, ticks: u32) {
        let sleeper = Sleeper { process, ticks };
    }

    /// decreases timer on the list and returns the number
    /// of finished sleepers
    pub fn tick(&mut self) -> u32 {
        let mut i: u32 = 0;
        while let Some(link) = self.queue.get_mut(i as usize) {
            // find how many links have 0 ticks left
            if link.ticks > 0 {
                link.ticks -= 1;
                break;
            }
            i += 1;
        }
        i
    }
}
