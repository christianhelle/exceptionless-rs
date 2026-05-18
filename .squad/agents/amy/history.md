# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: The first Rust milestone needs coverage for errors, logs, and feature usage, with parity-minded tests around the shared submission path.
- 2026-05-18T10:43:35.499+02:00: Treat errors, logs, and feature usage as one shared submission slice; a fake transport plus queue flush helper should be the first Rust test harness.
- 2026-05-18T10:43:35.499+02:00: Reference parity points live in Exceptionless.Net `src/Exceptionless/Extensions/ExceptionlessClientExtensions.cs`, `src/Exceptionless/ExceptionlessClient.cs`, `src/Exceptionless/Extensions/ExceptionExtensions.cs`, and `src/Exceptionless/Queue/DefaultEventQueue.cs`.

## Team Coordination

- 2026-05-18T10:43:35.499Z: Test gate proposal merged to decisions.md. Leela confirmed first slice scope. Fry confirmed docs won't block testing. Ready to scaffold test infrastructure when Bender/Farnsworth lay out module structure.
