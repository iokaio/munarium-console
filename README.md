# Munarium Console

**One interface for approvers, operators, and auditors.** Console is the human-facing view of the
Munarium Governance Platform: inventory, decision explanations, pending approvals, unresolved
actions and operational status, in one reusable web interface with a command-line alternative.
Every consequential thing a person does in Console is a call to a public, governed service API.
Approval goes to Council, suspension goes to Warden, a policy edit creates a candidate through the
permitted intake. **Console has no hidden administrative bypass.**

> **Status: Planned — Rust scaffold present.** This checkout contains a dependency-free,
> non-publishable [Cargo library](Cargo.toml) and documented interfaces under [src/](src/lib.rs).
> The interfaces have no implementations: no runtime service, client transport, database,
> provider integration or contract implementation is available. No production path is qualified.
> Build checks validate source structure, not governance capabilities. The
> [capability table](#capability-status) remains the authoritative functional status.

Console is one of nine components built around the existing Munarium foundation, Munarium Server
and Munarium Matrix. Their shared architecture, normative contracts, decision records, roadmap and
composition evidence live in the public hub,
[iokaio/munarium-platform](https://github.com/iokaio/munarium-platform). This repository will hold
Console's implementation, its tests, operational diagnostics, package definitions, a local
development recipe and release evidence. It is open source from its first public commit, under the
Apache License 2.0, with no proprietary edition or administration tier.

## Start building

Read the [development index](docs/README.md), then the [architecture](docs/architecture.md),
[implementation plan](docs/implementation-plan.md) and [validation guide](docs/validation.md).
They map the public platform plan to source modules, dependencies, a first bounded work item
and acceptance cases. Runtime capabilities remain planned; supported contract versions are **none**.

## What Console is for

The assurance plane reads authoritative records and may hold rebuildable views. When an operator
uses Console to approve, revoke or suspend, Console calls the same Council or Warden API as any
other authorized client. Visual polish must not outrun the APIs that enforce its behavior, and an
accurate interface must reveal the system's uncertainty rather than conceal it.

## The design, as planned

### Read first, then governed interaction

The first public increment can be read-only. An operator should be able to identify an agent owner,
inspect a manifest, understand a denial and trace an action without editing effective policy. An
approver view arrives when Council's binding and authentication contracts are ready.

Every consequential Console action calls a public, governed service API. **Console has no direct
table-write connection** to bypass those services.

Human authentication uses an established identity provider through a supported federation method.
A deployment may begin with a local test identity provider; production documentation does not treat
a demo identity configuration as enterprise qualification. Session protection, CSRF defenses, role
boundaries, audit attribution and secret handling are part of the release scope.

### The approval experience

An approval view shows the exact proposed effect, target, request digest, evidence, obligations,
expiry and relevant prior state. The user can distinguish a new approval from a **stale request**
that no longer matches the target or policy. A changed request creates a new review rather than
silently updating the details behind an old approval button.

The interface distinguishes **proposed, authorized, dispatched, completed and unresolved**. It does
not render every non-error response as success. An unresolved payment, deployment or message is an
investigation item, not an invitation to press retry.

Console reduces approval fatigue by showing the decision that requires human judgment and the
specific obligation to satisfy. A bulk approval feature, if later added, binds an explicit set of
request hashes and retains the same role and evidence requirements as individual approvals.

### Solo scope and accessibility

One reusable web interface rather than separate consoles per role. Keyboard operation, readable
contrast, useful empty states and exportable diagnostic references are requirements. A
command-line alternative remains important for headless and restricted deployments.

The first release does not attempt a general workflow designer, an arbitrary dashboard builder or a
proprietary administration tier. Schema-generated forms can reduce implementation effort, but they
must not allow fields that the underlying contract excludes. **Front-end validation improves
usability; server-side enforcement remains authoritative.**

## First public increment

**Read-only inventory and decision views, before any write experience**: agent owners, manifests,
denial explanations and action traces, drawn from Registry and Gate through their governed read
APIs, behind a federated human login.

Target window: Stage 3 (months 7–9); the approver view arrives when Council's binding and authentication contracts are ready.

## Capability status

The labels are evidence labels, not editions: **Planned**, **Experimental**, **Conformance-tested**,
**Reference-qualified**, **Independently reviewed**. In the hub's component catalog this
repository is at **repository created**.

| Capability | Status | Evidence |
|---|---|---|
| Read-only inventory: agent owners and manifests from Registry | Planned | none |
| Decision explanations and action traces from Gate's records | Planned | none |
| Operational status and Sentinel's timeline views | Planned | none |
| Human authentication through an established identity provider (local test IdP for development) | Planned | none |
| Session protection, CSRF defenses, role boundaries, audit attribution, secret handling | Planned | none |
| Approver view bound to Council's request-binding and authentication contracts | Planned, after Council's contracts | none |
| Suspension through Warden's API | Planned | none |
| Policy candidate creation through the permitted intake | Planned | none |
| Unresolved-action views that offer investigation, never retry | Planned | none |
| Command-line alternative for headless and restricted deployments | Planned | none |
| Bulk approval bound to explicit request-hash sets | Deferred | none |
| Workflow designer, dashboard builder, administration tier | Not planned | none |

Supported contract versions: **none**. Supported identity providers: **none**. Operations available
today: **none**.

## Acceptance evidence for the first release

| Test | Required outcome |
|---|---|
| Role confusion | An approver, operator or auditor sees and can do only what the role permits |
| Approval substitution | An approval binds the exact request digest; a changed request needs a new review |
| Stale state | A request that no longer matches the target or policy is shown as stale, not approvable |
| Tenant isolation | No view or action crosses a tenant boundary |
| Session expiry | An expired session performs no action and leaks no state |
| Console-only powers | None exist: every consequential action is reproducible through the governed API alone |
| Direct database write | None exists |
| Non-error rendered as success | Never; unresolved is rendered as unresolved |
| Keyboard operation and contrast | The interface is operable and readable without a pointer |

A blank evidence field means unverified, not passed.

## Invariants

| ID | Required property | Owner and first gate |
|---|---|---|
| INV-11 | An unresolved effect is never blindly repeated | Gate, connector, Harness, Console; stages 2–3 |
| INV-16 | Console cannot perform a privileged operation unavailable through governed APIs | Console, Council, Warden; stage 3 |
| INV-22 | A release advertises only the profiles and capabilities supported by its evidence | every component; every stage |

## Contracts, dependencies and neighbors

- **Contracts.** The hub's contracts directory is normative for every API Console calls and for
  the outcome vocabulary it renders. Console implements no contract of its own that the services do
  not enforce. Supported contract versions: none yet.
- **Foundation.** Munarium Server 1.3.0 and Munarium Matrix 1.2.0 already carry their own
  operator interfaces; Console does not replace them and reads the platform's records through the
  governed APIs, not Server's tables.
- **Council** is the approval API; **Warden** is the suspension API; **Registry** is the inventory
  and the policy-candidate intake; **Gate** is the source of decisions and traces; **Sentinel**
  supplies timeline and health views; **Harness** shares the outcome vocabulary and the rule
  against retrying the unresolved.
- **External dependencies.** An established identity provider through a supported federation
  method; the web stack is chosen with the first implementation and recorded in the hub.

## Not in scope

- Any privileged path that bypasses Council, Warden or the permitted intake.
- Separate consoles per role, a workflow designer, a dashboard builder, or a proprietary
  administration tier.
- Fields in schema-generated forms that the contract excludes.
- Treating a demo identity configuration as enterprise qualification.
- Compensating for a weak authority model with a good interface.

## Roadmap position

| Stage | Console's part |
|---|---|
| 0 · month 1 | This repository |
| 1–2 · months 2–6 | Consumes the read APIs and outcome vocabulary as they land; a command-line approval path in Council is sufficient for the first governed action |
| 3 · months 7–9 | Read-only inventory, explanations and traces; role-safe interactions; suspension through Warden; approver view when Council's contracts are ready |
| 4 · months 10–12 | Unresolved-work views, evidence references for Assure, inclusion in the reference composition |
| 5 · months 13+ | Bulk approval with bound hash sets, further views, demand-led |

UI polish is among the first things reduced when capacity is constrained; the authority model is
not.

## Repository layout

| Path | What exists |
|---|---|
| [Cargo.toml](Cargo.toml), [Cargo.lock](Cargo.lock) | Independent library, version 0.1.0-dev, publishing disabled, no external crate dependencies |
| [src/lib.rs](src/lib.rs) | Documented proposed module interfaces; no runtime implementations |
| [docs/](docs/README.md) | Architecture, implementation sequence and acceptance specifications |
| [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md) | Contribution process and aligned development guidance |
| [.github/workflows/](.github/workflows/) | Automatic Rust, repository-hygiene and DCO checks |
| [scripts/](scripts/), [check_license.py](check_license.py) | Existing documentation, private-material and license checks |
| [LICENSE](LICENSE), [NOTICE](NOTICE), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) | Licensing and dependency notices |

Subsystem modules: [views](src/views.rs), [session](src/session.rs), [commands](src/commands.rs).
Tests, fixtures, migrations, binaries and deployment assets arrive with the implementation that
uses them. The scaffold defines no shared wire types and depends on no sibling checkout.

## Development

Use Rust 1.98.1 with rustfmt, Clippy and the platform's native linker. From this repository root:

```console
cargo fmt --all --check
cargo build --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
cargo doc --offline --locked --no-deps
```

The crate currently has **zero runtime or conformance tests**. A successful test command checks
the scaffold only. The [validation guide](docs/validation.md) gives the required behavioral
test specifications and explains how to retain evidence when they are implemented.

Also run the existing hygiene gates:

```console
py check_license.py
py scripts/private_material_scan.py
py scripts/docs_linkcheck.py
gitleaks dir . --config .gitleaks.toml --no-banner --redact --exit-code 1
git diff --check
```

Use `python` or `python3` where `py` is unavailable. The new
[Rust workflow](.github/workflows/rust.yml) runs on main pushes and pull requests alongside
the existing [repository hygiene](.github/workflows/repo-hygiene.yml) and
[DCO](.github/workflows/dco.yml) workflows. They provide build and repository checks, not a
qualified runtime. No package is published or service deployed by these workflows.
Local checks do not imply hosted CI success. See [CONTRIBUTING.md](CONTRIBUTING.md).

## The platform

| Repository | Plane | Role |
|---|---|---|
| [iokaio/munarium-platform](https://github.com/iokaio/munarium-platform) | hub | Architecture, normative contracts, decision records, roadmap and composition evidence for the whole platform |
| [iokaio/munarium](https://github.com/iokaio/munarium) | foundation (mediation) | Munarium Server: governed memory, the append-only ledger, and the Server client libraries |
| [iokaio/munarium-matrix](https://github.com/iokaio/munarium-matrix) | foundation (mediation) | Munarium Matrix: governed, read-only structured evidence from enterprise data sources |
| [iokaio/munarium-registry](https://github.com/iokaio/munarium-registry) | authority | Inventory of agents, tools, manifests, and policy bundles |
| [iokaio/munarium-harness](https://github.com/iokaio/munarium-harness) | agent | SDKs that make the governed path easy for honest agents |
| [iokaio/munarium-warden](https://github.com/iokaio/munarium-warden) | authority | Workload identity, delegation, just-in-time credentials, kill switches |
| [iokaio/munarium-gate](https://github.com/iokaio/munarium-gate) | mediation | Policy decision and enforcement point for every tool call |
| [iokaio/munarium-gateway](https://github.com/iokaio/munarium-gateway) | mediation | Model-call mediation: routing, BYOK, budgets, screening |
| [iokaio/munarium-council](https://github.com/iokaio/munarium-council) | authority | Approvals, policy lifecycle, ratified governance transitions |
| [iokaio/munarium-sentinel](https://github.com/iokaio/munarium-sentinel) | assurance | Telemetry, anomaly detection, circuit breakers, incident replay |
| [iokaio/munarium-assure](https://github.com/iokaio/munarium-assure) | assurance | Control-framework mapping and evidence packs |
| [iokaio/munarium-console](https://github.com/iokaio/munarium-console) | assurance | One interface for approvers, operators, and auditors |
| [iokaio/munarium-clients-publish](https://github.com/iokaio/munarium-clients-publish) | tooling | The one place Munarium client packages are built for release and published from |
| [iokaio/munarium-demo](https://github.com/iokaio/munarium-demo) | examples | Munarium Demo: working applications and bundled datasets for evaluating the foundation |

The development tool VCP ([iokaio/vcp](https://github.com/iokaio/vcp)) is separate: not one of the
nine components and not a runtime dependency for adopters. Ioka's private repositories hold
planning material awaiting publication review and the proprietary Matrix analytics adapters;
nothing from them is copied into a public repository without that review.

## Licensing

Apache-2.0 ([LICENSE](LICENSE), [NOTICE](NOTICE)). The names are not part of that grant:
[TRADEMARK.md](TRADEMARK.md) says what you may do without asking, which is most things. There is
no proprietary edition of this component and none is planned; a capability that arrives later is
deferred roadmap work, not a commercial restriction.

## Contributing, support, security

Signed-off pull requests, no CLA ([CONTRIBUTING.md](CONTRIBUTING.md)). Questions go to Discussions,
defects and design findings to Issues, and suspected vulnerabilities to the private channel
[SECURITY.md](SECURITY.md) names, never a public issue. What is and is not supported:
[SUPPORT.md](SUPPORT.md). Conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Release history,
such as it is: [CHANGELOG.md](CHANGELOG.md).
