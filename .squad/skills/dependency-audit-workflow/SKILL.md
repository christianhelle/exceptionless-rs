---
name: "dependency-audit-workflow"
description: "Add independent cargo-audit hardening without touching release or publish workflow logic"
domain: "security, release, github-actions"
confidence: "high"
source: "earned"
tools:
  - name: "cargo"
    description: "Installs and runs cargo-audit against Cargo.lock"
    when: "Verifying the workflow locally or in CI"
---

## Context
Use this pattern when release or publish workflows are locked down by review feedback, but the repo still needs a lightweight hardening slice. The goal is to add supply-chain signal without changing the existing release path.

## Patterns
- Create a standalone GitHub Actions workflow such as `.github/workflows/dependency-audit.yml`.
- Trigger it on `workflow_dispatch`, a weekly `schedule`, and dependency-related `push`/`pull_request` path changes.
- Keep permissions at `contents: read`.
- Add workflow `concurrency` so duplicate audits cancel cleanly.
- Pin shared setup actions by commit SHA inside the new workflow.
- Install `cargo-audit` with `cargo install cargo-audit --locked`.
- Run `cargo audit` against the checked-in `Cargo.lock`.

## Examples
- `C:\projects\christianhelle\exceptionless-rs\.github\workflows\dependency-audit.yml`

## Anti-Patterns
- Touching `publish.yml`, `release.yml`, or README release guidance after reviewers have locked that surface.
- Hiding the audit behind manual-only execution so dependency drift is never rechecked.
- Giving the audit workflow broader permissions than read-only repository access.
