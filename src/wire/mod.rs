//! Advanced wire payload types used by the Exceptionless transport layer.
//!
//! Most users should stay on the builder APIs exposed at the crate root. This
//! module is for advanced scenarios that need direct control over the serialized
//! event shape or custom transport integration.
//!
//! Prefer [`crate::event::Event`] when you want a public event value that still
//! follows the ergonomic builder flow. Reach for this module only when you need
//! the exact serialized payload contract.

/// Error payload models used inside wire-format events.
pub mod error;
/// Event payload models serialized for `/api/v2/events`.
pub mod event;
