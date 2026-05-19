# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

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
