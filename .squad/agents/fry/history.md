# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: Initial docs should explain the Rust-first API while clearly calling out which Exceptionless capabilities are already implemented versus planned.
- 2026-05-18T10:43:35.499+02:00: MVP docs are three-tiered: README (landing + first example), lib.rs crate docs (API narrative for docs.rs), and examples/ (50-line runnable patterns). This tier order decouples README completeness from example readiness.
- 2026-05-18T10:43:35.499+02:00: Example naming: `{feature}_{style}.rs` (e.g., `error_basic.rs`, `config_from_env.rs`). Start with three core examples (error, log, feature) before expanding.
- 2026-05-18T10:43:35.499+02:00: Exceptionless.Net README references external docs heavily; Rust SDK should do the same but lead with "here's what works now" to avoid new-user paralysis.
- 2026-05-18T10:43:35.499+02:00: First example is part of the product. If setup feels confusing or requires external docs lookup, SDK is not ready. README must contain complete minimal example.
- 2026-05-18T10:43:35.499+02:00: README completed with three-tiered structure: Quick Start (copy-paste ready), Configuration (custom server + disable toggle), and Examples (four runnable patterns covering error, log, feature, and config). Examples verified to compile; tokio added as dev-dependency (macros + rt-multi-thread). All core MVP capabilities (error/log/feature with tags/data/identity) documented with inline code.
- 2026-05-18T11:44:38.999+02:00: Renamed `Client` to `ExceptionlessClient` throughout the codebase to match Exceptionless.NET naming conventions. Updated README with all import statements and usage examples; updated all four example files (error_basic, log_structured, feature_track, config_custom); updated lib.rs exports; updated builder.rs, error.rs, and client.rs struct definitions. All examples and tests compile successfully.
- 2026-05-19T23:57:10.867+02:00: Completed crates.io & docs readiness audit. Found: edition "2024" is invalid (blocker); missing Cargo.toml fields (description, categories, keywords); no crate-level lib.rs docs; no CHANGELOG.md; missing CONTRIBUTING.md, SECURITY.md. README quality good, examples solid. Audit saved to AUDIT_CRATES_IO_READINESS.md with prioritized fix list (blockers first, then pre-release, then optional post-release).
- 2026-05-20T00:16:20.423+02:00: Crates.io/docs slice shipped in `Cargo.toml`, `README.md`, and `src/lib.rs`. Package metadata now includes description/docs/homepage/readme/keywords/categories plus packaging excludes for `.copilot`, `.github`, `.squad`, `plan.md`, and the audit note; `cargo package --allow-dirty` dropped package contents from 175 files to 40.
- 2026-05-20T00:16:20.423+02:00: Crate docs contract: README and `src/lib.rs` must explicitly lead with the supported MVP surface (errors, logs, feature usage), mention the async runtime requirement (`tokio` in examples), and avoid implying silent disabled mode or automatic retries. Good first-task guidance also tells users that `send().await` returns `SubmissionResult` they can inspect.
- 2026-05-20T14:53:27.948+02:00: Release docs now describe one `release.yml` workflow with two manual operator paths: prepare release creates the GitHub prerelease and artifact bundle, while publish consumes the prior `release_tag` as the only publish identity. README wording should keep both paths explicitly default-branch-only and avoid referencing a separate publish workflow once the merge lands.

## Team Coordination

- 2026-05-18T10:43:35.499Z: Docs MVP structure merged to decisions.md. Leela confirmed first slice scope (error/log/feature → shared transport). Amy confirmed test harness blocks docs examples. Tier order decouples README (ready before code) from examples (ready after modules stable). Ready to draft README template when Bender scaffolds lib.rs.
- 2026-05-18T10:43:35.499+02:00: README and examples delivered. Four example files created (error_basic, log_structured, feature_track, config_custom); all compile clean. README explains "what's supported" vs "not yet supported", covers three configuration patterns (basic API key, custom server, disable toggle), and includes complete copy-paste-ready minimal examples inline. Decision on documentation direction written to inbox.
- 2026-05-18T11:44:38.999+02:00: Docs updated for Client → ExceptionlessClient rename. User-facing surfaces (README, examples) now reflect idiomatic naming that matches Exceptionless.NET client. Type renamed in all internal references (builder.rs, error.rs, client.rs, lib.rs). Verified: examples compile clean, tests pass, documentation consistent.
- 2026-05-18T13:31:36.000Z: Added eight screenshots from `images/` inline under Error, Log, and Feature sections in README.md using repo-relative Markdown image links. Enhanced visual documentation for first-time user onboarding. Commit: `2b7851f docs: add screenshots to error, log, and feature sections`.
- 2026-05-18T13:54:28Z: Repository renamed from exceptionless.rust to exceptionless-rs. Leela updated GitHub, Cargo.toml, README, and team metadata. Fry updated all documentation references. Commit: fedea6d refactor: rename repository to exceptionless-rs.

- 2026-05-20T14:53:27.948+02:00: README release guidance should describe a single `.github/workflows/release.yml` workflow with **Prepare release** and **Publish existing tag** paths, keep both paths explicitly default-branch-only, and keep `release_tag` documented as the publish source of truth.

- 2026-05-20T12:53:27.948Z: README release docs were refreshed to match the merged workflow shipped in commit `2bdfb5d` (`fix(ci): merge release workflow`). Operator guidance now points to one workflow with two manual paths and keeps the branch/environment and `release_tag` guardrails explicit.
