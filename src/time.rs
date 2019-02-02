use spin::Mutex;

/// Kernel start time, measured in (seconds, microseconds) since Unix epoch
pub static START: Mutex<(u32, u32)> = Mutex::new((0, 0));
/// Kernel up time, measured in (seconds, microseconds) since `START_TIME`
pub static OFFSET: Mutex<(u32, u32)> = Mutex::new((0, 0));

pub fn monotonic() -> (u32, u32) {
    *OFFSET.lock()
}

pub fn realtime() -> (u32, u32) {
    let offset = monotonic();
    let start = *START.lock();
    let sum = start.1 + offset.1;
    (start.0 + offset.0 + sum / 1_000_000, sum % 1_000_000)
}

pub fn uptime() {
    let offset = self::OFFSET.lock();
    println!("{}s", offset.0 + offset.1 / 1_000_000);
}
