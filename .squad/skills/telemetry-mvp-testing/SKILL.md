---
name: "telemetry-mvp-testing"
description: "Behavior-first testing pattern for telemetry SDK MVPs"
domain: "testing"
confidence: "high"
source: "observed"
---

## Context

Use this when an SDK sends telemetry events through a shared submission path and the first milestone only covers a few event types. The safest first coverage proves wire-visible behavior and queue outcomes before the implementation grows many builders and helpers.

## Patterns

- Start with one fake submission transport that captures serialized batches and can return canned responses.
- Write one happy-path acceptance test per event type before adding breadth.
- Lock down the shared event envelope with contract assertions or snapshots for canonical payloads.
- Put disabled-config and submission-contract checks in a separate regression file immediately; add retry, batching, and queue-drain checks there once a queue exists.
- Assert observable behavior: captured payload fields, batch counts, queue count, and retry outcome. Do not couple tests to internal structs or plugin order.

## Examples

- `tests/support/mod.rs` provides a captured transport and payload helper.
- `tests/acceptance_errors.rs`, `tests/acceptance_logs.rs`, and `tests/acceptance_feature_usage.rs` each prove one event contract end to end.
- `tests/regression_submission_path.rs` proves endpoint/auth shaping plus no-send gates for a direct-submit MVP before queue behavior exists.
- `tests/regression_submission_queue.rs` can later prove retries and oversized-batch handling once batching lands.

## Anti-Patterns

- Testing builders only by inspecting private fields.
- Duplicating transport fakes per feature instead of sharing one harness.
- Declaring parity based on happy-path submission without retry and batching coverage.
- Waiting for queue implementation before proving request shaping and config no-send behavior.
- Treating serializer changes as low risk because the public API stayed the same.
