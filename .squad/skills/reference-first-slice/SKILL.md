---
name: "reference-first-slice"
description: "How to carve the first Rust SDK slice from a larger reference client without freezing the wrong API"
domain: "api-design"
confidence: "high"
source: "observed"
---

## Context
Use this when a greenfield Rust client is being ported from a mature SDK with much more behavior than can fit in the first deliverable.

## Patterns
- Find the narrowest end-to-end path shared by the MVP features before mirroring the whole reference architecture.
- Freeze the first public Rust boundary at `Client -> Builder -> Event`, then keep transport and wire serialization private behind internal modules.
- Port the wire contract early, but defer durable queues, plugin systems, settings polling, and platform hooks until after the first successful submission path exists.
- Split public API ownership from transport ownership so façade changes and wire changes can evolve independently.

## Examples
- In Exceptionless.Net, errors, logs, and feature usage all converge on `ExceptionlessClient`, `EventBuilder`, and `DefaultSubmissionClient.PostEventsAsync`.
- A good first Rust slice is `Client::error/log/feature` → shared builder → internal wire event array → `POST /api/v2/events`.

## Anti-Patterns
- Promising broad parity before one real submission path passes end to end.
- Rebuilding queue persistence or plugin orchestration before the transport boundary is proven.
- Letting internal wire DTOs become the public Rust API.
