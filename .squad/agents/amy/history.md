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
- 2026-05-19T23:57:10.867+02:00: Validation surface: 13 integration tests (all passing), 4 examples (compile), 0 unit tests, 0 doc comments, 0 doc tests. Blocking for release: Cargo.toml edition "2024" invalid, 9 format violations, 2 clippy warnings, no CI/CD, missing package metadata, no MSRV. Test matrix has acceptance coverage for error/log/feature happy paths and one regression path (submission endpoint/auth). Missing: config unit tests, retry/queue behaviors, transport error recovery, doc coverage.
- 2026-05-20T00:16:20.423+02:00: The current release-proof command set is `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`, and `cargo package --allow-dirty`; the suite now passes, but `cargo package` still emits a non-blocking warning that `Cargo.toml` lacks a `description`.
- 2026-05-20T01:03:55.309+02:00: Publish automation must treat the scaffolded `release_tag` as the canonical version source; recomputing from current `RELEASE_BASE_VERSION` can strand older prerelease tags. Keep `CARGO_REGISTRY_TOKEN` scoped to the real publish step, and do not weaken the verified cargo gate from `cargo test --all-targets` when moving from release scaffolding to publish.

## Team Coordination

- 2026-05-18T10:43:35.499Z: Test gate proposal merged to decisions.md. Leela confirmed first slice scope. Fry confirmed docs won't block testing. Ready to scaffold test infrastructure when Bender/Farnsworth lay out module structure.
- 2026-05-18T12:28:06Z: Bender completed backtrace capture refactor and Client → ExceptionlessClient rename. Amy analyzed stack-trace test coverage gaps and specified three required regression tests. Fry updated all documentation and examples. Three commits delivered: real backtrace frames, test updates, and deepened examples. Decision inbox merged to decisions.md.
- 2026-05-19T23:57:10.867+02:00: Amy audited the validation surface and defined release gates. Found 13 integration tests (all passing), 4 working examples, but blocking issues: Cargo.toml edition invalid ("2024" does not exist), 9 format violations, 2 clippy warnings, no CI/CD workflow, no doc comments, no unit tests, missing package metadata, no MSRV declared. Documented gaps, release checklist, and recommended cadence. Decision written to decisions/inbox/amy-validation-surface-analysis.md.
- 2026-05-20T00:16:20.423+02:00: Amy reviewed commit `d79ae32` (`fix: green cargo release gates`) and approved it. The diff is behavior-preserving (import ordering plus equivalent `let`-chain collapse in `extract_message`), and the local release-proof commands all passed; only a non-blocking `cargo package` warning about missing manifest description remains.
- 2026-05-19T23:03:55Z: Farnsworth landed `add-publish-workflow` as commit `e5fcf8e` (`feat: add crates publish workflow`). Amy is now reviewing `.github/workflows/publish.yml` for release-tag pinning, shared version-suffix reuse, dry-run-before-token publish, and the owner follow-up to lock down the `release` environment secret on the default branch.
