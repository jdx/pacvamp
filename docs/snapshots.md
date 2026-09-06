---
description: Inspect tested snapshots, pin a mirror, and preview rollback with a configured snapshot publisher.
---

# Snapshots and rollback

A snapshot is a fixed repository package set. A channel points to a snapshot;
operators move the pointer after checks and promotion. These workflows require a
configured snapshot publisher, trusted feed key, and `[channel] snapshot_base`.
Installing pacvamp alone does not provision a distro snapshot service.

## Inspect the channel

```sh
pacvamp channel
pacvamp channel --offline --json
```

The report identifies the snapshot, promotion and hold status, reported tests,
and whether the mirrorlist is pinned. The offline form uses cached feeds.

A `tested` label means the package appears in the snapshot's reported test set.
Read the suite name and logs: the built-in consistency suite checks repository
files and hashes, while a boot or desktop suite exercises a different boundary.
A `snapshot` label does not mean the package itself was exercised. Neither label
proves that a package is harmless.

## Pin and unpin

Replace `SNAPSHOT_ID` with an ID from your configured publisher:

```sh
pacvamp channel pin SNAPSHOT_ID
pacvamp channel unpin
```

Pinning verifies the snapshot manifest and promotion, backs up the original
`/etc/pacman.d/mirrorlist` once, and changes the mirrorlist to the fixed snapshot.
It does not immediately install or downgrade packages. Unpinning restores the
saved mirrorlist; it does not restore earlier installed versions.

## Preview a rollback

```sh
pacvamp rollback --snapshot SNAPSHOT_ID --dry-run
```

Review the downgrade plan before running the same command without `--dry-run`.
Execution pins the mirror, refreshes databases, and runs a repository sync with
downgrades allowed. By default, the target must have reached `rc` or `stable`.

Repository rollback is not a filesystem restore. It does not undo application
data migrations, restore all configuration, or roll AUR packages back to prior
builds. Pair it with a separately managed filesystem backup/snapshot procedure.
[Recovery](/recovery) addresses interrupted bookkeeping, not restoration of files.

For publisher formats and promotion rules, see the
[release-train](/spec/release-train) and [snapshot-store](/spec/snapshot-store)
specifications. The [channel](/cli/pacvamp/channel) and
[rollback](/cli/pacvamp/rollback) references list advanced flags.
