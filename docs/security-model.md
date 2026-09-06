---
description: What pacvamp’s approvals, build jail, signed feeds, and provenance checks establish—and what they do not.
---

# Security model

pacvamp combines explicit source choices, recipe review, confined builds, and
checks on publisher evidence. Each control answers a limited question. None proves
that arbitrary software is safe to install.

For machine-specific results, use [protection status](/protection-status). For
published signing identities, use [trust roots](/trust).

## Threats and controls

| Risk | Control | Boundary |
| --- | --- | --- |
| New or misleading AUR package | Age, reputation, similar-name findings, and recipe review | Heuristics can miss malicious recipes and flag legitimate ones |
| A recipe changes after approval | Approval and build tied to a Git commit | Upstream VCS inputs may still move independently |
| Recipe code reads credentials or changes unrelated files | Landlock/seccomp confinement in every makepkg phase, private HOME and scratch | Default jail is not a complete process or Unix-socket namespace |
| Build downloads unexpected dependencies | Separate networked source verification and network-denied build by default | Explicit build-network grants expand the boundary |
| Mirror changes or replays repository data | Signed indexes, database/package digests, recorded sequences | Initial trust roots and publisher custody still matter |
| A vendor artifact differs from its release statement | Packslip signature/pin, digest, size, and repository evidence policy | Vendor identity does not establish harmless behavior |
| A build host requests a package signature | Provenance verification at the signer gate | Isolation requires a genuinely separate signer and trustworthy build identity |
| User configuration disables mandatory protections | Managed policy floor | Administrators must control the managed files and their selection |
| Automation encounters a risky candidate | Refusal or reported skip instead of implicit approval | Successful updates can retain older packages |

## Origin and evidence

`arch`, `opr`, `aur`, and `custom` are source labels. They are not interchangeable
with evidence levels:

- Arch packages use pacman's configured signature policy. Pacvamp does not invent
  build provenance for a publisher that does not supply it.
- OPR and custom publishers can supply [signed indexes](/spec/repository-feeds),
  [provenance](/spec/provenance), vendor manifests, and review feeds. The client
  applies the configured evidence policy to the selected transaction.
- AUR review inspects the selected recipe history and records the decision to
  build it. Recipe checksums and [local receipts](/build-receipts) do not provide
  independent publisher provenance for the installed output.

A signed feed authenticates the publisher's claims. `doctor` counts advertised
evidence without verifying every package sidecar. Transaction checks bind supported
evidence to the selected files; `verify` provides a separate inspection path.
Read the relevant command's report rather than treating all three as equivalent.

## Recipe findings

The policy library evaluates signals including package and commit age, maintainer
changes, orphan status, reputation, similar names, changed source domains, skipped
checksums, VCS sources, install scripts, large diffs, commit drift, suspicious
content, language package-manager commands, and published verdicts/advisories.

These are local checks and authenticated feed inputs. OSV and Socket lookups are
not implemented. Reviewer kinds describe external feed producers; they do not
cause an antivirus scan or AI review to run automatically.

Approval records the reviewed recipe commit and acknowledged evidence. It does not
turn off an explicit install-script denial. See [AUR workflows](/aur) and
[configuration](/configuration) for the controls a user can actually set.

## Build isolation

The source verification workspace is disposable and separate from the later build.
Every makepkg phase receives a private HOME and a filtered environment; filesystem
rules separately restrict credential access. Verified sources are read-only while
building. Writable output, logs, and scratch belong to the private run directory;
shared temporary directories are not general writable scratch.

A network grant does not grant unrestricted filesystem access. However, current
network grants are configured by package name, not bound to the approved commit.
When required jail enforcement is unavailable, the build fails instead of silently
running unconfined.

[Resource controls](/build-controls) supervise phases and constrain resources.
Per-process limits and sampled disk accounting differ from aggregate cgroup limits.
The optional [clean-chroot backend](/clean-chroot) adds namespace isolation;
opt-in delegated cgroups add aggregate limits and independent cleanup.

The boundary ends at package installation: pacman applies files and executes package
scripts with its normal privileges. Confining makepkg does not confine the installed
application or its installation hooks.

## Publisher verification and remaining gaps

The package-signing key, feed key, and build key have separate roles. The
[reference registry](/operations/registry) currently keeps them on one host;
separate custody is a deployment objective, not an implemented guarantee of that setup.

The signer verifies the supported provenance envelope and can verify a Rekor Merkle
proof and checkpoint signature when given the log public key. The client's separate
`verify` transparency report currently checks the entry's payload binding and the
presence of a proof; it does not authenticate that proof or checkpoint. Signed
entry timestamps are not yet verified. See the [provenance spec](/spec/provenance)
for exact paths. Do not equate a successful client report with authenticated public
log inclusion.

The reserved `.sigstore.json` build sidecar is not currently a verified alternative
to `.provenance.json`. This limitation is separate from implemented packslip v1
bundle verification in vendor and tool-channel workflows.

## Validate the boundaries

The [security acceptance suite](/security-testing) exercises adversarial recipes
and kernel enforcement. The [development guide](/development) distinguishes local
fixtures, real Arch transactions, and VM lifecycle tests. Those checks support
specific guarantees; they cannot certify every package, host configuration, or
future distro integration.
