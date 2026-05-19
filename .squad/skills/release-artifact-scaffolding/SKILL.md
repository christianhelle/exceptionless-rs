---
name: "release-artifact-scaffolding"
description: "Scaffold a manual prerelease flow that versions in CI, packages the crate, and publishes release artifacts without pushing to the registry"
domain: "release, versioning, github-actions"
confidence: "high"
source: "earned"
tools:
  - name: "cargo"
    description: "Runs tests and packages the crate"
    when: "Validating the release artifact before any publish step exists"
  - name: "gh"
    description: "Creates GitHub prereleases and uploads packaged assets"
    when: "Publishing release notes and artifacts without crates.io automation"
---

## Context

Use this pattern when a Rust crate needs an observable release path before the final registry publish workflow exists. The goal is to prove version resolution, artifact generation, and GitHub release behavior without mutating committed crate metadata or shipping to crates.io.

## Patterns

- Trigger the flow manually with `workflow_dispatch`.
- Resolve the base version from workflow input first, then environment or repository `RELEASE_BASE_VERSION`, then a local default.
- Suffix the resolved base version with `GITHUB_RUN_NUMBER` to guarantee a unique prerelease version for each run.
- Rewrite `Cargo.toml` only inside the runner so the repository stays on the canonical checked-in version.
- Validate with `cargo test` and `cargo package --allow-dirty`.
- Capture the packaged `.crate` and a `.sha256` checksum as workflow artifacts.
- Create a GitHub prerelease with generated notes and upload the packaged assets there.
- Keep crates.io publish out of this scaffold so artifact inspection and registry publication stay separate concerns.

## Examples

- `C:\projects\christianhelle\exceptionless-rs\.github\workflows\release.yml`
- `C:\projects\christianhelle\exceptionless-rs\README.md`

## Anti-Patterns

- Committing CI-generated prerelease versions back into `Cargo.toml`.
- Bundling crates.io publish into the first release-artifact slice.
- Producing a release without attaching the exact packaged crate that was validated.
