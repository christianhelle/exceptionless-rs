---
name: "dependency-surface-slicing"
description: "Reduce Rust crate dependency surface by fixing packaging boundaries before swapping leaf crates"
domain: "rust, architecture, packaging"
confidence: "high"
source: "earned"
---

## Context
Use this pattern when a Rust library wants fewer transitive dependencies without guessing. The fastest path is to identify which public/default boundaries force the heaviest subtree into every consumer.

## Patterns
- Count direct runtime dependencies, lockfile packages, and the unique normal-tree package count.
- Bucket each direct dependency by responsibility: transport, stack traces, time, JSON/public data model, error ergonomics.
- Identify which dependencies are pulled in by default public entrypoints, README examples, and regression tests.
- Ask the first user question about the boundary that can delete the largest subtree, usually whether built-in HTTP stays in the default crate or becomes opt-in.
- Defer smaller crate swaps until MSRV, semver tolerance, and public API compatibility are explicit.

## Examples
- `C:\projects\christianhelle\exceptionless-rs\Cargo.toml`
- `C:\projects\christianhelle\exceptionless-rs\src\client.rs`
- `C:\projects\christianhelle\exceptionless-rs\src\config.rs`

## Anti-Patterns
- Replacing `reqwest` before deciding whether HTTP belongs in the core crate at all.
- Treating public leaks like `serde_json::Value` or `chrono::DateTime<Utc>` as if they were purely internal.
- Chasing small proc-macro removals before resolving the main packaging boundary.
