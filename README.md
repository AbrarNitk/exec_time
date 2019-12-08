# Time measure for Rust functions
### It will simply print execution time of a function

#### Usage
```toml
[dependencies]
exec_time = "0.1.0"
```

## Examples

```rust
#[macro_use]
extern crate exec_time;

#[exec_time(print = "always", prefix = "user/lib", suffix="route")]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    login()
}
```  


```text
Time user/lib::login::test_module: 102 mills
```

Here `print`, `prefix` and `suffix` all are optional field. Default value of print is `always`.
`print` may be `always`(by default), `debug`, `never`. If the value is `always` it will print always.
If value is `debug`, It will print only in debug mode else, It will never print.