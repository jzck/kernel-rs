#[derive(Debug)]
pub enum State {
    Running,
    Ready,
    Sleeping(u32),
    Blocked(),
}

#[derive(Debug)]
pub struct Process {
    pid: i32,
    // this is eip right now
    // this will be an elf blob later
    ip: u32,
    state: State,
}

impl Process {
    pub fn new(pid: i32, ip: u32) -> Process {
        Process {
            pid,
            ip,
            state: State::Ready,
        }
    }

    pub unsafe fn execute(&mut self) {
        use super::schedule;
        let scheduler_loop = schedule as *const () as u32;
        asm!("push $0; push $1; ret" :: "r"(scheduler_loop) ,"r"(self.ip) :: "volatile", "intel");
        unreachable!();
    }
}
