# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-19T23:57:10.867+02:00: Release readiness plan drafted. Identified 10 key ambiguities (MSRV, edition, token provisioning, semver strategy, cross-platform testing, docs site, changelog automation, approval gates, edition 2024 validation, dependency constraints). Proposed 7 slices across 3 phases: Phase 1 (test/docs/audit) is critical path, Phase 2 (version/publish) unblocks crates.io, Phase 3 (notify/security) deferred. Total 12–14 days with parallelization. Owner input required on all ambiguities before Slice 1a begins.
- 2026-05-18T10:43:35.499+02:00: Team initialized. Start with a greenfield Rust SDK and audit the .NET client before hardening public APIs.
- 2026-05-18T10:43:35.499+02:00: Exceptionless.Net's MVP surface converges on `src/Exceptionless/ExceptionlessClient.cs`, `src/Exceptionless/EventBuilder.cs`, and `src/Exceptionless/Extensions/ExceptionlessClientExtensions.cs`; errors, logs, and feature usage all become `Event` payloads with types `error`, `log`, and `usage`.
- 2026-05-18T10:43:35.499+02:00: The wire contract for the first Rust slice is anchored by `src/Exceptionless/Submission/DefaultSubmissionClient.cs`: POST a JSON event array to `/api/v2/events` with `Authorization: Bearer {api_key}` and a user agent, then treat 2xx responses as success.
- 2026-05-18T10:43:35.499+02:00: The repo is still scaffolding-only with no `Cargo.toml`, so the first implementation pass must create the crate and keep queue persistence, plugin parity, settings sync, and session plumbing out of the initial Rust public API.
- 2026-05-18T10:43:35.499+02:00: Reviewer gate for slice 1 found a config-boundary bug in `src/config.rs`: `validate()` trims `server_url` for parsing, but `events_url()` builds the submission endpoint from the untrimmed string, so whitespace-wrapped URLs pass validation and still yield invalid `/api/v2/events` endpoints for all MVP event types.
- 2026-05-18T10:43:35.499+02:00: The current acceptance suite (`tests/acceptance_errors.rs`, `tests/acceptance_logs.rs`, `tests/acceptance_feature_usage.rs`) proves the shared event envelope and the regression harness in `tests/regression_submission_path.rs` already covers endpoint/auth shaping; the next regression to add is normalized `server_url` handling.
- 2026-05-18T10:43:35.499+02:00: Review of the revised `src/config.rs` and `tests/regression_submission_path.rs` confirmed the config boundary now reuses one normalized `server_url` value for both validation and `events_url()` construction, and the whitespace-wrapped URL regression is locked down end to end. `cargo test --test regression_submission_path` and full `cargo test` both passed, so slice 1 can continue without config-related blocking caveats.

## Team Coordination

- 2026-05-18T10:43:35.499Z: First slice decision merged to decisions.md. Amy agreed test harness needs fake transport + shared submission path. Fry agrees docs tier order decouples README from examples. Ready for Bender/Farnsworth build phase.
- 2026-05-18T13:54:28Z: Repository renamed from exceptionless.rust to exceptionless-rs. Leela updated GitHub, Cargo.toml, README, and team metadata. Fry updated all documentation references. Commit: fedea6d refactor: rename repository to exceptionless-rs.
