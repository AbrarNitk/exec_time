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

For auto tracing backend support:

```toml
[dependencies]
exec_time = { version = "0.1.6", features = ["tracing"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt"] }
```

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

Without the `tracing` feature, the default backend is `stdout`.

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
[exec_time][warn] db.query took 312 ms
```

## Tracing

```rust
use exec_time::exec_time;

#[exec_time(name = "db.query", level = "info", warn_over = "250ms")]
fn query_db() {}
```

With the `tracing` feature enabled and a subscriber installed, the default `backend = "auto"` resolves to tracing events instead of `stdout`.

Runnable example: [examples/example-1/src/main.rs](/home/ak/github/abrarnitk/exec_time/examples/example-1/src/main.rs)

```bash
cargo run --manifest-path examples/example-1/Cargo.toml --offline
```

Example output:

```text
INFO example.login took 26 ms label="example.login" elapsed_ns=26306096 elapsed_unit="ms" elapsed_value=26
login=ok
WARN example.query took 50 ms label="example.query" elapsed_ns=50656683 elapsed_unit="ms" elapsed_value=50
rows=42
[exec_time] example.stdout took 10 ms
override=stdout
```

To force `stdout` even when the tracing feature is enabled:

```rust
use exec_time::exec_time;

#[exec_time(backend = "stdout", name = "cache.lookup")]
fn lookup() {}
```

## Options

- `print = "always" | "debug" | "never"`: controls whether timing is emitted. Default: `always`.
- `name = "..."`: replaces the generated label.
- `prefix = "..."`, `suffix = "..."`: build the label as `<prefix>::<function>::<suffix>`. Ignored when `name` is set.
- `backend = "auto" | "stdout" | "tracing"`: output backend. Default: `auto`.
- `level = "trace" | "debug" | "info" | "warn" | "error"`: tracing event level. Default: `info`.
- `unit = "ns" | "us" | "ms" | "s"`: output unit. Default: `ms`.
- `log_over = "..."`: print only when execution time meets or exceeds the threshold.
- `warn_over = "..."`: write a warning to `stderr` only when execution time meets or exceeds the threshold.

Threshold values support `ns`, `us`, `ms`, and `s`, for example `500us`, `5ms`, or `0.5s`.

## Rules

- Default output format: `[exec_time] <label> took <value> <unit>`
- `backend = "auto"` uses `stdout` by default and switches to tracing when the `tracing` feature is enabled
- `backend = "tracing"` requires the `tracing` feature on `exec_time` and a direct `tracing` dependency in the consuming crate
- `backend = "tracing"` also needs a subscriber such as `tracing-subscriber` to render events
- `warn_over` takes precedence over `log_over` when both thresholds match
- `print = "never"` disables all output
