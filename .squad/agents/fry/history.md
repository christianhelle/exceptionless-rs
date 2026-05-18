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

## Team Coordination

- 2026-05-18T10:43:35.499Z: Docs MVP structure merged to decisions.md. Leela confirmed first slice scope (error/log/feature → shared transport). Amy confirmed test harness blocks docs examples. Tier order decouples README (ready before code) from examples (ready after modules stable). Ready to draft README template when Bender scaffolds lib.rs.
