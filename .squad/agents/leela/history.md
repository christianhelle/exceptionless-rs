# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: Team initialized. Start with a greenfield Rust SDK and audit the .NET client before hardening public APIs.
- 2026-05-18T10:43:35.499+02:00: Exceptionless.Net's MVP surface converges on `src/Exceptionless/ExceptionlessClient.cs`, `src/Exceptionless/EventBuilder.cs`, and `src/Exceptionless/Extensions/ExceptionlessClientExtensions.cs`; errors, logs, and feature usage all become `Event` payloads with types `error`, `log`, and `usage`.
- 2026-05-18T10:43:35.499+02:00: The wire contract for the first Rust slice is anchored by `src/Exceptionless/Submission/DefaultSubmissionClient.cs`: POST a JSON event array to `/api/v2/events` with `Authorization: Bearer {api_key}` and a user agent, then treat 2xx responses as success.
- 2026-05-18T10:43:35.499+02:00: The repo is still scaffolding-only with no `Cargo.toml`, so the first implementation pass must create the crate and keep queue persistence, plugin parity, settings sync, and session plumbing out of the initial Rust public API.

## Team Coordination

- 2026-05-18T10:43:35.499Z: First slice decision merged to decisions.md. Amy agreed test harness needs fake transport + shared submission path. Fry agrees docs tier order decouples README from examples. Ready for Bender/Farnsworth build phase.
