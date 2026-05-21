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

### 2026-05-20T14:53:27.948+02:00: Workflow merge guardrails
**By:** Amy
**What:** Preserve the release-automation contract while merging the standalone publish workflow into the manual release flow: keep `ci.yml` as the automatic push/PR validation workflow with its existing concurrency guard and validation sequence, keep prerelease creation in `release.yml`, ensure the publish path continues to trust `release_tag` as the publish source of truth, and keep `CARGO_REGISTRY_TOKEN` scoped only to the final publish step.
**Why:** This slice touched CI, release scaffolding, publish automation, and operator docs at once, so the highest regression risk was contract drift between files or a cleanup that silently weakened release safety.

### 2026-05-20T14:53:27.948+02:00: Approve merged release workflow slice
**By:** Amy
**What:** APPROVED the merged `.github/workflows/release.yml` plus `README.md` update and removal of `.github/workflows/publish.yml`. The single workflow now carries the prepare and publish manual paths, keeps prerelease creation in the prepare path, restricts both paths to the default branch, treats `release_tag` as the publish version source of truth, preserves `cargo generate-lockfile`, `cargo test --all-targets`, and `cargo publish --dry-run --locked --allow-dirty`, and still scopes `CARGO_REGISTRY_TOKEN` only to the final publish step.
**Why:** The merge preserves the previously approved release and publish contracts while removing duplicated workflow surface and keeping operator documentation aligned with the actual release path.

### 2026-05-20T14:53:27.948+02:00: Merge manual release and publish workflows
**By:** Farnsworth
**What:** Merged the manual release preparation and publish flows into `.github/workflows/release.yml` behind a required `action` workflow dispatch choice, kept `release_tag` as the publish source of truth for checkout and version derivation, and added explicit default-branch guards to both manual paths.
**Why:** One workflow removes duplicated release automation while preserving prerelease creation, publish identity, and the branch restrictions that protect the `release` environment.

### 2026-05-20T14:53:27.948+02:00: README release workflow wording after merge
**By:** Fry
**What:** Updated README release guidance to describe a single `.github/workflows/release.yml` workflow with two manual paths: **Prepare release** and **Publish existing tag**. The docs now state that prepare creates the GitHub prerelease and release artifacts, both paths are restricted to the default branch, and `release_tag` remains the publish source of truth.
**Why:** Operator-facing docs must match the merged release story without sending users to a deleted standalone publish workflow, while still preserving the existing release guardrails.
### 2026-05-20T10:28:01.000+02:00: Dependency floor starting point
**By:** Farnsworth
**What:** Start dependency reduction with internal-only cuts first: remove the unused `reqwest` `json` feature, then consider replacing direct `thiserror` usage with manual error implementations if the team wants a stricter floor. Treat `reqwest`, `serde_json`, `serde`, `chrono`, `backtrace`, and `async-trait` as locked for now because each one still preserves either the default HTTP path, the JSON payload contract, public wire types, stack-frame behavior, or the public custom-transport seam.
**Why:** The crate's transport and wire seams are intentionally observable today, so cutting those dependencies without a redesign or feature gate would break downstream custom transports, change event payload types, or degrade shipped MVP behavior.

### 2026-05-20T10:28:01.000+02:00: Dependency audit first slice
**By:** Bender
**What:** The smallest safe dependency-reduction slice is to remove direct `thiserror` usage first, then make a separate product and API decision on whether the crate keeps a built-in `reqwest` transport in the default product or feature-gates HTTP transport so `reqwest` can leave the default dependency set.
**Why:** `thiserror` is internal derive sugar with no user-visible contract, while `reqwest`, `async-trait`, `serde_json`, `chrono`, and `backtrace` are still coupled either to public extension points, the documented default HTTP story, or shipped event behavior.

### 2026-05-20T10:28:01.000+02:00: Dependency reduction proof gate
**By:** Amy
**What:** Treat dependency-reduction work as safe only if the plan explicitly preserves or intentionally breaks these consumer-visible contracts: custom transport implementations through `transport::Transport`, `HttpTransport::new(reqwest::Client)` and `HttpTransport::default()`, plus builder `.data(... Into<serde_json::Value>)` calls. Proof must include doctest compilation, example compilation, and the existing payload and config regressions; if transport or response handling changes, add explicit coverage for HTTP response classification and invalid-config and error paths before claiming parity.
**Why:** The current suite proves the shared submission contract, stack-trace quality, and a few config gates, but it does not by itself prove that swapping or hiding public-facing dependencies is a harmless internal refactor.

