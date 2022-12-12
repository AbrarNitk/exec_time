

#[exec_time::exec_time]
async fn foo() {
    let f = || async {"hello"};
    let fr = f().await;
    println!("{}", fr)
}


#[tokio::main]
async fn main() {
    use tokio::spawn;
    println!("Hello, world!");
    foo().await;
}

