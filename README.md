# `exec_time`

Attribute macro for printing function execution time.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/exec_time)](https://crates.io/crates/exec_time)
[![Build Status](https://travis-ci.org/AbrarNitk/exec_time.svg?branch=master)](https://travis-ci.org/AbrarNitk/exec_time)

## Install

```toml
[dependencies]
exec_time = "0.1.4"
```

MSRV: Rust `1.88`.

## Example

```rust
use exec_time::exec_time;

#[exec_time]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    login();
}
```

```text
Time login: 102 ms
```

## Options

- `print = "always"`: print in all builds. Default.
- `print = "debug"`: print only in debug builds.
- `print = "never"`: disable printing.
- `prefix = "..."`: prepend a label before the function name.
- `suffix = "..."`: append a label after the function name.

## Custom Label

```rust
use exec_time::exec_time;

#[exec_time(prefix = "user/lib", suffix = "route")]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

```text
Time user/lib::login::route: 102 ms
```
