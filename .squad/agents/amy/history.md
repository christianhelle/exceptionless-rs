# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: The first Rust milestone needs coverage for errors, logs, and feature usage, with parity-minded tests around the shared submission path.
- 2026-05-18T10:43:35.499+02:00: Treat errors, logs, and feature usage as one shared submission slice; a fake transport plus queue flush helper should be the first Rust test harness.
- 2026-05-18T10:43:35.499+02:00: Reference parity points live in Exceptionless.Net `src/Exceptionless/Extensions/ExceptionlessClientExtensions.cs`, `src/Exceptionless/ExceptionlessClient.cs`, `src/Exceptionless/Extensions/ExceptionExtensions.cs`, and `src/Exceptionless/Queue/DefaultEventQueue.cs`.
- 2026-05-18T10:43:35.499+02:00: First-slice acceptance coverage now lives in `tests/support/mod.rs`, `tests/acceptance_errors.rs`, `tests/acceptance_logs.rs`, `tests/acceptance_feature_usage.rs`, and `tests/regression_submission_path.rs`.
- 2026-05-18T10:43:35.499+02:00: Concrete top-level error type names are preserved by the generic `Client::error<E>()` path in `src/client.rs` and `src/error.rs`; nested `source()` errors still only expose trait-object typing through the standard library API.
- 2026-05-18T11:44:38.999+02:00: Public-type rename work needs tests and example compilation to move together; `cargo test --all-targets` catches stale public API uses across integration tests and example binaries.

## Team Coordination

- 2026-05-18T10:43:35.499Z: Test gate proposal merged to decisions.md. Leela confirmed first slice scope. Fry confirmed docs won't block testing. Ready to scaffold test infrastructure when Bender/Farnsworth lay out module structure.
- 2026-05-18T12:28:06Z: Bender completed backtrace capture refactor and Client → ExceptionlessClient rename. Amy analyzed stack-trace test coverage gaps and specified three required regression tests. Fry updated all documentation and examples. Three commits delivered: real backtrace frames, test updates, and deepened examples. Decision inbox merged to decisions.md.
