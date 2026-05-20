---
name: "dependency-floor-audit"
description: "Separate easy manifest cuts from dependency removals that would leak through transport or wire contracts"
domain: "integration, packaging"
confidence: "high"
source: "earned"
---

## Context
Use this when a crate wants the smallest believable dependency set without accidentally breaking consumers that rely on exposed transport or wire types.

## Patterns
- Map each direct dependency to one concrete responsibility in config, transport, wire, or public builders before proposing cuts.
- Split findings into three buckets: internal-only cuts, feature-gated cuts, and public-API/behavior-changing cuts.
- Treat dependencies as effectively public when public constructors, trait signatures, or exposed structs mention their types.
- Prefer removing unused crate features first; that trims transitive weight without changing the crate's story.
- Call out when a dependency is compile-time-only (`async-trait`, `thiserror`) versus runtime-observable (`reqwest`, `chrono`, `backtrace`), because the reduction strategy differs.

## Examples
- `src/transport/http.rs`: `reqwest` owns the default HTTP path and currently leaks through `HttpTransport::new(reqwest::Client)`.
- `src/wire/event.rs` and `src/wire/error.rs`: `chrono`, `serde`, and `serde_json` define the public payload model, so replacing them is not just a manifest edit.
- `src/transport/mod.rs`: `async-trait` sits on the public `Transport` seam, so removing it affects custom transport implementers.

## Anti-Patterns
- Calling a dependency "easy to remove" when its types appear in public signatures.
- Swapping out stack-trace or timestamp machinery without checking whether payload shape or semantics change.
- Counting only direct dependencies and ignoring large transitive trees added by enabled features.
