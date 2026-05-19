---
name: "manual-crates-publish"
description: "Publish a Rust crate from GitHub Actions without drifting from a previously scaffolded release version or overexposing the crates.io token"
domain: "release, publishing, github-actions"
confidence: "high"
source: "earned"
tools:
  - name: "cargo"
    description: "Refreshes Cargo.lock, validates the crate, and publishes to crates.io"
    when: "Publishing a CI-rewritten crate version"
  - name: "gh"
    description: "Supplies the release tag created by the prerelease scaffold"
    when: "Connecting a publish run back to the reviewed GitHub release artifact"
---

## Context
Use this pattern after a separate release-scaffolding flow already proved the packaged artifact and created a GitHub prerelease tag. The goal is to publish that exact logical release to crates.io without committing a generated version back into `Cargo.toml` or leaking the crates.io token across unrelated workflow steps.

## Patterns
- Trigger the publish flow manually with `workflow_dispatch`.
- Require the `release_tag` created by the release scaffold and check out that exact ref before publishing.
- Guard both release and publish workflows so they only run from the repository default branch when secrets or tag creation are involved.
- Treat `release_tag` as the canonical release identity once the scaffold has run; any recomputed `base_version` or suffix must validate against that tag, not replace it.
- Resolve `base_version` from workflow input first, then `RELEASE_BASE_VERSION`, then a local default only when you are creating a fresh scaffolded version. A publish workflow consuming an existing tag must not strand valid older tags because mutable base-version settings changed later.
- Derive the numeric suffix from `release_tag` by default; allow an explicit override only when you must restate the known suffix.
- Rewrite `Cargo.toml` only inside the runner, then run `cargo generate-lockfile` so `--locked` commands stay consistent.
- Keep the validated cargo gate at least as strict as the already-proven release gate; if the repo standard is `cargo test --all-targets`, do not silently downgrade to plain `cargo test` in publish.
- Run `cargo publish --dry-run --locked --allow-dirty` before the real publish.
- Pin third-party GitHub Actions to immutable commit SHAs instead of moving tags such as `@v4`.
- Add workflow `concurrency` keyed by `release_tag` so duplicate manual publish clicks cannot race the same crates.io version.
- Scope `CARGO_REGISTRY_TOKEN` only to the actual `cargo publish` step, and use `--no-verify` there so the secret-bearing phase uploads the already-verified package instead of re-running code.
- Restrict the GitHub Actions environment that holds `CARGO_REGISTRY_TOKEN` to the default branch so branch-specific workflow edits cannot reach the secret.

## Examples
- `C:\projects\christianhelle\exceptionless-rs\.github\workflows\publish.yml`
- `C:\projects\christianhelle\exceptionless-rs\.github\workflows\release.yml`
- `C:\projects\christianhelle\exceptionless-rs\README.md`

## Anti-Patterns
- Publishing from the currently selected branch without tying the run back to a scaffolded release tag.
- Exposing `CARGO_REGISTRY_TOKEN` at the job level or during dry-run verification.
- Recomputing a publish version from mutable current workflow state when the intention is to publish a previously scaffolded release tag.
- Downgrading `cargo test --all-targets` to `cargo test` in the publish path after the stricter gate has already been adopted elsewhere.
- Rewriting `Cargo.toml` and then using `--locked` without refreshing `Cargo.lock`.
