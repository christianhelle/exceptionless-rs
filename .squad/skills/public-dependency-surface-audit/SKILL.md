---
name: "public-dependency-surface-audit"
description: "Classify Rust crate dependencies by public leak, behavioral lock-in, and safest removal order"
domain: "rust, cargo, api-design"
confidence: "high"
source: "earned"
tools:
  - name: "cargo tree"
    description: "Shows which direct dependencies drag the largest transitive graph"
    when: "Sizing impact after mapping public touchpoints"
---

## Context
Use this when a Rust crate needs fewer dependencies but the public SDK surface may already expose transport types, JSON value types, or async extension traits.

## Patterns
- Audit only direct runtime dependencies first; transitive noise is downstream of those choices.
- For each dependency, map four things: internal implementation use, public type-signature leak, docs/example leak, and shipped behavior it preserves.
- Classify dependencies into three buckets: pure internal sugar, public-extension-point leak, and behavior-locked runtime dependency.
- Recommend the first implementation slice from the pure internal bucket only; anything else usually needs a product/API decision before code moves.
- Heavy HTTP client dependencies should trigger a deliberate product choice: batteries-included default transport vs. core crate plus optional transport feature/subcrate.

## Questions To Force Early
- Is the built-in HTTP transport part of the default product promise?
- Are custom transports a first-class extension point or an advanced escape hatch?
- Is structured stack-trace capture a required MVP behavior?
- Can the server assign event timestamps, or is client-side timestamp generation part of the contract?

## Anti-Patterns
- Starting with the heaviest dependency before proving whether it is actually public contract.
- Treating docs/examples as non-public when they teach consumers the leaked type shape.
- Ignoring trait-implementation ergonomics for downstream users implementing custom transports.
