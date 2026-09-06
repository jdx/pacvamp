---
description: How the pacvamp client, pacman engine, policy library, and repository publisher fit together.
---

# Architecture

pacvamp adds policy and recorded intent around pacman's package transactions.
The client can operate against ordinary pacman repositories; richer provenance,
verdicts, and snapshots require a publisher that supplies the corresponding feeds.

## Components

```text
User commands + layered manifests
                 │
              pacvamp
       ┌─────────┼───────────┐
       │         │           │
   alpm-db   AUR review   trust checks
       │     and build       │
       └─────────┼───────────┘
          transaction engine
                 │
              pacman
                 │
       installed packages + ledger

pacvamp-repo → signed feeds, provenance, snapshots, tool channel
packslip     → external vendor release format and verifier
```

| Component | Responsibility |
| --- | --- |
| `crates/pacvamp` | CLI, manifests, AUR workflows, trust, transactions, channels, and interactive views |
| `crates/alpm-db` | Native reads of pacman configuration/databases and compatible version comparison |
| `crates/pacvamp-policy` | Shared findings and decisions for client AUR review and repository sync gating |
| `crates/cli-support` | Shared argument, usage, and version handling |
| `crates/pacvamp-repo` | Indexes, provenance, signing gate, vendor processing, verdicts, snapshots, and tools |
| `plugins/mise-tool-channel` | A mise backend that delegates verified tool downloads to pacvamp |
| `harness` and `e2e` | Snapshot suite contract, boot acceptance, and real Arch package tests |

Packslip 1.x is a crates.io dependency, not a workspace crate or bundled CLI.
See the [integration notes](/spec/packslip).

## Package resolution

The client reads `pacman.conf` and local/sync databases directly. `core`, `extra`,
and `multilib` map to `arch`; `omarchy` maps to `opr`; other configured repositories
map to `custom` and retain their names. Repository order determines name resolution;
`repo/name` selects an explicit repository. Provider matching supports virtual names.

The AUR is a separate source accessed through its RPC and Git interfaces. Commands
select it explicitly with `--aur` or a manifest's `source = "aur"`. An origin label
is distinct from verified package evidence; see the [security model](/security-model).

## Transaction engine

The `Engine` interface separates transaction requests from execution. It supports
planning, database refresh, package download, repository transactions, and local
package-file installation. `PacmanCli` is the implemented engine.

Plans use pacman's print mode. Transactions preserve pacman's prompts and hooks;
pacvamp supplies the Omarchy update-guard environment when invoking pacman.
Non-root transactions elevate through sudo; noninteractive elevation must already
be available. Makepkg runs as the build user, never as root.

Database reads do not link libalpm. This keeps queries independent of libalpm soname
changes while pacman remains authoritative for transactions. A native engine is
[future work](https://github.com/jdx/pacvamp/blob/main/PLAN.md), subject to mandatory
on-disk compatibility and acceptance tests.

## Intent and recorded state

The [manifest](/manifests) merges distro and user package declarations.
[Configuration](/configuration) applies ordinary settings followed by mandatory
policy floors. The lockfile records AUR commits and the evidence acknowledged at
approval; it is not a complete lock of repository versions or source bytes.

The root-owned ledger records completed package operations, accepted repository
evidence, index sequences, snapshots, and local receipt references. Before mutation,
the client journals the intended ledger patch. A successful pacman operation is
recorded durably before the patch is merged. An interruption can therefore leave a
journal to inspect with [recover](/recovery). Matching package versions alone never
turn an uncertain operation into accepted evidence.

## AUR build lifecycle

1. Fetch recipe metadata/history and review the selected commit against policy and
   any prior approval. Planning uses recipe metadata without running PKGBUILD code.
2. Record approval for the exact commit in the user's lockfile. Export that Git
   object into a private run directory rather than building a mutable checkout.
3. Fetch and verify sources in a confined disposable workspace. Destroy that
   workspace before the build phase; keep verified inputs read-only for building.
4. Run makepkg phases under [supervision and limits](/build-controls), with the
   configured filesystem/network jail and optional [clean image](/clean-chroot).
5. Validate output paths and write a [receipt](/build-receipts). Installation
   rechecks the output digest before pacman installs the package.

Per-pkgbase locks prevent conflicting recipe operations. Cache leases retain
artifacts through approval, installation, and ledger recording. [Cache pruning](/cache)
protects active builds and evidence referenced by installed or pending transactions.

## Update flow

`update` takes its operation lock, handles the pacman database lock, and refreshes
repository databases unless disabled. It obtains publisher information and plans
repository upgrades, configured/age holds, AUR candidates, and optional orphan removal.
A dry run returns the plan before package mutation and may populate metadata caches.

Execution confirms the plan, runs configured pre-hooks, applies repository changes
with transaction evidence checks, then handles approved AUR builds and requested
orphan removal. Post-hooks run around the transaction result; failures are reported.
The client reports retained packages and configuration files needing attention.
See [updates](/update-policy) for the user-facing meaning of success and skips.

Hooks implement the distro's surrounding backup/migration workflow. Pacvamp does
not itself provide a filesystem snapshot or a general undo operation.

## Publishing and adoption

Repositories use `pacvamp-repo` as command-line tooling rather than embedding its
implementation. Each format has one canonical specification:

- [Repository feeds](/spec/repository-feeds): signed package indexes, advisories, verdicts.
- [Build provenance](/spec/provenance): package-bound statements, transparency, signer gate.
- [Vendor pipeline](/spec/vendor-pipeline): verified packslip releases and repackaging.
- [AUR sync gate](/spec/sync-gate): candidate review and recorded auto-merge decisions.
- [Release train](/spec/release-train) and [snapshot store](/spec/snapshot-store): client and operator sides of immutable releases.
- [Tool channel](/spec/tool-channel): mirrored vendor artifacts for per-user mise tools.

The [reference registry](/operations/registry) is independent of OPR. The
[Omarchy](/adoption/omarchy), [OPR](/adoption/opr), and [mise](/adoption/mise) guides
are integration instructions, not evidence of adoption. The
[design decisions](/design-decisions) explain why these boundaries exist.