### 2026-05-20T10:28:01.000+02:00: Dependency reduction boundary and packaging direction
**By:** Leela
**What:** Treat dependency reduction as a boundary decision first, not a crate-swap exercise. The immediate packaging recommendation is to keep one crate and move the built-in HTTP path behind a default-disabled `http` feature if the team proceeds past the no-risk cuts.
**Why:** `reqwest` drives most of the current runtime footprint, and the present default client surface hardwires that cost through `ExceptionlessClient<T = HttpTransport>`, `ExceptionlessClient::with_api_key`, `transport::http::HttpTransport`, and `ClientConfig::validate()`. A same-crate feature gate removes default HTTP dependency pollution with lower semver and documentation cost than introducing a companion crate immediately.

### 2026-05-20T10:28:01.000+02:00: First no-risk dependency-minimization slice
**By:** Bender
**What:** Remove the direct `thiserror` dependency and replace derive usage with manual `Display`, `std::error::Error`, and `From` implementations in `src\config.rs`, `src\error.rs`, and `src\transport\mod.rs`, and remove the unused `reqwest` `json` feature from `Cargo.toml`.
**Why:** This reduces the direct dependency surface without changing the public API, transport contract, or runtime behavior, and it stays inside the validated no-risk bucket while broader HTTP-boundary decisions remain open.

### 2026-05-20T10:28:01.000+02:00: Approve first dependency cleanup slice
**By:** Amy
**What:** APPROVED the slice removing direct `thiserror` usage and trimming the unused `reqwest` `json` feature after focused regression coverage locked `Display` text and `source()` chaining for `ConfigError`, `TransportError`, and `ClientError`.
**Why:** The slice preserved behavior parity, kept examples compiling, left JSON serialization and parsing on `serde_json`, and cleared the expected local validation bar without widening the transport boundary.

### 2026-05-20T10:28:01.000+02:00: Final gate for first dependency-minimization slice
**By:** Leela
**What:** APPROVED the first dependency-minimization slice exactly as shipped: direct `thiserror` removal, trimmed unused `reqwest` `json` feature, stable `ConfigError`, `TransportError`, and `ClientError` display and source behavior, and targeted regression coverage for those error contracts. The next slice must stay narrowly on the packaging boundary around built-in HTTP.
**Why:** This diff stayed inside the no-risk bucket, the local validation bar passed end to end, and there was no evidence-based reason to block the commit once the behavior-locking regressions were in place.

### 2026-05-20T10:28:01.000+02:00: Approve opt-in HTTP boundary slice
**By:** Amy
**What:** APPROVED the opt-in HTTP boundary slice after re-checking both the default lane and the `http` feature lane, including doctest coverage and dependency-tree expectations. Keep docs explicit that consumers without the `http` feature must provide their own transport.
**Why:** Gating built-in HTTP is only safe if lean-core consumers still have a documented path forward and the optional HTTP experience remains fully validated.

### 2026-05-20T10:28:01.000+02:00: Final gate for opt-in HTTP boundary slice
**By:** Leela
**What:** APPROVED Farnsworth's built-in HTTP packaging slice exactly as shipped: default-disabled `http` feature, optional `reqwest`, gated `transport::http` and `ExceptionlessClient::with_api_key()`, `url::Url` in core, and validated default plus `http` release lanes. The next slice must not yet touch `async-trait`, `serde_json`, `chrono`, or broader public-surface reshaping.
**Why:** This lands the biggest default dependency reduction at the package boundary without mixing in deeper API changes before the team maps the next reference-backed seam.


### 2026-05-20T11:59:35.339+02:00: Approve default-enabled HTTP transport slice
**By:** Amy
**What:** Approve Farnsworth's opt-out telemetry/default-enabled transport slice.
**Why:** The diff stays disciplined to packaging and docs: `Cargo.toml` now defaults `http`, while `README.md` and `src/lib.rs` consistently explain the new default path and the lean-core opt-out via `default-features = false`. Revalidation passed in the default lane, lean-core no-default-features lane, explicit `http` lane, and doctest coverage, and the no-default-features dependency tree still excludes `reqwest`/TLS transport baggage.
**Impact:**
- Default consumers keep the ergonomic `ExceptionlessClient::with_api_key(...)` path without adding features manually.
- Opt-out consumers can still build the lean core and supply a custom transport.
- No blocking follow-up remains for this slice beyond carrying the same validation matrix into final gate review.

### 2026-05-20T11:59:35.339+02:00: User directive
**By:** Christian Helle (via Copilot)
**What:** Prefer opt-out telemetry over opt-in `http`; if feasible, make built-in transport enabled by default and let consumers opt out.
**Why:** User request — captured for team memory

