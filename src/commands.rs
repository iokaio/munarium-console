// SPDX-License-Identifier: Apache-2.0
//! Submit an exact operator intent through the owning service API.
//!
//! Council handles approval, Warden suspension, and Registry candidate intake. This proposed port supplies no direct database mutation or credential bypass.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: submit an exact operator intent through the owning service API.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait GovernedCommandClient {
    /// Input whose concrete shape and validation rules are still to be specified.
    type BoundCommand;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type ServiceReceipt;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Submit an exact operator intent through the owning service API.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn submit(&mut self, input: &Self::BoundCommand) -> Result<Self::ServiceReceipt, Self::Error>;
}
