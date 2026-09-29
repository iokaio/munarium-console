// SPDX-License-Identifier: Apache-2.0
//! Load a tenant-scoped projection for an operator.
//!
//! Preserve source references, freshness and distinct proposed/authorized/dispatched/completed/unresolved states; no UI-only success inference.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: load a tenant-scoped projection for an operator.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait ViewReader {
    /// Input whose concrete shape and validation rules are still to be specified.
    type ViewQuery;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type View;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Load a tenant-scoped projection for an operator.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn load(&self, input: &Self::ViewQuery) -> Result<Self::View, Self::Error>;
}
