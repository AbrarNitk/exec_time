# Time measure for Rust functions
### It will simply print execution time of a function

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/exec_time)](https://crates.io/crates/exec_time)
[![Build Status](https://travis-ci.org/AbrarNitk/exec_time.svg?branch=master)](https://travis-ci.org/AbrarNitk/exec_time)

#### Usage
```toml
[dependencies]
exec_time = "0.1.4"
```

## Examples
In print log, it is printing `Time <prefix>::<function_name>::<suffix>: <execution time> mills`

#### Example 1
It will always print.

```rust
#[macro_use]
extern crate exec_time;

#[exec_time]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    login()
}
```  

```text
Time login: 102 mills
```

#### Example 2
It will print only in debug mode.

```rust
#[macro_use]
extern crate exec_time;

#[exec_time(print = "debug")]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    login()
}
```  

```text
Time login: 102 mills
```

#### Example 3
It will never print.

```rust
#[macro_use]
extern crate exec_time;

#[exec_time(print = "never")]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    login()
}
```  

```text
```

#### Example 4
It will print, prefix and suffix with function name.

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
Time user/lib::login::route: 102 mills
```


### Note Point
Here `print`, `prefix` and `suffix` all are optional field. Default value of print is `always`.
`print` may be `always`(by default), `debug`, `never`. If the value is `always` it will print always.
If value is `debug`, It will print only in debug mode else, It will never print.