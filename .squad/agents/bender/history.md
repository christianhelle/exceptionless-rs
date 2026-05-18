# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: The first milestone targets errors, logs, and feature usage with an idiomatic Rust surface.
- 2026-05-18T10:43:35.499+02:00: Public SDK calls now land on `Client::error`, `Client::log`, and `Client::feature`, each returning typed builders that share send logic through `src/builder.rs`.
- 2026-05-18T10:43:35.499+02:00: `src/event.rs` wraps `wire::event::Event` to preserve Rust ergonomics while keeping submission compatibility with `transport::SubmissionRequest::from_events`.

## 2026-05-18T10:43:35Z
**Scribe Team Update:**
Fixed config normalization defect and delivered SDK first-slice facade with Client::{error,log,feature} builders and shared submit_batch send path.
