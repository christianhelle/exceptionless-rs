# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-21T22:42:11.831+02:00: Completed API rename of ExceptionlessClient::error() to capture_error() across src/client.rs, src/lib.rs, src/error.rs, src/builder.rs, and src/event.rs; all rustdoc cross-references updated; doc test suite (26 tests) passes cleanly. Tests in tests/*.rs remain authored by Fry/Amy per charter boundary and are awaiting their update slice.
- 2026-05-21T22:37:26.919+02:00: Renaming `ExceptionlessClient::error` requires a full public-surface sweep across the method definition in `src\client.rs`, rustdoc cross-references in `src\builder.rs`, `src\error.rs`, `src\event.rs`, and `src\lib.rs`, plus user-facing call sites in `tests\acceptance_errors.rs`, `tests\regression_error_stack_trace.rs`, `examples\error_basic.rs`, and `README.md`; `cargo test --doc` and example compilation are part of the compatibility gate.
- 2026-05-20T10:28:01.000+02:00: The first no-risk dependency-minimization slice is safe only if `src\config.rs`, `src\error.rs`, and `src\transport\mod.rs` preserve exact `Display`, `Error::source`, and `From` behavior after removing direct `thiserror` usage, while the `reqwest` `json` feature can be dropped because `src\transport\http.rs` already serializes and parses through `serde_json` plus raw request bodies.
- 2026-05-20T10:28:01.000+02:00: The no-risk dependency-cut pattern for this crate is to keep behavior in `src\config.rs`, `src\error.rs`, and `src\transport\mod.rs` byte-for-byte compatible at the message/API level while replacing `thiserror` derives with manual `Display`/`Error`/`From` impls, and only drop a reqwest feature after proving the code path already serializes via `serde_json` plus raw `.body(...)` in `src\transport\http.rs`. Christian's preference on this slice was strict scope control: run the existing release gate from `.github\workflows\ci.yml` before and after, avoid async-trait or transport-surface redesign, do not commit, and record team artifacts for the dependency-cut decision.
- 2026-05-20T09:59:21.307+02:00: Dependency-surface choke points for this crate live in `Cargo.toml`, `src\transport\mod.rs`, `src\transport\http.rs`, `src\builder.rs`, `src\error.rs`, and `src\wire\event.rs`; `thiserror` is pure internal sugar, `async-trait` leaks through the public `Transport` extension point, `reqwest` leaks through `HttpTransport` and the default client transport, `serde_json` leaks through all `.data(...)` builder methods, `chrono` leaks through the public `wire::event::Event` date field, and `backtrace` is internal but backs the documented automatic stack-trace behavior.
- 2026-05-20T09:59:21.307+02:00: The smallest safe dependency-reduction slice is to remove `thiserror` first; removing `reqwest` or `async-trait` from the default dependency set requires a product/API decision about whether this crate stays batteries-included with built-in HTTP transport or splits core transport abstractions from the reqwest-backed implementation.
- 2026-05-20T01:03:55.309+02:00: For manual crates.io publish, the scaffolded `release_tag` must be the only publish-version source; recomputing from mutable workflow settings can drift from the prerelease artifact that Amy actually reviewed.
- 2026-05-18T10:43:35.499+02:00: The first milestone targets errors, logs, and feature usage with an idiomatic Rust surface.
- 2026-05-18T10:43:35.499+02:00: Public SDK calls now land on `Client::error`, `Client::log`, and `Client::feature`, each returning typed builders that share send logic through `src/builder.rs`.
- 2026-05-18T10:43:35.499+02:00: `src/event.rs` wraps `wire::event::Event` to preserve Rust ergonomics while keeping submission compatibility with `transport::SubmissionRequest::from_events`.
- 2026-05-18T11:44:38.999+02:00: Public SDK naming now uses ExceptionlessClient in src/client.rs and the src/lib.rs re-export so the Rust facade matches Exceptionless.Net without changing the submission flow.
- 2026-05-18T11:44:38.999+02:00: Public client renames have directly coupled touch points in src/builder.rs, src/error.rs, tests/*.rs, examples/*.rs, and README.md; update them together or the crate stops compiling cleanly.
- 2026-05-20T00:16:20.423+02:00: Release-gate cleanup stayed behavior-preserving: `src\transport\http.rs` keeps the same JSON error-message extraction semantics but now uses a single let-chain to satisfy clippy, while cargo fmt reordered imports and wrapped long assertions in `src\client.rs`, `examples\config_custom.rs`, `tests\acceptance_errors.rs`, `tests\acceptance_feature_usage.rs`, `tests\acceptance_logs.rs`, `tests\regression_error_stack_trace.rs`, and `tests\regression_submission_path.rs`. Christian's priority for this slice was to green `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` without breaking `cargo test --all-targets`, `cargo doc --no-deps`, or `cargo package --allow-dirty`.
- 2026-05-20T00:16:20.423+02:00: CI for this repo can stay as one Ubuntu-only stable Rust job in `.github\workflows\ci.yml`, and the validated baseline gate is `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo doc --no-deps`, then `cargo package --allow-dirty` so the workflow matches the local release-readiness path without touching existing squad automation.
- 2026-05-20T01:03:55.309+02:00: The publish slice lives in `.github\workflows\publish.yml` with directly coupled release notes in `README.md`; the safe shape is to treat `release_tag` from `Release Scaffolding` as the only version source, keep `CARGO_REGISTRY_TOKEN` scoped to the final publish step, and preserve `cargo test --all-targets` in the publish gate.

## 2026-05-18T10:43:35Z
**Scribe Team Update:**
Fixed config normalization defect and delivered SDK first-slice facade with Client::{error,log,feature} builders and shared submit_batch send path.

## 2026-05-18T12:28:06Z
**Scribe Team Update:**
Completed real backtrace capture using `backtrace` crate with eager symbol resolution and two-level frame filtering. Renamed public type to ExceptionlessClient. Amy specified stack-trace coverage gaps (three new regression tests required). Fry updated all docs/examples. Three commits delivered. Decision inbox merged to decisions.md.

## 2026-05-19T23:03:55.309Z
**Scribe Team Update:**
Amy approved commit `c5359e0` (`fix(ci): trust scaffolded release tag`). The publish path now derives `publish_version` directly from `release_tag`, keeps `CARGO_REGISTRY_TOKEN` scoped to the final publish step, preserves `cargo test --all-targets`, and does not regress `.github/workflows/release.yml`. Owner follow-up remains to configure `CARGO_REGISTRY_TOKEN` on the `release` environment and restrict it to the default branch.
## 2026-05-20T10:28:01.000+02:00
**Scribe Team Update:**
Committed the first dependency-minimization slice as `12ee13024d03cecf53de3f72918f4665fff8e82c` (`refactor: remove thiserror and trim reqwest features`). Amy approved the change after adding focused error-contract regressions, and Leela cleared the final gate with the next slice constrained to the built-in HTTP packaging boundary.

## 2026-05-20T13:00:42.108+02:00
**Scribe Team Update:**
Bender revised Amy's rejected single `opt-out` feature slice under reviewer lockout, fixed `README.md`, `src/lib.rs`, and `tests/regression_submission_path.rs`, and committed the approved revision as `87393f6f063b5a9767f63681ff452eef5117917b` (`feat: add opt-out submission coverage`).
## 2026-05-21 22.48.09 UTC - Scribe: Cross-agent coordination

Team completed parallel refactoring of ExceptionlessClient::error() → capture_error():
- Bender: API source code changes committed
- Fry: Documentation and examples updated
- Amy: Tests updated and validation passed

All work integrated and ready for delivery.
