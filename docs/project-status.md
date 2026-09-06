---
title: Status and limitations
description: What pacvamp implements, what depends on your publisher, and where the proof of concept stops.
---

# Status and limitations

pacvamp is a proof of concept for pacman-based Linux systems. It is experimental,
unsupported, and not ready for real use. Use a disposable machine when evaluating it.
The packaged installation targets x86-64 Arch Linux; AUR builds require a Linux
kernel that can enforce the requested confinement.

## What you can try

| Capability | What it does today | Start here |
| --- | --- | --- |
| Package transactions | Previews and runs pacman installations, removals, and upgrades | [Package operations](/packages) |
| Declared packages | Layers manifests, previews differences, and applies explicit present/absent declarations | [Manifests](/manifests) |
| AUR builds | Reviews and approves recipe commits, confines makepkg, and records local build receipts | [AUR guide](/aur) |
| Repository evidence | Verifies configured signed feeds and supported package sidecars | [Security model](/security-model) |
| Build management | Controls resources, supports optional clean images, and retains artifacts for inspection | [Build controls](/build-controls) |
| Publisher tooling | Produces indexes, provenance, gated signatures, snapshots, and tool channels | [Run a registry](/operations/registry) |

## What depends on configuration

Installing pacvamp does not turn on every protection it supports. Managed policy,
kernel capabilities, trust roots, and the evidence a publisher supplies determine
what a machine can enforce. Run `pacvamp doctor` to inspect those conditions, or
`pacvamp doctor --refresh` to refresh signed feeds. See
[how to read the report](/protection-status).

An `arch`, `opr`, `aur`, or `custom` label identifies origin. It is not a safety
rating. Arch repositories retain pacman's signature checks without acquiring
pacvamp provenance they do not publish. The independent pacvamp registry is a
custom repository; its name does not make it OPR.

Snapshot pinning and tool downloads need configured publishers and keys. The
[adoption guides](/adoption/omarchy) describe integration work, not a claim that
Omarchy, OPR, or mise has deployed every step.

## Security boundaries

- A reviewed recipe and a signature do not prove a package is harmless. Installed
  package scripts run with pacman's privileges, outside the AUR build jail.
- The default jail restricts filesystem access and internet sockets. It does not
  provide a complete process or Unix-socket namespace. The optional
  [clean-chroot backend](/clean-chroot) adds namespaces.
- [Receipts](/build-receipts) record local build observations. They are not
  independent attestations or proof of reproducibility.
- Local recipe scanning does not query OSV or Socket. `policy.safe` and
  `policy.scanner.socket_token` are unsupported and rejected even if another
  configuration layer would override them.
- Build provenance uses `.provenance.json`. The reserved `.sigstore.json` build
  sidecar is not a substitute for this verification path. Packslip release
  bundles have their own implemented verification path.
- A snapshot's `tested` label identifies membership in its reported test set.
  Inspect the suite: a consistency check does not establish desktop or hardware
  compatibility. See [snapshots](/snapshots).
- The reference registry stores its signing keys on one isolated host. Separate
  signing identities alone do not provide separate custody.

Read the [security model](/security-model) for evidence and enforcement boundaries,
and the [roadmap](https://github.com/jdx/pacvamp/blob/main/PLAN.md) for remaining work.
