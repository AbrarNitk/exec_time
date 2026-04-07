# `exec_time` Usability Improvement Tasks

## Goal

Make `exec_time` useful as a real developer utility instead of only a demo macro
that prints execution time to `stdout`.

The crate should be easy to adopt, produce low-noise output, integrate with common
Rust observability tooling, and stay simple for small projects.


## Current Limitations

- The crate only prints to `stdout`, which is not ideal for applications already using `tracing`, `log`, or metrics systems.
- Output is plain text and unstructured, so it is hard to filter, search, or aggregate.
- There is no threshold support, so even very fast calls create noise.
- The public API is minimal and does not cover common production use cases.
- The README explains basic usage, but it does not position the crate as 
  part of a larger debugging or observability workflow.
- The macro implementation needs cleanup and test coverage before larger feature work.


## Guiding Principles

- Keep the zero-config case simple.
- Make advanced behavior opt-in.
- Prefer ecosystem integrations over custom logging behavior.
- Avoid macro options that are hard to understand or easy to misuse.
- Keep sync and async behavior consistent.

## Prioritized Tasks

### Phase 1: Stabilize the Core

- [x] Add unit and integration tests for sync functions.
- [x] Add unit and integration tests for async functions.
- [x] Verify behavior with generics, return values, and `where` clauses.
- [x] Verify behavior with methods inside `impl` blocks if supported.
- [x] Fix output formatting issues such as `mills` wording and inconsistent elapsed formatting.
- [x] Review generated code to ensure it preserves function semantics and visibility.
- [x] Test compiler compatibility with current stable Rust and document the supported MSRV.

### Phase 2: Improve Output Usability

- [x] Add configurable output units such as `ns`, `us`, `ms`, and `s`.
- [x] Add a `name` option so users can override the default function-based label.
- [x] Improve default message formatting to be more readable and consistent.
- [x] Consider adding a compact output mode for command-line tools.
- [x] Ensure sync and async macros format timing output the same way.


### Phase 3: Reduce Noise

- [ ] Add `warn_over` or `log_over` style thresholds so only slow calls are reported.
- [ ] Add `min_duration` support to suppress very small timings.
- [ ] Allow developers to choose behavior when below threshold: skip, debug log, or trace log.
- [ ] Define clear precedence rules when `print`, thresholds, and backend settings are combined.


### Phase 4: Support Real Logging Backends

- [ ] Add optional `tracing` integration behind a feature flag.
- [ ] Add optional `log` integration behind a feature flag.
- [ ] Keep `stdout` as the default fallback for simple projects.
- [ ] Emit structured fields when using `tracing` or `log` backends.
- [ ] Let users configure log level, such as `trace`, `debug`, `info`, `warn`, or `error`.

### Phase 5: Add Metrics Support

- [ ] Add optional integration with the `metrics` crate.
- [ ] Record execution time as histograms instead of only printing text.
- [ ] Allow metric name customization.
- [ ] Allow static labels or tags such as module name or operation name.
- [ ] Document how this integrates with common metrics exporters.

### Phase 6: Improve Macro API Design

- [ ] Review existing options: `print`, `prefix`, and `suffix`.
- [ ] Decide whether `prefix` and `suffix` should be replaced by a clearer `name` or `target` API.
- [ ] Consider an enum-like backend option such as `backend = "stdout|tracing|log|metrics"`.
- [ ] Consider an output-level option such as `level = "info"`.
- [ ] Consider a `disabled` or feature-gated mode for release builds.
- [ ] Keep attribute syntax short and readable for the common case.

### Phase 7: Documentation and Developer Experience

- [ ] Rewrite the README around developer use cases, not only printed output.
- [ ] Add examples for sync, async, threshold-based logging, and tracing integration.
- [ ] Add a comparison section explaining when to use `exec_time` versus manual instrumentation.
- [ ] Document runtime overhead and tradeoffs.
- [ ] Add a changelog or release notes process for future versions.
- [ ] Add crate-level docs with examples that render well on docs.rs.

## Candidate API Ideas

These are not final. They are examples to evaluate during implementation.

```rust
#[exec_time]
fn login() {}

#[exec_time(name = "user.login")]
fn login() {}

#[exec_time(print = "debug", min_duration = "5ms")]
fn read_cache() {}

#[exec_time(backend = "tracing", level = "info", warn_over = "100ms")]
async fn fetch_user() {}

#[exec_time(backend = "metrics", name = "db.query.time")]
fn run_query() {}
```

## Open Questions

- Should `stdout` remain the default, or should logging integration become the primary recommendation?
- How much configuration should live in macro attributes versus crate features?
- Should duration arguments be strings like `"100ms"` or integer values with a separate `unit` option?
- Should the crate stay intentionally lightweight, or should it grow into a small observability helper?
- Should metrics support be first-party or left for a separate crate?

## Suggested Implementation Order

1. Stabilize the macro implementation and add test coverage.
2. Clean up output wording and make the current `stdout` path better.
3. Add threshold support to reduce noise.
4. Add `tracing` integration.
5. Add `log` integration.
6. Evaluate whether metrics support belongs in this crate or a follow-up crate.
7. Rewrite the README and publish a new release after the API settles.
