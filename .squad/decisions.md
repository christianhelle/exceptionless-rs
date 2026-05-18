# Squad Decisions

## Active Decisions

### 2026-05-18T10:43:35.499+02:00: Initial squad roster
**By:** Christian Helle (confirmed via Copilot)
**What:** The project uses a Futurama-cast squad with Leela as Lead, Bender as Rust SDK Dev, Farnsworth as Integration Dev, Amy as Tester, Fry as Docs/DevRel, plus Scribe and Ralph.
**Why:** The repository started with Squad scaffolding but no configured roster, so named ownership and routing had to be established before implementation work could begin.

### 2026-05-18T10:43:35.499+02:00: MVP direction
**By:** Christian Helle (confirmed via Copilot)
**What:** Build an idiomatic Rust client that uses Exceptionless.Net as the behavioral reference, with the first milestone covering error reporting, logs, and feature usage.
**Why:** This keeps the Rust SDK ergonomic for Rust users while focusing early work on the highest-value Exceptionless capabilities.

### 2026-05-18T10:43:35.499+02:00: First Rust SDK slice
**By:** Leela
**What:** Build one async crate proving three end-to-end calls (error/log/feature) converging on shared internal event model and JSON array submission to POST /api/v2/events with Bearer auth. Files split: Bender owns client/event/builder/error/log/feature; Farnsworth owns config/transport/wire. Public API: `client.error(&err).send().await?`, `client.log("msg").level(...).send().await?`, `client.feature("name").send().await?`.
**Why:** Smallest vertical slice to prove transport and behavioral parity without queue, plugins, settings, or sessions. Focuses on highest-value MVP capabilities before expanding scope.

### 2026-05-18T10:43:35.499+02:00: MVP telemetry test gate
**By:** Amy
**What:** Black-box contract coverage for shared submission pipeline: fake transport for tests, acceptance tests for error/log/feature events, regression coverage for config/queue/retry/batch edge cases.
**Why:** These three features feed the same event envelope. Payload shape, queue behavior, and retry handling must not drift in production.

### 2026-05-18T10:43:35.499+02:00: Docs/DevRel MVP structure
**By:** Fry
**What:** Three-tier documentation: README.md (landing + quick start + links), lib.rs crate doc (module narrative), examples/ (error_basic, error_with_context, log_structured, feature_track, config_from_env, config_custom). Tier order prevents blocking; README before code ships.
**Why:** New users land on README, developers use docs.rs, hands-on learners run examples. Each tier is complete before shipping without re-writing earlier tiers.

### 2026-05-18T10:43:35.499+02:00: Leela review gate rejection
**By:** Leela
**What:** REJECTED the first Rust slice. `src/config.rs` normalizes `server_url` before both validation and endpoint construction; `ClientConfig::validate()` currently accepts whitespace-wrapped URLs that `events_url()` turns into malformed `/api/v2/events` endpoints. Bender owns the revision.
**Why:** Configuration can report as valid and still break the shared submission path that error/log/feature depend on. Parity claims are premature until the config boundary is trustworthy.

### 2026-05-18T10:43:35.499+02:00: Normalize server URL consistently in config
**By:** Bender
**What:** `ClientConfig` now uses a shared trimmed server URL value for both `validate()` parsing and `events_url()` construction.
**Why:** Validation and endpoint generation must agree so whitespace-wrapped URLs cannot pass validation and then produce malformed event endpoints.

### 2026-05-18T10:43:35.499+02:00: Leela review config fix — APPROVED
**By:** Leela
**What:** APPROVED the revised `src/config.rs` and `tests/regression_submission_path.rs`. The fix normalizes `server_url` once and uses that normalized value for both `ClientConfig::validate()` and `events_url()`, and the new regression exercises the whitespace-wrapped URL path through `submit_batch`.
**Why:** This closes the exact defect that blocked the first slice: config validation and endpoint construction now agree on the same base URL. With the regression passing alongside the full cargo test suite, the first vertical slice can proceed without blocking caveats.

### 2026-05-18T10:43:35.499+02:00: Bender SDK first-slice facade
**By:** Bender
**What:** The Rust public surface uses `Client::{error,log,feature}` with typed builders and one shared async send path (`Client::submit_batch`) returning `SubmissionResult` from transport.
**Why:** This keeps APIs discoverable and idiomatic while preserving a single observable submission pipeline for MVP testing and transport integration.

### 2026-05-18T10:43:35.499+02:00: Transport MVP contract
**By:** Farnsworth
**What:** The first Rust integration slice standardizes on a fakeable `Transport` trait that accepts a fully materialized `SubmissionRequest` (`endpoint`, `authorization`, `payload`) and returns a classified `SubmissionResult`. Wire contract locked: POST JSON event arrays to `{server}/api/v2/events` with `Authorization: Bearer {api_key}`; event types `error`, `log`, and `usage` flow through the shared `wire::event::Event` model.
**Why:** This keeps API/wire behavior observable for test transport-fake regression harness while keeping HTTP implementation details contained in `HttpTransport`.

### 2026-05-18T10:43:35.499+02:00: Land first telemetry test slice
**By:** Amy
**What:** Land the first telemetry test slice as black-box acceptance tests around the direct submission path, with one shared capturing transport harness plus regression checks for request shaping and config no-send gates.
**Why:** The current MVP ships direct async submission without a queue. Proving endpoint/auth/payload behavior now blocks false parity claims, while queue and retry coverage should be added only when that behavior exists.

### 2026-05-18T10:43:35.499+02:00: README & Examples — First-Tier Documentation Shipped
**By:** Fry
**What:** Delivered initial user-facing documentation: README.md with Quick Start, Configuration, and Examples sections; four runnable examples in `examples/`: error_basic.rs, log_structured.rs, feature_track.rs, config_custom.rs; added tokio as dev-dependency. All examples verified to compile clean.
**Why:** First example is part of the product. New users land on README; developers use docs.rs; hands-on learners run examples. README must be self-contained. This unblocks onboarding for the first slice (error/log/feature) even though crate-level docs and more examples can follow.
**Documentation Contract:** README leads with "here's what works now" + feature matrix; Configuration shown with builder pattern; Examples live in examples/ and require compilation (keeps README maintainable); No lib.rs doc comments on public types yet (deferred to next tier).

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
