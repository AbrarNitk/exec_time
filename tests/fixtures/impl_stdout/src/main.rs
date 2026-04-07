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
}

fn main() {
    let worker = Worker;
    println!("result={}", worker.run());
    println!("named-result={}", worker.run_named());
}
