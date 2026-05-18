# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: Audit the .NET client early to capture transport, buffering, and payload-shaping behavior before implementation spreads.
- 2026-05-18T10:43:35.499+02:00: First transport slice uses `ClientConfig -> SubmissionRequest -> Transport` so tests can fake transport while asserting exact endpoint/auth/payload wire fields.
- 2026-05-18T10:43:35.499+02:00: Wire payload lives in `src/wire/event.rs` and `src/wire/error.rs`; `Event::log` trims/guards `@level`, `Event::feature_usage` maps feature name to `source`, and errors serialize under `@error`.
- 2026-05-18T10:43:35.499+02:00: HTTP submission is in `src/transport/http.rs` and posts JSON arrays to `/api/v2/events` with Bearer auth; response handling classifies success, retry, split-and-retry (413), and discard in `src/transport/response.rs`.

## 2026-05-18T10:43:35Z
**Scribe Team Update:**
Delivered transport MVP contract with fakeable Transport trait, SubmissionRequest model, and wire contract locked for POST /api/v2/events with Bearer auth.
