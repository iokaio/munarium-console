# Munarium Console build plan

**Proposed work; no functional milestone is complete.** The design baseline is the
[public platform plan, revision 4](https://github.com/iokaio/munarium-platform/blob/main/docs/platform-plan.md), section 15, and its
stage sequence in section 25. Console's initial delivery belongs to **Stage 3**.
Calendar windows are planning targets; acceptance evidence controls advancement.

## Preparation present in this checkout

- A non-publishable, dependency-free Cargo library with documented interface modules.
- An [architecture map](architecture.md) naming ownership, trust assumptions and failures.
- An [acceptance specification](validation.md) and automatic Rust build checks.
- Existing contribution, security, support and repository-hygiene processes.

These artifacts prepare implementation; they do not complete Stage 0 foundation qualification
or advance this repository beyond the hub's **repository created** catalog state.

## First work packet: CONSOLE-01: read-only inventory and decision views

**Prerequisites:** accepted hub decisions and the specific contracts named in
[Architecture](architecture.md); record the exact revisions used. All fixtures must be synthetic
or authorized public inputs. The hub [contract backlog](https://github.com/iokaio/munarium-platform/blob/main/docs/architecture/contract-backlog.md)
tracks unresolved cross-component definitions.

**Work:** After governed read contracts and session requirements are accepted, build a minimal view of agent owners, manifests, denial explanations and action traces using synthetic tenant-scoped fixtures. Select the browser approach separately. Show loading, empty, denied, stale and unavailable states explicitly.

**Permitted scope:** the relevant modules under `src/`, component-local tests/fixtures,
and their documentation. Add dependencies, runtime wiring, or migrations only when the packet
requires them and its owner has reviewed the design. Do not copy sibling implementations.

**Acceptance:** Two tenants cannot see each other's records. Keyboard users can inspect source references and distinguish unresolved from completed. No approval, retry, suspension or direct table-write path is added by the read-only slice.

**Handoff:** retain commands, exit codes, fixture/contract revisions, limitations and the
diff for review. A test specification is not a passed test. Publishing, deployment, live
provider calls, signing changes and policy activation are separate operations.

## Subsequent packets

| Packet | Implementation scope | Exit condition |
|---|---|---|
| CONSOLE-02 | Integrate a real federated session path and session/CSRF protections. | Expired sessions, role confusion, cross-tenant requests and CSRF attempts are refused server-side. |
| CONSOLE-03 | Add exact-request approval and bounded suspension via public APIs. | A changed digest, target, expiry or prior state invalidates the old review. |
| CONSOLE-04 | Add headless parity and accessibility checks. | UI and headless paths use identical service authority; unresolved offers investigation rather than blind retry. |

Each packet gets a concrete component issue and links to the coordinating hub issue when
execution begins. The identifiers above are local planning references, not claims that remote
issues or approvals already exist. Work advances one coherent capability slice at a time.

## Integration and operational readiness

Before any runtime capability is advertised, document its supported contracts, immutable source
revision, accepted dependency versions and deployment boundary. Demonstrate relevant failure
paths from [Validation](validation.md), then add the component runbook: required identities,
health and dependency states, migration order, backup/restore, key rotation where applicable,
and unresolved-work investigation.

A component result alone is not platform qualification. The hub's
[delivery sequence](https://github.com/iokaio/munarium-platform/blob/main/docs/build-plan.md) requires composition evidence, including the
Server/Matrix foundation and the authority path required by the selected consequence class.
Write UX before Council/Warden contracts, bulk approval, arbitrary dashboards, and workflow designers.

## Completion criteria for the first functional increment

- The documented local recipe works from a clean clone using bounded disposable inputs.
- The acceptance cases are executable, retain their intended oracle, and include refusal paths.
- Unsupported operations remain explicit; logs and reports expose no credentials or private data.
- The README links the actual evidence before any capability or release label changes.
