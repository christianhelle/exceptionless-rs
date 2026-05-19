# Project Context

- **Owner:** Christian Helle
- **Project:** Rust client for Exceptionless with Exceptionless.Net as the reference implementation
- **Stack:** Rust, Cargo, Exceptionless API integration, GitHub Actions
- **Created:** 2026-05-18T10:43:35.499+02:00

## Learnings

- 2026-05-18T10:43:35.499+02:00: Audit the .NET client early to capture transport, buffering, and payload-shaping behavior before implementation spreads.
- 2026-05-18T10:43:35.499+02:00: First transport slice uses `ClientConfig -> SubmissionRequest -> Transport` so tests can fake transport while asserting exact endpoint/auth/payload wire fields.
- 2026-05-18T10:43:35.499+02:00: Wire payload lives in `src/wire/event.rs` and `src/wire/error.rs`; `Event::log` trims/guards `@level`, `Event::feature_usage` maps feature name to `source`, and errors serialize under `@error`.
- 2026-05-18T10:43:35.499+02:00: HTTP submission is in `src/transport/http.rs` and posts JSON arrays to `/api/v2/events` with Bearer auth; response handling classifies success, retry, split-and-retry (413), and discard in `src/transport/response.rs`.
- 2026-05-20T00:16:20.423+02:00: Release/versioning scaffolding lives in `.github/workflows/release.yml`; manual `workflow_dispatch` resolves `base_version` from input or `RELEASE_BASE_VERSION`, appends `GITHUB_RUN_NUMBER`, rewrites `Cargo.toml` in-runner only, packages the crate, uploads `.crate` plus `.sha256`, and creates a GitHub prerelease with generated notes. User-facing release usage is documented in `README.md`.
- 2026-05-20T00:16:20.423+02:00: Manual crates.io publishing lives in `.github/workflows/publish.yml`; it checks out a `release_tag` from `Release Scaffolding`, resolves `base_version` from workflow input or `RELEASE_BASE_VERSION`, derives or accepts the numeric suffix, refreshes `Cargo.lock`, dry-runs publish without the registry token, and uses `CARGO_REGISTRY_TOKEN` only for `cargo publish --no-verify`. `README.md` documents the required `release` environment branch restriction and publish inputs.
- 2026-05-20T00:16:20.423+02:00: Lightweight release hardening can stay inside the existing path by pinning third-party GitHub Actions to commit SHAs, requiring `Release Scaffolding` to run from the default branch, and serializing manual publish runs by `release_tag` so duplicate operator clicks cannot race the same crates.io version.
- 2026-05-20T01:03:55.309+02:00: Highest-value optional hardening for the current release path lives in GitHub Actions, not the crate: pin third-party actions by commit SHA in `.github/workflows/ci.yml`, `.github/workflows/release.yml`, and `.github/workflows/publish.yml`; gate manual release/publish runs to the repository default branch; and serialize publish runs by `release_tag`.

## 2026-05-18T10:43:35Z
**Scribe Team Update:**
Delivered transport MVP contract with fakeable Transport trait, SubmissionRequest model, and wire contract locked for POST /api/v2/events with Bearer auth.
