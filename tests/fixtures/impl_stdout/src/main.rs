use exec_time::exec_time;

struct Worker;

impl Worker {
    #[exec_time]
    fn run(&self) -> i32 {
        42
    }
}

fn main() {
    let worker = Worker;
    println!("result={}", worker.run());
}
