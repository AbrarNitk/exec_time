# `exec_time`

Attribute macro for printing function execution time.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/exec_time)](https://crates.io/crates/exec_time)
[![Build Status](https://travis-ci.org/AbrarNitk/exec_time.svg?branch=master)](https://travis-ci.org/AbrarNitk/exec_time)

## Install

```toml
[dependencies]
exec_time = "0.1.5"
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
[exec_time] login took 102 ms
```

## Options

- `print = "always"`: print in all builds. Default.
- `print = "debug"`: print only in debug builds.
- `print = "never"`: disable printing.
- `name = "..."`: override the default function-based label. Takes precedence over `prefix` and `suffix`.
- `unit = "ns" | "us" | "ms" | "s"`: choose the reported time unit. Default: `ms`.
- `prefix = "..."`: prepend a label before the function name.
- `suffix = "..."`: append a label after the function name.

## Custom Label

```rust
use exec_time::exec_time;

#[exec_time(name = "user.login", unit = "us")]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

```text
[exec_time] user.login took 102345 us
```
