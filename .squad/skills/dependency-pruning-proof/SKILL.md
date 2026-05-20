---
name: "dependency-pruning-proof"
description: "Prove dependency removal is behavior-safe before calling it an internal cleanup"
domain: "testing, api-compatibility, maintenance"
confidence: "high"
source: "earned"
---

## Context
Use this when a crate wants fewer dependencies without surprising downstream users. The risky cases are dependencies that leak into public constructors, trait impl ergonomics, serialized payloads, or example code.

## Patterns
- Map each dependency to an observable contract before proposing removal: public types, builder signatures, serialized wire shape, runtime behavior, docs, and examples.
- Keep compile proof for every public entrypoint users copy from docs: doctests, runnable examples, and integration tests.
- Separate “internal dependency swap” from “public compatibility change”; require an explicit owner decision if a dependency appears in a public signature or extension point.
- For telemetry SDKs, keep black-box assertions on endpoint, auth header, payload shape, stack traces, and config no-send behavior while pruning internals.
- Add targeted regression tests for the exact behavior the removed dependency used to provide before calling the reduction safe.

## Examples
- `C:\projects\christianhelle\exceptionless-rs\src\transport\http.rs` exposes `HttpTransport::new(reqwest::Client)`.
- `C:\projects\christianhelle\exceptionless-rs\src\transport\mod.rs` exposes the async `Transport` trait used by `tests\support\mod.rs`.
- `C:\projects\christianhelle\exceptionless-rs\src\builder.rs`, `src\error.rs`, `src\feature.rs`, and `src\log.rs` expose `.data(... Into<serde_json::Value>)`.
- `C:\projects\christianhelle\exceptionless-rs\tests\acceptance_*.rs` and `tests\regression_*.rs` prove current wire-visible behavior.

## Anti-Patterns
- Calling a change “dependency-only” when it alters public signatures or the ergonomics of implementing extension traits.
- Trusting green happy-path tests while leaving docs/examples uncompiled.
- Removing a serialization, backtrace, or HTTP dependency without adding a regression for the behavior it was providing.
