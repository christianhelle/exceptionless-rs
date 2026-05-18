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

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
