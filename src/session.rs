// SPDX-License-Identifier: Apache-2.0
//! Verify a human session through an established identity integration.
//!
//! Session expiry, role and tenant checks belong on the service boundary. Browser validation cannot supply authority.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: verify a human session through an established identity integration.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait SessionVerifier {
    /// Input whose concrete shape and validation rules are still to be specified.
    type SessionEvidence;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type ViewerContext;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Verify a human session through an established identity integration.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn verify(&self, input: &Self::SessionEvidence) -> Result<Self::ViewerContext, Self::Error>;
}
