# Munarium Console implementation architecture

**Proposed design; scaffold only.** Based on section 15 of the
[platform plan, revision 4](https://github.com/iokaio/munarium-platform/blob/main/docs/platform-plan.md), with lifecycle and failure rules in
sections 17–19 and 22. See the hub's
[scaffold decision proposal](https://github.com/iokaio/munarium-platform/blob/main/docs/decisions/0001-scaffold-boundaries.md)
for the distinction between local interfaces and normative contracts.

## Responsibility and current boundary

Read-only operator views first, followed by interactions through governed service APIs. Console belongs to the **assurance plane**.
The crate declares interfaces only: no concrete implementations, serialization,
network listeners, persistence, service authentication or target operations exist.

The associated input, output and error types are intentionally unspecified.
These are proposed in-process seams for implementation work, not a released Rust API
or a second definition of the shared wire contract. A trait signature does not enforce
the trust assumptions below. Async runtime, transport and storage choices remain open.

## Module map

| Source | Proposed interface | Responsibility |
|---|---|---|
| [views](../src/views.rs) | `ViewReader` | Preserve source references, freshness and distinct proposed/authorized/dispatched/completed/unresolved states; no UI-only success inference. |
| [session](../src/session.rs) | `SessionVerifier` | Session expiry, role and tenant checks belong on the service boundary. Browser validation cannot supply authority. |
| [commands](../src/commands.rs) | `GovernedCommandClient` | Council handles approval, Warden suspension, and Registry candidate intake. This proposed port supplies no direct database mutation or credential bypass. |

## Planned flow and state ownership

Verify human session → query tenant-scoped governed read APIs → display evidence, freshness and unresolved work. Later, bind an operator command to the exact reviewed request → call Council/Warden/intake → display the authoritative service result.

Own session handling and rebuildable presentation caches only. No direct write access to authority tables, target credentials or authoritative approval state. The Rust scaffold covers application/service interfaces; browser framework and rendering strategy remain open.

## Dependencies and failure behavior

| Dependency | Required input or service | Failure rule |
|---|---|---|
| Registry / Gate | Governed inventory, explanations and action reads | Show access denial, unavailable data or stale coverage; never invent success. |
| Sentinel | Timeline coverage and operational observations | Expose monitoring gaps and source watermarks. |
| Council / Warden | Governed approval and suspension APIs | Changed/expired requests require new review; no direct administrative fallback. |
| Human identity provider | Established session federation and roles | Expired or unverifiable sessions cannot perform privileged requests. |

No dependency is linked into this scaffold. Supported contract versions are **none**.
Future adapters must consume a reviewed, versioned contract and identify its digest;
a floating hub branch is design context, never deployment authority.

## Threat assumptions

Treat agent code, supplied content and self-reported identity as untrusted.
Host administrators, release roots and required signing authorities remain explicit
trust assumptions of a qualified deployment. Process separation alone does not prove
independent administration.

| Threat | Required control to implement and test |
|---|---|
| Confused deputy or hidden administrator | Governed APIs only; session context is checked server-side for each request. |
| Stale approval and optimistic success | Bind exact reviewed intent; use authoritative outcome vocabulary. |
| Tenant leakage or browser attack | Tenant-scoped reads, session/CSRF protection, and no target secrets in UI state. |

The [validation specification](validation.md) connects these requirements to the hub
invariants. No test evidence is implied by this design.

## Decisions needed before implementation

Choose one web framework only after read APIs and session/CSRF design are reviewed. Decide pagination/freshness and exact approval binding; keep a headless alternative. No UI framework is implied by using Rust for these application interfaces.

A cross-component semantic change starts in a hub decision record. Keep publication,
activation and component implementation separate. Use expand, migrate, remove for
future breaking contract changes; never duplicate hashing, identity or grant rules.

## Deferred scope

Write UX before Council/Warden contracts, bulk approval, arbitrary dashboards, and workflow designers.

The [implementation plan](implementation-plan.md) sequences the first useful increment.
No deployment recipe, service port or live-provider configuration is supplied at this stage.
