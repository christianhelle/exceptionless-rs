# Rust MVP Acceptance + Regression Test Matrix

**Date:** 2026-05-18T10:43:35.499+02:00  
**Owner:** Amy  
**Scope:** First Rust SDK slice for errors, logs, and feature usage

## What the first test harness must observe

- Capture outbound event batches without hitting the real Exceptionless service.
- Force queue flushes and canned transport outcomes (success, retryable failure, oversized batch).
- Assert serialized payload fields and batch counts as black-box behavior, not internal structs.

## Acceptance coverage to land first

| Priority | Area | Behavior to prove | Observable assertions | First file |
| --- | --- | --- | --- | --- |
| P0 | Errors | Reporting an error submits exactly one `error` event through the shared pipeline. | One captured event batch; event `type = error`; payload includes error message, error type, and stack information; event gets a timestamp. | `tests\\acceptance_errors.rs` |
| P0 | Errors | Context added by callers survives serialization. | Tags, user identity, version, and custom data appear in the submitted payload. | `tests\\acceptance_errors.rs` |
| P0 | Logs | Creating a log event produces the same contract shape as the reference client. | Event `type = log`; `message` present; optional `source` preserved; `@level` present only when a non-blank level is supplied. | `tests\\acceptance_logs.rs` |
| P0 | Feature usage | Tracking a feature creates a usage event with the feature name as the source. | Event `type = usage`; `source = <feature name>`; no accidental fallback to log/error types. | `tests\\acceptance_feature_usage.rs` |
| P0 | Shared submission path | A manual flush drains queued events and hands batched payloads to the transport. | Queue count drops to zero after successful flush; transport sees expected number of batches and events. | `tests\\regression_submission_queue.rs` |
| P0 | Safety gate | Disabled or invalid configuration does not pretend to succeed. | No transport call; no queued event growth; caller gets a visible no-op/error result consistent with the API design. | `tests\\regression_submission_queue.rs` |

## Highest-risk regression cases

| Priority | Risk | Why it matters | Observable regression to lock down | File |
| --- | --- | --- | --- | --- |
| P0 | Payload drift on shared event envelope | Errors, logs, and usage all ride the same submission path, so one serializer change can break all three. | Snapshot or field-by-field contract tests for one canonical error, log, and usage payload. | `tests\\regression_payload_contract.rs` |
| P0 | Inner error loss | Nested exceptions are core debugging value and easy to flatten incorrectly. | Submitted error preserves inner exception chain order and messages. | `tests\\acceptance_errors.rs` |
| P1 | Duplicate / dirty tags | Reference client de-duplicates tags; noisy tags create parity gaps quickly. | Duplicate tags collapse to one logical tag; null/empty tags are ignored. | `tests\\acceptance_errors.rs` |
| P1 | Whitespace log levels | Reference behavior trims level strings and omits blank levels. | `" Warn "` becomes `Warn`; blank/whitespace level is absent from payload. | `tests\\acceptance_logs.rs` |
| P1 | Retryable transport failure | Network failures must not silently drop telemetry. | Failed batch stays queued after retryable failure and is retried on the next flush. | `tests\\regression_submission_queue.rs` |
| P1 | Oversized batch response | Batch splitting is a common real-world failure mode. | Oversized batch response causes a retry with a smaller batch instead of dropping all events. | `tests\\regression_submission_queue.rs` |
| P1 | Timestamp defaults | Missing event dates create ingestion weirdness and parity gaps. | Event timestamp is auto-populated when caller does not set one. | `tests\\regression_payload_contract.rs` |
| P2 | Queue discard / suspension policy | Response-code handling can be subtle and should be added only after the basics are stable. | Explicit tests for whatever the Rust MVP chooses on auth failure / plan limit / bad request. | `tests\\regression_submission_queue.rs` |

## Recommended first test files

1. `tests\\support\\mod.rs` — fake transport, captured request sink, flush helper, deterministic fixtures.
2. `tests\\acceptance_errors.rs` — prove error event shape first, including inner errors and caller-added context.
3. `tests\\acceptance_logs.rs` — prove `log` contract, source handling, and level normalization.
4. `tests\\acceptance_feature_usage.rs` — prove `usage` contract and source mapping.
5. `tests\\regression_submission_queue.rs` — prove flush, batching, retry, and no-send gates.
6. `tests\\regression_payload_contract.rs` — canonical payload fixtures/snapshots for error, log, and usage events.

## Order I would block on

1. Acceptance tests for one happy-path error, log, and feature event.
2. Regression coverage for disabled/invalid config and successful queue drain.
3. Regression coverage for retryable failure and oversized batch handling.
4. Snapshot/contract locks once the wire format stops moving daily.

## Reference parity notes

- `.NET` log creation sets `type = log`, copies `message`, keeps optional `source`, and writes `@level` only for non-blank values.
- `.NET` feature tracking sets `type = usage` and stores the feature name in `source`.
- `.NET` error reporting uses the same submission path, auto-populates event date, and preserves inner exceptions plus extra data where available.
- Shared queue behavior in the reference client retries transient failures, can shrink oversized batches, and avoids shipping when configuration is disabled or invalid.
