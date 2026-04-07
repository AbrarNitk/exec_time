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
- `log_over = "..."`: print only when execution time is at least the given threshold.
- `warn_over = "..."`: emit a warning to `stderr` only when execution time is at least the given threshold.
- `prefix = "..."`: prepend a label before the function name.
- `suffix = "..."`: append a label after the function name.

Threshold values currently support `ns`, `us`, `ms`, and `s`, for example `log_over = "5ms"` or `warn_over = "0.5s"`.
If both `log_over` and `warn_over` are set, `warn_over` takes precedence when both thresholds match.

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

## Slow Call Reporting

```rust
use exec_time::exec_time;

#[exec_time(log_over = "50ms")]
fn query_cache() {}

#[exec_time(name = "db.query", warn_over = "250ms")]
fn query_db() {}
```
