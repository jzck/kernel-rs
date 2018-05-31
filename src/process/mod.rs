mod fifo;

lazy_static! {
    static ref SCHEDULER: fifo::Fifo = fifo::Fifo::new();
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
    ip: u32,
}

impl Process {
    pub fn new(ip: u32) -> Process {
        Process { ip }
    }

    pub unsafe fn execute(&mut self) {
        asm!("push $0; ret" :: "r"(self.ip) :: "volatile", "intel");
    }
}

pub trait Scheduler {
    fn add_process(&mut self, ip: u32);
    fn next(&mut self) -> Option<Process>;
}

pub fn ploop() {
    let ip = self::init as *const () as u32;
    unsafe {
        SCHEDULER.add_process(ip);
    }
    loop {
        if let Some(mut p) = unsafe { SCHEDULER.next() } {
            print!("{:?}", p);
            unsafe {
                p.execute();
            }
        }
    }
}

pub fn fork() -> i32 {
    let ip;
    unsafe {
        asm!("" : "={eip}"(ip) ::: "volatile");
        SCHEDULER.add_process(ip);
    }
    0
}

pub fn init() -> ! {
    let i = self::fork();
    println!("inside fork_print() function!!!!!, fork={}", i);
    flush!();
    loop {}
}
