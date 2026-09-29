# Security

Do not file a vulnerability as an issue or a pull request.

Report a suspected vulnerability in anything in this repository privately, by either route:

- GitHub's private vulnerability reporting ("Report a vulnerability" under the Security tab), or
- email to **info@ioka.io** with "security" in the subject.

Say what you found, where, and how to reproduce it. Do not include live credentials, customer data,
or a proof of concept run against a system you do not operate. You will get an acknowledgement
within two business days, and a fix, or a recorded decision, on the affected path before any related
release. Credit is given if you ask for it.

## Supported versions

Munarium Console has no release. Until the first tagged release, `main` is the only line and a fix
lands there. Once releases exist, security fixes go to the current minor release and to the previous
one for six months after its successor ships; an older release gets a fix only where the
vulnerability is in a contract it still speaks.

A finding in the design is welcome now, through the same private channel if it has security
consequences and as an ordinary issue otherwise. The threat model this component is built against
is in [README.md](README.md) and, for the platform as a whole, in the hub
([iokaio/munarium-platform](https://github.com/iokaio/munarium-platform)).

## What matters most here

As runtime behavior is implemented, these are the classes of finding taken most seriously and most
quickly:

- **A Console-only privilege**: any consequential operation Console can perform that is not available through a governed service API, or a direct table-write path that bypasses Council, Warden or the permitted policy-candidate intake.
- **Approval substitution and stale state**: an approval that binds to a changed request, a view that does not distinguish a fresh approval from a request that no longer matches the target or policy, or a bulk action that does not bind an explicit set of request hashes.
- **Role confusion and tenant leakage** between approver, operator and auditor views, or across tenants.
- **Session protection, CSRF, session expiry and audit attribution failures**, and secret handling in the browser or server.
- **A non-error response rendered as success**, or an unresolved payment, deployment or message offered a retry button.

## What is deliberate, and is not a defect

- **Console has no hidden administrative bypass.** Approval goes to Council, suspension to Warden, and a policy edit creates a candidate through the permitted intake; Console calls the same public, governed APIs as any authorized client. A report that Console "cannot do X directly" describes the design.
- **Front-end validation improves usability; server-side enforcement is authoritative.** Schema-generated forms cannot add fields the underlying contract excludes.
- **A demo identity configuration is not enterprise qualification.** A deployment may begin with a local test identity provider, and production documentation says so.

When a local development profile exists, its test identity provider, test broker, disposable target
and generated sample credentials are development conveniences confined to that profile. They are
not vulnerabilities in themselves. A path by which they reach a production deployment unnoticed is.

## Findings that cross components

A contract ambiguity that lets two components disagree about authority, a canonicalization
difference between clients, or a gap between what a release advertises and what its evidence
supports is still a security finding. Report it here, or to any other Munarium repository, through
the same private channel; it is routed to the hub and the affected repositories together. Do not
open a public issue for it in the hub.

## Secrets

If you have committed a token or key, treat it as compromised: rotate it first, then report it.
