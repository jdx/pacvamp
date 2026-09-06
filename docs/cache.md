---
description: Inspect retained AUR builds and preview cleanup without deleting active builds or recorded evidence.
---

# Build cache retention

Pacvamp retains build runs, outputs, logs, and receipts for inspection. Check what
is retained before deleting data, especially if you may need to
[compare or replay a build](/build-receipts).

## Inspect and preview

```sh
pacvamp cache status
pacvamp cache prune --dry-run
```

Both commands accept `--json`. Pruning defaults to runs older than 30 days.
`--older-than-days` changes that age; `--max-bytes` selects the oldest eligible
runs until the remaining total meets the requested size target.

Status and previews can inspect a running build under a shared lease. Sizes are
live estimates. Actual pruning takes an exclusive lease and recalculates eligibility.

## Remove eligible runs

```sh
pacvamp cache prune
```

Omitting `--dry-run` deletes eligible build runs. Recipe checkouts and
synchronization locks remain. Use the same user's cache and the system ledger that
records those builds; sharing a cache across independent sysroots is unsupported.

## What cleanup protects

Active builds block pruning. Runs less than an hour old are protected, as are
receipts referenced by the selected ledger or its pending transactions. Commands
that may build or install hold a shared lease through approval, installation, and
ledger recording, including long confirmation prompts.

If protected data exceeds the size target, it remains intact. Missing referenced
receipts cause cleanup to fail rather than guess. An unreadable run is retained
with unknown size (`bytes: null` in JSON); known size totals exclude it.

Pruning makes owned directories writable without changing file permissions or
following symlinks. Removal failures are reported per run while other cleanup
continues; any selected run that could not be removed makes the command fail.
See the [cache reference](/cli/pacvamp/cache) for selection flags.
