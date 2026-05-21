---
name: "api-rename-feedback-loop"
description: "Fast diagnosis pattern for cargo test failures caused by stale public API call sites"
domain: "testing"
confidence: "high"
source: "observed"
---

## Context

Use this when a Rust crate just renamed a public method or type and `cargo test` starts failing. The goal is to build a deterministic compile-time loop first, fix the stale call sites at the correct observable seam, then rerun the broader proof matrix.

## Patterns

- Start with `cargo test` to capture the exact compiler symptom and the failing files.
- Tighten the loop immediately to the affected test targets with `cargo test --test ...` so compile-time failures return in about a second.
- Search for the old API spelling across source, docs, examples, and tests, but fix only the slice that is still wrong.
- Prefer acceptance/regression tests as the seam for rename fixes when those tests already prove the user-visible behavior.
- After the focused loop passes, rerun the rename-proof matrix that covers default targets, feature variants, examples, and rustdoc.

## Examples

- `tests/acceptance_errors.rs` and `tests/regression_error_stack_trace.rs` still called `.error(...)` after `ExceptionlessClient::capture_error(...)` became the public API.
- A good fast loop was `cargo test --test acceptance_errors --test regression_error_stack_trace`.
- The closing proof for this slice was `cargo test --all-targets`, `cargo test --all-targets --features opt-out`, `cargo build --examples`, and `cargo doc --no-deps`.

## Anti-Patterns

- Guessing about the cause before capturing the actual compiler error.
- Jumping straight to a full suite rerun for every iteration when two targeted test crates reproduce the failure faster.
- Adding new regression files when the existing acceptance/regression seam already exercises the renamed API.
- Fixing unrelated historical references in planning artifacts as part of the code slice.
