# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: The first milestone targets errors, logs, and feature usage with an idiomatic Rust surface.
- 2026-05-18T10:43:35.499+02:00: Public SDK calls now land on `Client::error`, `Client::log`, and `Client::feature`, each returning typed builders that share send logic through `src/builder.rs`.
- 2026-05-18T10:43:35.499+02:00: `src/event.rs` wraps `wire::event::Event` to preserve Rust ergonomics while keeping submission compatibility with `transport::SubmissionRequest::from_events`.
- 2026-05-18T11:44:38.999+02:00: Public SDK naming now uses ExceptionlessClient in src/client.rs and the src/lib.rs re-export so the Rust facade matches Exceptionless.Net without changing the submission flow.
- 2026-05-18T11:44:38.999+02:00: Public client renames have directly coupled touch points in src/builder.rs, src/error.rs, tests/*.rs, examples/*.rs, and README.md; update them together or the crate stops compiling cleanly.

## 2026-05-18T10:43:35Z
**Scribe Team Update:**
Fixed config normalization defect and delivered SDK first-slice facade with Client::{error,log,feature} builders and shared submit_batch send path.
