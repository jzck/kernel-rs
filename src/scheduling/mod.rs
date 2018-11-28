mod process;
mod sleep;

mod fifo;

use spin::Mutex;
pub use self::process::*; 
lazy_static! {
    pub static ref SCHEDULER: Mutex<fifo::Fifo> = Mutex::new({
        let init_process: u32 = self::init as *const () as u32;
        let mut f = fifo::Fifo::new();
        f.add_task(init_process);
        f
    });
}

/// Scheduler algorithm needs to implement this
pub trait Scheduler {
    fn add_task(&mut self, ip: u32);
    fn next(&mut self) -> Option<Process>;
}

pub fn schedule() {
    loop {
        if let Some(mut p) = SCHEDULER.lock().next() {
            println!("executing {:#?}", p);
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
    SCHEDULER.lock().add_task(ip);
    0
}

pub fn sleep() {
    
}

pub fn init() {
    fprintln!("init first line");
    // let i = self::fork();
    // println!("fork={}", i);
    fprintln!("init last line");
}
