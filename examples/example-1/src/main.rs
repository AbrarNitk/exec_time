

async fn foo() {
    let f = || async {""};
    let fr = f().await;
    println!("{}", fr)
}


fn main() {
    println!("Hello, world!");
    async {foo().await};
}
