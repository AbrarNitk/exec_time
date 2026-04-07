use exec_time::exec_time;
use std::time::Duration;

struct Worker;

impl Worker {
    #[exec_time]
    fn run(&self) -> i32 {
        42
    }

    #[exec_time(name = "worker.job", unit = "us")]
    fn run_named(&self) -> i32 {
        std::thread::sleep(Duration::from_millis(1));
        7
    }

    #[exec_time(log_over = "1ms")]
    fn run_slow_logged(&self) -> i32 {
        std::thread::sleep(Duration::from_millis(2));
        9
    }

    #[exec_time(log_over = "50ms")]
    fn run_fast_suppressed(&self) -> i32 {
        11
    }

    #[exec_time(name = "worker.warn", warn_over = "1ms")]
    fn run_warned(&self) -> i32 {
        std::thread::sleep(Duration::from_millis(2));
        13
    }
}

fn main() {
    let worker = Worker;
    println!("result={}", worker.run());
    println!("named-result={}", worker.run_named());
    println!("slow-logged-result={}", worker.run_slow_logged());
    println!("fast-suppressed-result={}", worker.run_fast_suppressed());
    println!("warned-result={}", worker.run_warned());
}
