mod fifo;
use spin::Mutex;

lazy_static! {
    pub static ref SCHEDULER: Mutex<fifo::Fifo> = Mutex::new({
        let init_process: u32 = self::init as *const () as u32;
        let mut f = fifo::Fifo::new();
        f.add_process(init_process);
        f
    });
}

// lazy_static! {
//     static ref SCHEDULER: self::Scheduler = self::Scheduler {
//         list: Mutex::new(VecDeque::new())
//     };
// }

#[derive(Debug)]
pub struct Process {
    // this is eip right now
    // this will be an elf blob later
    pid: i32,
    ip: u32,
}

impl Process {
    pub fn new(pid: i32, ip: u32) -> Process {
        Process { pid, ip }
    }

    pub unsafe fn execute(&mut self) {
        let scheduler_loop = schedule as *const () as u32;
        asm!("push $0; push $1; ret" :: "r"(scheduler_loop) ,"r"(self.ip) :: "volatile", "intel");
        unreachable!();
    }
}

pub trait Scheduler {
    fn add_process(&mut self, ip: u32);
    fn next(&mut self) -> Option<Process>;
}

pub fn schedule() {
    loop {
        if let Some(mut p) = SCHEDULER.lock().next() {
            println!("executing {:#x}", p.ip);
            flush!();
            unsafe {
                SCHEDULER.force_unlock();
                p.execute();
            }
            unreachable!();
        }
    }
}

pub fn fork() -> i32 {
    let ip;
    unsafe { asm!("pop $0" : "=r"(ip) ::: "intel") };
    println!("ip = {:#x}", ip);
    flush!();
    unsafe { asm!("push $0" :: "r"(ip) :: "intel") };
    SCHEDULER.lock().add_process(ip);
    0
}

pub fn init() {
    fprintln!("init first line");
    // let i = self::fork();
    // println!("fork={}", i);
    fprintln!("init last line");
}
