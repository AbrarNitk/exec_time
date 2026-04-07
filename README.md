# `exec_time`

Attribute macro for printing sync and async function execution time.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/exec_time)](https://crates.io/crates/exec_time)
[![Build Status](https://travis-ci.org/AbrarNitk/exec_time.svg?branch=master)](https://travis-ci.org/AbrarNitk/exec_time)

## Install

```toml
[dependencies]
exec_time = "0.1.6"
```

MSRV: Rust `1.88`.

## Default

```rust
use exec_time::exec_time;

#[exec_time]
fn login() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

```text
[exec_time] login took 102 ms
```

## Slow Calls

```rust
use exec_time::exec_time;

#[exec_time(name = "cache.lookup", unit = "us", log_over = "500us")]
fn lookup() {}

#[exec_time(name = "db.query", warn_over = "250ms")]
fn query_db() {}
```

```text
[exec_time] cache.lookup took 734 us
```

```text
[exec_time][warn] [exec_time] db.query took 312 ms
```

## Options

- `print = "always" | "debug" | "never"`: controls whether timing is emitted. Default: `always`.
- `name = "..."`: replaces the generated label.
- `prefix = "..."`, `suffix = "..."`: build the label as `<prefix>::<function>::<suffix>`. Ignored when `name` is set.
- `unit = "ns" | "us" | "ms" | "s"`: output unit. Default: `ms`.
- `log_over = "..."`: print only when execution time meets or exceeds the threshold.
- `warn_over = "..."`: write a warning to `stderr` only when execution time meets or exceeds the threshold.

Threshold values support `ns`, `us`, `ms`, and `s`, for example `500us`, `5ms`, or `0.5s`.

## Rules

- Default output format: `[exec_time] <label> took <value> <unit>`
- `warn_over` takes precedence over `log_over` when both thresholds match
- `print = "never"` disables all output
