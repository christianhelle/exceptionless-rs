# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-20T14:53:27.948+02:00: The release automation can stay in one `release.yml` without losing safety if `workflow_dispatch` switches between `prepare-release` and `publish-existing-tag`, both paths fail off the default branch, release prep keeps the GitHub prerelease step, and publish serializes by `release_tag` while scoping `CARGO_REGISTRY_TOKEN` to the final `cargo publish`.
- 2026-05-20T10:28:01.000+02:00: The first dependency-minimization batch confirmed the safe starting point from the dependency-floor audit: land internal-only cuts first, then treat any `reqwest` reduction beyond unused features as a packaging-boundary change around built-in HTTP rather than a pure cleanup.
- 2026-05-20T09:59:21.307+02:00: Dependency-floor work in this crate should start at the transport seam: `reqwest` is only needed for `HttpTransport` in `src/transport/http.rs`, while `src/config.rs`, `src/transport/mod.rs`, `src/wire/event.rs`, and `src/wire/error.rs` together prove the request can stay observable as `ClientConfig -> SubmissionRequest -> JSON payload`.
- 2026-05-20T09:59:21.307+02:00: The current public surface leaks dependency choices to consumers: `HttpTransport::new(reqwest::Client)` exposes `reqwest`, `pub mod wire` exposes `chrono::DateTime<Utc>` plus `serde_json::{Map, Value}`, and custom transport implementers inherit `async-trait` semantics from `transport::Transport`; cutting those crates cleanly needs a deliberate API boundary change or feature gate.
- 2026-05-20T01:03:55.309+02:00: When release/publish workflow files are reviewer-locked, optional hardening can still ship safely as independent GitHub Actions work; a standalone dependency-audit workflow adds value without reopening the approved publish artifact.
- 2026-05-18T10:43:35.499+02:00: Audit the .NET client early to capture transport, buffering, and payload-shaping behavior before implementation spreads.
- 2026-05-18T10:43:35.499+02:00: First transport slice uses `ClientConfig -> SubmissionRequest -> Transport` so tests can fake transport while asserting exact endpoint/auth/payload wire fields.
- 2026-05-18T10:43:35.499+02:00: Wire payload lives in `src/wire/event.rs` and `src/wire/error.rs`; `Event::log` trims/guards `@level`, `Event::feature_usage` maps feature name to `source`, and errors serialize under `@error`.
- 2026-05-18T10:43:35.499+02:00: HTTP submission is in `src/transport/http.rs` and posts JSON arrays to `/api/v2/events` with Bearer auth; response handling classifies success, retry, split-and-retry (413), and discard in `src/transport/response.rs`.
- 2026-05-20T00:16:20.423+02:00: Release/versioning scaffolding lives in `.github/workflows/release.yml`; manual `workflow_dispatch` resolves `base_version` from input or `RELEASE_BASE_VERSION`, appends `GITHUB_RUN_NUMBER`, rewrites `Cargo.toml` in-runner only, packages the crate, uploads `.crate` plus `.sha256`, and creates a GitHub prerelease with generated notes. User-facing release usage is documented in `README.md`.
- 2026-05-20T00:16:20.423+02:00: Manual crates.io publishing lives in `.github/workflows/publish.yml`; it checks out a `release_tag` from `Release Scaffolding`, resolves `base_version` from workflow input or `RELEASE_BASE_VERSION`, derives or accepts the numeric suffix, refreshes `Cargo.lock`, dry-runs publish without the registry token, and uses `CARGO_REGISTRY_TOKEN` only for `cargo publish --no-verify`. `README.md` documents the required `release` environment branch restriction and publish inputs.
- 2026-05-20T00:16:20.423+02:00: Lightweight release hardening can stay inside the existing path by pinning third-party GitHub Actions to commit SHAs, requiring `Release Scaffolding` to run from the default branch, and serializing manual publish runs by `release_tag` so duplicate operator clicks cannot race the same crates.io version.
- 2026-05-20T01:03:55.309+02:00: When publish/release workflow files are reviewer-locked, the safest independent hardening is a separate `.github/workflows/dependency-audit.yml` job that runs `cargo audit` on `push`, `pull_request`, `workflow_dispatch`, and a weekly schedule without touching release/publish guidance.
- 2026-05-20T01:03:55.309+02:00: Highest-value optional hardening for the current release path lives in GitHub Actions, not the crate: pin third-party actions by commit SHA in `.github/workflows/ci.yml`, `.github/workflows/release.yml`, and `.github/workflows/publish.yml`; gate manual release/publish runs to the repository default branch; and serialize publish runs by `release_tag`.

## 2026-05-18T10:43:35Z
**Scribe Team Update:**
Delivered transport MVP contract with fakeable Transport trait, SubmissionRequest model, and wire contract locked for POST /api/v2/events with Bearer auth.

## 2026-05-19T23:03:55.309Z
**Scribe Team Update:**
Completed optional hardening independently under lockout with commit `e1bc409` (`ci: add dependency audit`). The release/publish workflow files stayed untouched, and owner follow-up remains to configure `CARGO_REGISTRY_TOKEN` on the `release` environment and restrict that environment to the default branch.

- 2026-05-20T14:53:27.948+02:00: Release automation can safely live in one `.github/workflows/release.yml` when `workflow_dispatch` switches between prepare and publish actions, both paths fail outside the default branch, `release_tag` remains the publish source of truth, and `CARGO_REGISTRY_TOKEN` stays scoped to the final publish step.

## 2026-05-20T12:53:27.948Z
**Scribe Team Update:**
Merged the manual release preparation and publish flows into `.github/workflows/release.yml` in commit `2bdfb5d` (`fix(ci): merge release workflow`). The slice removed `.github/workflows/publish.yml` while preserving default-branch guards, `release_tag` publish identity, publish validation guardrails, and final-step token scoping.
## Team Coordination

- 2026-05-20T10:28:01.000+02:00: Farnsworth's dependency-floor starting point was merged into project decisions and used to keep the first slice limited to the no-risk `thiserror` removal plus the unused `reqwest` `json` feature trim.
