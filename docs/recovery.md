---
description: Inspect interrupted package operations and restore only durably completed ledger updates.
---

# Interrupted transaction recovery

An interrupted package operation may leave a journal recording its intended
ledger update. Recovery reconciles that bookkeeping; it does not undo package
files, replay failed hooks, or rerun pacman automatically.

## Inspect the operation

```sh
pacvamp recover
pacvamp recover --json
```

The report compares each intended version or removal with current installed state.
It includes the original journal, whether restoration is allowed, suggested
commands, and bounded pacman log context. Select one report with `--id OPERATION_ID`.

Log context covers at most the last 256 KiB and 50 matching lines since the intent.
Rotation, missing logs, and concurrent operations can leave that context incomplete.

## Restore completed bookkeeping

When the report permits restoration, replace `OPERATION_ID` with its journal ID:

```sh
pacvamp recover --id OPERATION_ID --write
```

Restoration requires both a durably recorded successful pacman operation and
installed state that still matches the journal. Matching versions or log entries
alone never turn an uncertain operation into trusted evidence.

## Resolve uncertain or divergent operations

Inspect the differences and reconcile the host through an explicitly reviewed
package operation. Once you have inspected a journal that should be forgotten:

```sh
pacvamp recover --discard OPERATION_ID
```

Discard removes that journal entry only. It does not change packages or certify
their evidence. Use [snapshot rollback](/snapshots) or your separate filesystem
recovery procedure when package contents need restoration.

See the [recover reference](/cli/pacvamp/recover) for flags and the
[architecture](/architecture#intent-and-recorded-state) for journal ordering.
