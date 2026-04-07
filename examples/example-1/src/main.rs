#[exec_time::exec_time(name = "async_foo", unit = "ms", log_over = "5s")]
async fn foo() {
    std::thread::sleep(std::time::Duration::from_secs(2));
    let f = || async { "hello" };
    let fr = f().await;
    println!("{}", fr)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    foo().await;
}
