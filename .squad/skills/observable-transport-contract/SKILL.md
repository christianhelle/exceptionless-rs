---
name: "observable-transport-contract"
description: "Keep telemetry transport fakeable by separating request construction from send execution"
domain: "integration"
confidence: "high"
source: "observed"
---

## Context
Use this when telemetry events share one submission endpoint and tests must assert wire details without real HTTP calls.

## Patterns
- Build a concrete submission request object that contains endpoint URL, auth header value, and serialized payload.
- Keep request building in a transport-agnostic module so tests can validate serialization and headers without network clients.
- Let concrete HTTP transport only send requests and map response status into explicit actions (`success`, `retry`, `split-and-retry`, `discard`).
- Keep event models (`error`, `log`, `usage`) in one wire module so all event paths serialize through one contract.
- Normalize configurable base URLs before both validation and request construction so the exact value proven valid is the value sent on the wire.

## Examples
- `src/transport/mod.rs`: `SubmissionRequest::from_events` + `Transport` trait.
- `src/transport/http.rs`: `HttpTransport` sends `POST /api/v2/events` with Bearer auth.
- `src/transport/response.rs`: status code classification for queue behavior.

## Anti-Patterns
- Hiding request shape inside HTTP client internals so test fakes cannot inspect it.
- Duplicating event serialization per feature path instead of converging on one event model.
- Returning only bool success/failure from transport and losing actionable retry semantics.
- Validating a trimmed URL but constructing requests from the untrimmed original, which creates configs that pass checks and still fail at submission time.
