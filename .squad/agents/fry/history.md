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

## Team Coordination

- 2026-05-18T10:43:35.499Z: Docs MVP structure merged to decisions.md. Leela confirmed first slice scope (error/log/feature → shared transport). Amy confirmed test harness blocks docs examples. Tier order decouples README (ready before code) from examples (ready after modules stable). Ready to draft README template when Bender scaffolds lib.rs.
- 2026-05-18T10:43:35.499+02:00: README and examples delivered. Four example files created (error_basic, log_structured, feature_track, config_custom); all compile clean. README explains "what's supported" vs "not yet supported", covers three configuration patterns (basic API key, custom server, disable toggle), and includes complete copy-paste-ready minimal examples inline. Decision on documentation direction written to inbox.
- 2026-05-18T11:44:38.999+02:00: Docs updated for Client → ExceptionlessClient rename. User-facing surfaces (README, examples) now reflect idiomatic naming that matches Exceptionless.NET client. Type renamed in all internal references (builder.rs, error.rs, client.rs, lib.rs). Verified: examples compile clean, tests pass, documentation consistent.
- 2026-05-18T13:31:36.000Z: Added eight screenshots from `images/` inline under Error, Log, and Feature sections in README.md using repo-relative Markdown image links. Enhanced visual documentation for first-time user onboarding. Commit: `2b7851f docs: add screenshots to error, log, and feature sections`.

