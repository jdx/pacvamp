# Roadmap

This document tracks remaining work and unresolved decisions. Current behavior is
in the [user guides](https://pacvamp.com/getting-started),
[architecture](docs/architecture.md), and [specifications](docs/spec/repository-feeds.md).
The [design decisions](docs/design-decisions.md) explain the constraints on future work.

pacvamp remains a proof of concept. The items below are completion criteria,
not release commitments or a priority order. Remove completed items and update
their documentation in the same change.

## Client and verification

- **Sigstore build-provenance sidecars:** add verification of the reserved
  `.sigstore.json` format alongside the implemented `.provenance.json` path.
  Completion requires binding the signer and statement to the exact package,
  enforcing log policy, and fixtures for invalid identities, digests, and proofs.
  This is distinct from the existing packslip v1 bundle verification.
- **Transparency authentication:** close the remaining distinction between
  checking a Merkle proof and authenticating the log. Document the trusted-key
  path, verify signed entry timestamps where used, and test forged checkpoints
  and log identities across client and signer workflows.
- **External scanner integration:** decide whether an external malicious-package
  lookup adds useful evidence beyond local recipe findings and signed feeds.
  Before adding one, specify supported subjects, authenticated results, freshness,
  failure policy, and adversarial tests. OSV/Socket lookups are not implemented;
  do not expose configuration that suggests otherwise.

## Build isolation and acceptance

- **Default build backend:** decide whether the optional clean-chroot backend
  should become the default after measuring compatibility, startup cost, disk
  use, and enforcement on supported Arch hosts. Keep failures explicit and
  document any migration before changing the default.
- **Distro acceptance coverage:** extend the existing Arch boot/update/rollback
  fixture into an Omarchy desktop and hardware matrix. Completion means testing
  actual supported images and upgrade paths, retaining logs, and binding imported
  test evidence to the exact snapshot and package artifacts exercised.

## Future native engine

A native transaction engine remains future work. Introduce it behind the existing
engine interface only after defining compatibility tests for dependency resolution,
package signatures, extraction, scriptlets, hooks, and local database writes.
Pacman's on-disk compatibility is mandatory; see the
[engine decision](docs/design-decisions.md#keep-pacman-compatible).

## External adoption

These changes belong to the adopting projects. The presence of a guide or helper in
this repository does not establish that an integration is deployed.

| Project | Remaining integration | Completion criteria |
| --- | --- | --- |
| Omarchy | Adopt package helpers, manifests, managed policy, and snapshot channels | Rehearse installation, updates, failures, and rollback; verify caller identity and required package versions before retiring existing paths. See the [guide](docs/adoption/omarchy.md). |
| OPR | Deploy index, provenance/signing, vendor and AUR gates, snapshots, and tool publishing | Distribute trust roots, separate signing custody, and exercise publish, hold, promotion, rollback, and key rotation in the actual deployment. See the [guide](docs/adoption/opr.md). |
| mise | Publish/consume packslips and support vetted channels natively | Verify pinned identities and rollback policy, preserve per-user versioned tools, and provide a migration from the included backend plugin. See the [guide](docs/adoption/mise.md). |

## Registry operations

- **Signing custody:** move beyond the reference registry's single-host key
  storage. Establish separate signer custody and a supported hardware-backed
  signing path before claiming that a build-host compromise cannot forge a
  repository release. Exercise rotation and recovery with the published trust roots.
- **Operational readiness:** rehearse backup restoration, failed publishing,
  held releases, and client recovery on the independent registry. Record the
  supported deployment and its limits in the [operations guide](docs/operations/registry.md)
  before changing the project's proof-of-concept status.