### 2026-05-20T11:59:35.339+02:00: HTTP default opt-out direction
**By:** Farnsworth
**What:** Flip the packaging slice to make the existing `http` feature part of `default` instead of renaming or removing it.
**Why:** This keeps the current transport gate and optional `reqwest` dependency model intact while restoring the documented built-in HTTP experience for default consumers. Opt-out consumers still get the lean core by building with `default-features = false`, which removes `reqwest` and `rustls` from the normal dependency tree without touching the transport trait or wire contract.
**Impact:**
- Default builds now include `transport::http::HttpTransport` and `ExceptionlessClient::with_api_key(...)`.
- Lean-core consumers must switch from doing nothing to explicitly using `default-features = false`.
- Existing explicit `features = ["http"]` requests remain valid but become redundant.

### 2026-05-20T11:59:35.339+02:00: Final gate for default-enabled HTTP transport slice
**By:** Leela
**What:** APPROVE Farnsworth's opt-out telemetry/default-enabled transport slice exactly as-is.
**Why:** The change stays inside the packaging boundary: `Cargo.toml` restores the default built-in HTTP path, while `README.md` and `src/lib.rs` now consistently explain that lean-core consumers must opt out with `default-features = false`. The validation bar passed across the default lane, explicit `http` lane, no-default-features lane, doctests, and example compilation, with the lean-core dependency tree still excluding `reqwest` and TLS baggage.
**Impact:**
- The crate regains the zero-config `ExceptionlessClient::with_api_key(...)` onboarding path in the default build.
- Lean-core consumers still have an explicit escape hatch that preserves the custom transport seam.
- Next boundary stays tight: do not touch `async-trait`, `serde_json`, `chrono`, or broader public API reshaping until the team maps the next .NET-backed seam for public dependency reduction.

### 2026-05-20T13:00:42.108+02:00: User directive
**By:** Christian Helle (via Copilot)
**What:** The intended feature model is an `opt-out` feature whose enabled state disables telemetry collection.
**Why:** User request — captured for team memory

### 2026-05-20T13:00:42.108+02:00: Single `opt-out` feature tradeoff
**By:** Farnsworth
**What:** To keep `ExceptionlessClient::with_api_key(...)` and `transport::http::HttpTransport` available while exposing only one consumer-facing Cargo feature, the crate now treats `reqwest` as an unconditional dependency and uses `opt-out` only to short-circuit submission to a synthetic success result.
**Why:** This preserves the transport-facing API shape, but it removes the previous lean-core packaging split: `--no-default-features` no longer drops the built-in HTTP dependency graph.

### 2026-05-20T13:00:42.108+02:00: Reject first single `opt-out` feature revision
**By:** Amy
**What:** REJECTED Farnsworth's single `opt-out` feature slice because `README.md` and `src/lib.rs` still overstated disabled-client behavior under `opt-out`, and there was no direct regression proving `submit_batch()` itself returned the same synthetic accepted success without transport calls. Revision ownership moved to Bender.
**Why:** The single-feature model is only safe to ship if docs tell the truth about the synthetic no-op success path and direct `submit_batch()` coverage proves the short-circuit wins even on the empty-batch edge.

### 2026-05-20T13:00:42.108+02:00: Approve revised single `opt-out` feature slice
**By:** Amy
**What:** APPROVED Bender's revision after `README.md` and `src/lib.rs` made the disabled-config behavior explicitly conditional on `opt-out`, and `tests/regression_submission_path.rs` added direct opt-out coverage for both builder `send()` and empty-batch `submit_batch()` synthetic 202 success.
**Why:** The revision closes the exact review blockers without widening scope, and the validation proof stayed green in normal, doctest, and `opt-out` lanes.

### 2026-05-20T13:00:42.108+02:00: Final gate for single `opt-out` feature revision
**By:** Leela
**What:** APPROVED the narrow revision slice exactly as-is: `README.md`, `src/lib.rs`, and `tests/regression_submission_path.rs` now align with the already-implemented single `opt-out` feature model, including direct `submit_batch()` no-op success coverage.
**Why:** The revision fixes the exact review findings without widening scope. Docs now truthfully state that the built-in HTTP transport and `ExceptionlessClient::with_api_key(...)` stay available in every build while `opt-out` short-circuits submission to a synthetic success, and the regression proof covers that behavior on both `send()` and `submit_batch()`.
**Impact:**
- Ready to commit exactly as-is.
- Validation rechecked with `cargo test --test regression_submission_path`, `cargo test --test regression_submission_path --features opt-out`, `cargo test --doc`, `cargo test --all-targets`, and `cargo test --all-targets --features opt-out`.
- No blocking follow-up remains for the overall single-feature `opt-out` work.

### 2026-05-20T13:37:53.920+02:00: User directive
**By:** Christian Helle (via Copilot)
**What:** Commit changes in small logical groups without a co-author, and do this automatically for this session and future sessions.
**Why:** User request — captured for team memory

## Governance
- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
