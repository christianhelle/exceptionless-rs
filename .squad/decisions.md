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

### 2026-05-18T11:26:29.220+02:00: User directive
**By:** Christian Helle (via Copilot)
**What:** Commit the current changes in small logical groups without a co-author.
**Why:** User request — captured for team memory

### 2026-05-18T11:44:38.999+02:00: Rename public client type
**By:** Bender
**What:** The public Rust facade is now ExceptionlessClient, and the crate no longer re-exports Client as the primary SDK type.
**Why:** The original Exceptionless .NET client uses ExceptionlessClient, and mirroring that name keeps cross-SDK terminology aligned without changing Rust-side behavior or the shared submission path.
**Files:** src/client.rs, src/lib.rs, src/builder.rs, src/error.rs, tests/*.rs, examples/*.rs, README.md

### 2026-05-18T11:44:38.999+02:00: Rename client tests
**By:** Amy
**What:** Treat the `Client` → `ExceptionlessClient` rename as a public API compatibility gate: acceptance tests, regression tests, and example compilation must all bind to `ExceptionlessClient`, with a regression covering `ExceptionlessClient::with_api_key`.
**Why:** Happy-path runtime tests alone would miss stale public imports in examples and convenience-constructor usage. The rename is not proven until the public surface compiles everywhere users touch it.

### 2026-05-18T11:44:38.999+02:00: Client → ExceptionlessClient Rename Documentation Update
**By:** Fry (Docs/DevRel)
**What:** Renamed `Client` struct to `ExceptionlessClient` throughout user-facing documentation and codebase to align with Exceptionless.NET naming conventions.
**Why:** Developers migrating from .NET will recognize the `ExceptionlessClient` type immediately, reducing onboarding friction and improving mental model transfer. Verification: ✅ All examples compile; ✅ Tests pass; ✅ Documentation consistent.

### 2026-05-18T11:52:41.203+02:00: Commit discipline default
**By:** Leela
**What:** Default repo policy is to stage and commit each completed logical slice immediately, use concise Conventional Commit subjects, omit co-author trailers unless explicitly requested, and keep generated/runtime artifacts out of commits.
**Why:** Small, verified commits preserve a readable progress history, keep boundaries sharp, and reduce the risk of mixing unrelated work or committing local runtime output.

### 2026-05-18T14:28:06.933+02:00: Real backtrace capture in map_error
**By:** Bender
**What:** `map_error()` in `src/error.rs` now uses the `backtrace` crate to capture a real call-frame stack trace at the point the error is reported to the SDK, rather than formatting the error's Debug representation. `capture_backtrace()` resolves symbols eagerly and applies a two-level filter: (1) Discard frames with no `::`; (2) Discard frames from `exceptionless::`, `backtrace::`, `std::*`, `tokio::*`, `core::*`. Inner errors receive no backtrace. `backtrace = "0.3"` is now a runtime dependency. Backtraces are always captured unconditionally; frame detail is best in debug builds with symbols.
**Why:** The previous debug-dump approach stored the error's Debug string as a single frame with zero location data. The new approach surfaces the user call site so developers can see where the error was reported. New regression file: `tests/regression_error_stack_trace.rs`.

### 2026-05-18T14:28:06.933+02:00: Stack trace test coverage gaps — update required
**By:** Amy
**What:** Existing assertion in `tests/acceptance_errors.rs` relies on the broken behavior (debug dump as method name); after Bender's fix, frame[0].method will be a real function name, not the error type. Required updates: (1) Replace brittle `contains("OuterError")` with assertions proving real frames (non-empty, .rs files, line numbers); (2) Add `stack_trace_from_stdlib_error_has_real_frames` test mirroring the example exactly, asserting multiple frames, .rs filenames, line numbers, and no debug-dump format; (3) Add `stack_trace_frames_include_error_site` test verifying at least one frame references user code. Implementation notes: Prefer `Backtrace::force_capture()` for deterministic tests; use `backtrace` crate (not std) for frame access; filter out `std::`, `core::`, `tokio::`, `futures::`, `alloc::` noise; debug builds have best symbol resolution.
**Why:** After Bender's fix, existing assertions will fail; coverage gaps mask the usefulness of the backtrace. Real frames must include user code location, not just SDK internals.

### 2026-05-18T14:28:06.933+02:00: Copilot directive — session workflow
**By:** Christian Helle (via Copilot)
**What:** Scribe orchestrates end-of-session archival: stage decided decisions from inbox, write orchestration logs per agent, write session log, update agent history, and commit the squad/ changes in one logical group.
**Why:** Preserves team continuity and keeps audit trail of work coordination.

### 2026-05-18T15:54:28.101+02:00: Repository rename to exceptionless-rs
**By:** Leela
**What:** Renamed GitHub repository from `exceptionless.rust` to `exceptionless-rs` and updated all project metadata and configuration files.
**Why:** The hyphenated name `exceptionless-rs` is the idiomatic Rust crate naming convention, signals intent clearly on GitHub, and aligns with how users will discover and reference the SDK.
**Files updated:**
- `Cargo.toml` — repository URL
- `README.md` — GitHub issues link
- `.squad/team.md` — project name (2 references)
- `.squad/agents/scribe/history.md` — project context
- `.squad/agents/scribe/charter.md` — project context
- `.squad/agents/ralph/history.md` — project context
- `.squad/agents/ralph/charter.md` — project context
**GitHub outcome:** ✅ Repository successfully renamed via `gh repo rename`
**Remote configuration:** ✅ Git remote updated from `git@github.com:christianhelle/exceptionless.rust.git` to `https://github.com/christianhelle/exceptionless-rs.git`
**Next:** Scribe will merge this decision to decisions.md on session close.

### 2026-05-19T23:57:10.867+02:00: Validation surface and release gates
**By:** Amy
**What:** Current release-readiness baseline is 13 passing integration tests, 4 compiling examples, and a successful `cargo doc --no-deps`; the must-pass gate for release work is `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo doc --no-deps`, and `cargo package --allow-dirty`.
**Why:** The crate needs an explicit, repeatable validation bar before claiming release readiness or wiring CI around it.
**Known blockers:** invalid `edition = "2024"`, missing package metadata, no CI workflow, no rustdoc coverage, no unit tests, and no declared MSRV.

### 2026-05-19T23:57:10.867+02:00: Crates.io and docs readiness audit
**By:** Fry
**What:** The first publish is blocked by invalid manifest metadata (`edition = "2024"`, missing `description`, missing `categories`, plus other release-facing metadata), and the next documentation gaps are missing crate-level rustdoc, `CHANGELOG.md`, `CONTRIBUTING.md`, and `SECURITY.md`.
**Why:** crates.io and docs.rs need a minimally complete manifest and documentation surface so the first public release is both publishable and usable.

### 2026-05-19T23:57:10.867+02:00: Release workflow planning boundaries
**By:** Leela
**What:** Broad release automation should be phased as quality gates, release management, and observability, but implementation beyond the immediate critical path should wait for owner clarification on MSRV, token ownership, semver policy, platform matrix, docs strategy, changelog curation, approval gates, and dependency constraints.
**Why:** Workflow scope and policy choices affect CI cost, publish safety, and release semantics, so the team should not over-automate unresolved release decisions.

### 2026-05-20T00:16:20.423+02:00: CI validation workflow baseline
**By:** Bender
**What:** Add `.github/workflows/ci.yml` as a single Ubuntu stable Rust job running `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo doc --no-deps`, then `cargo package --allow-dirty`.
**Why:** CI should mirror the proven local release-readiness path instead of inventing a wider matrix before the baseline is stable.

### 2026-05-20T00:16:20.423+02:00: User directive
**By:** Christian Helle (via Copilot)
**What:** Have all agents use Claude Opus 4.7 for the rest of the session, but only for this session.
**Why:** User request — captured for team memory

### 2026-05-20T00:16:20.423+02:00: Approve release-gate slice d79ae32
**By:** Amy
**What:** APPROVED Bender's `green-local-release-gates` slice at commit `d79ae32` after verifying the diff stayed behavior-preserving and the local release-proof commands passed: `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`, and `cargo package --allow-dirty`.
**Why:** The repo can carry the green local gate baseline forward into CI only if the release-fix slice is proven to preserve observable error/log/feature and submission-path behavior.
**Note:** `cargo package --allow-dirty` still emits a non-blocking warning that `Cargo.toml` lacks a `description`.

### 2026-05-20T00:16:20.423+02:00: MVP crate docs contract
**By:** Fry
**What:** README.md and crate-level rustdoc in `src/lib.rs` must describe only the verified MVP surface: `ExceptionlessClient`, error/log/feature builders, direct async submission, custom server configuration, optional custom transports, the `tokio` runtime requirement used by examples, and `send().await` returning `SubmissionResult`.
**Why:** The SDK does not yet implement queue workers, offline delivery, settings sync, sessions, plugins, or logging facade integrations, so docs must not imply unsupported behavior or treat disabled mode as a silent no-op.
**Packaging note:** Published crate metadata should point users to README/docs.rs and exclude repo-only coordination files from the package.

### 2026-05-20T00:16:20.423+02:00: Release artifact and prerelease scaffolding
**By:** Farnsworth
**What:** Add a manual `.github/workflows/release.yml` flow that resolves `base_version` from workflow input or `RELEASE_BASE_VERSION`, appends `github.run_number`, rewrites `Cargo.toml` only inside the runner, runs `cargo test` plus `cargo package --allow-dirty`, uploads the `.crate` and `.sha256`, and creates a GitHub prerelease with generated notes.
**Why:** The team needs an inspectable release artifact path before crates.io publishing exists, without mutating the checked-in package version.

### 2026-05-20T00:16:20.423+02:00: Publish workflow guardrails
**By:** Farnsworth
**What:** The manual crates.io publish path should not publish directly from an arbitrary branch snapshot. It must publish the exact `release_tag` produced by `Release Scaffolding`, reuse the shared base-version-plus-suffix scheme, refresh `Cargo.lock` after the in-runner version rewrite, keep `CARGO_REGISTRY_TOKEN` scoped to the actual publish step, and rely on the `release` environment being restricted to the default branch in GitHub settings.
**Why:** Publish credentials and release versions both drift silently if the workflow is free to run from branch-specific definitions or from code that no longer matches the scaffolded release tag. Separating dry-run verification from the token-bearing `cargo publish --no-verify` step keeps code execution out of the secret-bearing phase while preserving the existing release scaffold.

### 2026-05-20T01:03:55.309+02:00: Approve publish tag fix
**By:** Amy
**What:** APPROVED commit `c5359e0` (`fix(ci): trust scaffolded release tag`). The publish workflow now treats `release_tag` as the sole publish-version source, keeps `CARGO_REGISTRY_TOKEN` scoped to the real publish step, preserves the stronger `cargo test --all-targets` gate, and leaves the existing release scaffold behavior intact.
**Why:** The rejected publish artifact was blocked on version drift risk and secret exposure. This revision closes those gaps without weakening the proven release path.

### 2026-05-20T01:03:55.309+02:00: Publish tag is the source of truth
**By:** Bender
**What:** `.github\workflows\publish.yml` now accepts only `release_tag`, derives `publish_version` directly from that scaffolded tag, keeps `CARGO_REGISTRY_TOKEN` scoped to the final publish step, and preserves the proven `cargo test --all-targets` validation gate. `README.md` release docs were updated to match.
**Why:** Re-deriving publish coordinates from mutable workflow settings can drift from the prerelease artifact that was already reviewed. The publish job should trust the scaffolded tag, minimize secret exposure, and keep the same validation bar already proven locally.

### 2026-05-20T01:03:55.309+02:00: Independent dependency audit hardening
**By:** Farnsworth
**What:** Keep optional hardening independent from the locked release/publish path by adding a separate dependency-audit workflow instead of editing `publish.yml`, `release.yml`, or README release guidance.
**Why:** Reviewer lockout on the publish artifact means release-path edits are no longer safe to carry in this slice. A standalone `cargo audit` workflow still adds value by catching RustSec advisories on dependency changes and on a weekly cadence.

## Governance
- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
