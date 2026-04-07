use exec_time::exec_time;
use std::time::Duration;

struct Worker;

impl Worker {
    #[exec_time(name = "worker.trace")]
    fn run_trace(&self) -> i32 {
        1
    }

    #[exec_time(name = "worker.debug", level = "debug")]
    fn run_debug(&self) -> i32 {
        2
    }

    #[exec_time(name = "worker.slow", log_over = "1ms")]
    fn run_slow(&self) -> i32 {
        std::thread::sleep(Duration::from_millis(2));
        3
    }

    #[exec_time(name = "worker.suppressed", log_over = "50ms")]
    fn run_suppressed(&self) -> i32 {
        4
    }

    #[exec_time(name = "worker.warn", warn_over = "1ms")]
    fn run_warn(&self) -> i32 {
        std::thread::sleep(Duration::from_millis(2));
        5
    }

    #[exec_time(backend = "stdout", name = "worker.stdout")]
    fn run_stdout(&self) -> i32 {
        6
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .with_writer(std::io::stdout)
        .with_max_level(tracing::Level::TRACE)
        .init();

    let worker = Worker;

    println!("trace-result={}", worker.run_trace());
    println!("debug-result={}", worker.run_debug());
    println!("slow-result={}", worker.run_slow());
    println!("suppressed-result={}", worker.run_suppressed());
    println!("warn-result={}", worker.run_warn());
    println!("stdout-result={}", worker.run_stdout());
}
