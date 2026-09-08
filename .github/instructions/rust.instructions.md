---
description: 'Rust coding conventions - style, module layout, error handling, logging, configuration and performance.'
applyTo: '**/*.rs'
---

# Rust

## Style (enforced by `rustfmt` and `clippy`)

- **`cargo fmt` is authoritative.** Never hand-format Rust source; run `cargo fmt --all` before committing. CI fails on `cargo fmt --all --check`.
- **`cargo clippy -- -D warnings` must pass.** Treat every clippy lint as an error, not a suggestion. Prefer fixing the code over `#[allow(...)]`; when a suppression is genuinely warranted, scope it to the narrowest item and add a comment explaining why.
- **Edition**: 2021. **Indentation**: 4 spaces, LF line endings, final newline.
- **Naming**: `snake_case` for functions, methods, variables, modules and crates; `PascalCase` for types, traits and enum variants; `SCREAMING_SNAKE_CASE` for `const` and `static`.
- **Module-per-concern**: one focused responsibility per module file (`config.rs`, `telemetry.rs`, `worker.rs`). Modules are the Rust analogue of the sibling .NET repository's one-type-per-file rule - do not accumulate unrelated types in `main.rs`.
- **`main.rs` is wiring only**: load configuration, install the logging subscriber, register shutdown handling, hand off to a worker. Business logic belongs in a module.
- **Visibility is deliberate**: default to private. Mark an item `pub` only when another module genuinely needs it, `pub(crate)` when the need does not cross the crate boundary.
- **`use` ordering**: `std` first, then external crates, then `crate`/`self`/`super`, each group separated by a blank line (rustfmt's default grouping). Import types, not whole modules, unless the module qualifier aids readability (e.g. `std::env::consts::ARCH`).
- **Prefer iterators and combinators** (`map`, `filter`, `and_then`, `unwrap_or_else`) over index loops and nested `match` where they read more clearly. Stop when the chain becomes harder to read than an explicit loop.
- **Borrow, don't clone.** Take `&str` over `String` and `&[T]` over `Vec<T>` in function parameters. Reach for `.clone()` only when ownership genuinely needs to move; never to silence the borrow checker.
- **Newtypes for domain values.** Wrap primitives that carry meaning (identifiers, units, currencies) in a tuple struct rather than passing bare `String`/`u64` around.
- **Derive rather than implement**: `#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]` wherever the derived behaviour is correct. `Debug` on every public type, without exception.
- **`impl Default`** for configuration structs so the application runs with zero configuration supplied.

## Error Handling

- **No `unwrap()` or `expect()` outside tests, `const` initialisers and `main`.** Return `Result<_, E>` and propagate with `?`.
- **`main` returns `Result<(), Box<dyn Error>>`** so startup failures surface as a non-zero exit code and a printed error rather than a panic backtrace.
- **Library-style modules define their own error type** (`thiserror::Error` when a dependency is acceptable); binaries may use `Box<dyn Error>` or `anyhow::Result` at the boundary.
- **Add context when propagating.** A bare `?` that loses the file name or key being processed makes production failures unreadable - wrap with `map_err` or `anyhow::Context`.
- **Keep error context non-sensitive.** Name the failed operation and safe identifier, but never include credentials, tokens, connection strings, full local paths or personally identifying values.
- **Panics are for programmer errors only** (violated invariants), never for bad input, missing configuration or I/O failure.

## Logging & Instrumentation

- **`tracing` is the logging framework**, not `log` or `println!`. It is the Rust analogue of Serilog in the sibling .NET repository.
- **Structured fields, not interpolated strings.** Write `info!(user_id = %id, count = items.len(), "processed batch")` - never `info!("processed {} items for {}", items.len(), id)`. Fields are queryable in Loki/Elasticsearch; interpolated text is not.
- **Field names are `snake_case`** and match the sibling repositories' field names where the concept is shared (`git_repository`, `interval_seconds`, `process_architecture`).
- **Sigils**: `%value` for `Display`, `?value` for `Debug`, bare for primitives. Do not `format!` a value just to log it.
- **The message is a constant.** Keep the human-readable part of a `tracing` macro a static string so events group correctly; put every varying part in a field.
- **`#[instrument]` on meaningful units of work** (request handlers, long-running tasks) - not on trivial getters, where the span overhead outweighs the value.
- **Subscriber setup lives in one place** (`telemetry.rs`) and is installed exactly once from `main`. Application code never touches `tracing_subscriber`.
- **Verbosity via `RUST_LOG`**, parsed with `EnvFilter`, defaulting to `info`.
- **Never record secrets or personal data.** Credentials, tokens, connection strings, full local paths and personally identifying values must never become event fields or span attributes.
- **Keep hot-path fields cheap.** Avoid expensive `Debug` formatting or allocating temporary strings solely for events that may be filtered out.

## Configuration

- **Layered, file-then-environment**, using the `config` crate - the Rust analogue of `Microsoft.Extensions.Configuration`.
- **Every setting has a default** via `impl Default`, so the binary runs with no file and no environment variables present.
- **Deserialise into a typed struct** with `serde`; never read `std::env::var` scattered through the code base.
- **Keys are `snake_case`** in both the file and the environment. The `config` crate lower-cases environment keys but preserves file keys verbatim, so `snake_case` is the only casing where both sources resolve to the same key.
- **Section separator is `__`** in environment variables (`APP__INTERVAL_SECONDS`), matching the sibling .NET and Go repositories.
- **Validate at startup.** A configuration error must abort the process immediately with a clear message, never surface later as a runtime surprise.

## Async

- **Tokio is the runtime.** Enable only the features actually used (`macros`, `rt-multi-thread`, `sync`, `time`) rather than `full` - it materially affects compile time and binary size.
- **Never block the async runtime.** Use `tokio::time::sleep`, not `std::thread::sleep`; move CPU-bound work to `spawn_blocking`.
- **Cancellation is explicit.** Long-running loops select over their work and a shutdown signal (`tokio::select!` with a `watch` receiver or `CancellationToken`) so SIGINT/SIGTERM stop them promptly.
- **Every spawned task has an owner and exit path.** Propagate cancellation, retain its `JoinHandle` when completion matters, and await owned tasks during graceful shutdown.
- **Do not hold a `std::sync::Mutex` guard across an `.await`.** Use `tokio::sync::Mutex` when a lock must span a suspension point, and prefer message passing over shared mutable state.

## Testing

- **Unit tests live in the module they test**, in a `#[cfg(test)] mod tests` block at the bottom of the file. Integration tests live in `tests/`.
- **Test names describe the behaviour**, not the function - `load_falls_back_to_defaults_when_file_missing`, not `test_load`.
- **Arrange/Act/Assert**, separated by blank lines. One behaviour per test.
- **Assertion messages carry the actual value**: `assert_eq!(got, want, "greeting = {got}")`.
- **No shared mutable state between tests** - they run in parallel by default and must be independently repeatable.
- **Test stable module behaviour**, not incidental implementation details, unless a private helper contains genuinely complex logic.
- **Use synthetic credentials and identifiers.** Failure output and CI artifacts must not expose real secrets or personal data.

## Documentation

- **`///` doc comments on every public item** (module, struct, enum, trait, function, field). `//!` module-level comments explain what the module is for.
- **First line is a single-sentence summary**; further detail goes in following paragraphs.
- **Link related items** with intra-doc links (`` [`AppConfig`] ``) and external references with `<https://...>`.
- **Do not delete hyperlinks** to blog posts, issues or StackOverflow answers when refactoring a comment - move them into the doc comment.
- **`# Errors` and `# Panics` sections** on public functions that return `Result` or can panic.

## Performance

- **Measure before optimising.** Prefer clear code; reach for `criterion` benchmarks before micro-optimising.
- **Avoid allocation in hot loops**: reuse buffers, prefer `&str` slicing over `String` concatenation, and use `Vec::with_capacity` when the size is known.
- **`#[inline]` sparingly** - only on small cross-crate functions where profiling shows a benefit.
- **`Cow<'_, str>`** when a function usually returns borrowed data but occasionally needs to own it.
- **Release profile hardening** for binaries where size matters: `strip = true`, `lto = "thin"`, `codegen-units = 1`, `panic = "abort"`.

## Dependencies

- **Prefer the standard library.** Every crate is compile time, binary size and supply-chain surface; add one only when it earns its place.
- **Enable minimal features.** Default features are frequently broader than needed - use `default-features = false` plus an explicit list where it helps.
- **`Cargo.lock` is committed** for binaries, and builds use `--locked` so CI cannot silently drift.
- **Version requirements stay loose** (`"1"`, `"0"`) in `Cargo.toml`; `Cargo.lock` pins the exact resolution and Dependabot updates it.
