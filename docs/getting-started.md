---
description: Inspect your installation, preview a package transaction, and choose whether to record a package in your manifest.
---

# First steps

Start on a disposable Arch machine with pacvamp installed using the
[installation guide](/install). Run the commands below as your normal user;
pacvamp asks for elevation when a package transaction needs it.

## Check the machine

```sh
pacvamp version
pacvamp doctor
```

`doctor` reports configuration and available protections. Its default feed checks
use the authenticated cache. To fetch current publisher feeds, run
`pacvamp doctor --refresh`. Follow the reported failures before attempting a
transaction; [protection status](/protection-status) explains each check.

## Find and preview a package

```sh
pacvamp search helix
pacvamp info helix
pacvamp install helix --dry-run
```

Search reads your configured repository databases. Info shows the source and
available evidence. The installation preview shows every package pacman would
change and the command it would run. If your databases have not been populated,
complete the installation guide's full system update first.

## Choose how to install

For a one-time installation:

```sh
pacvamp install helix
```

Review the plan and confirm. This records the transaction in the ledger but does
not add a manifest declaration.

To record helix as a package you want on this machine:

```sh
pacvamp add helix
pacvamp status
```

`add` writes your manifest and installs the named package. It can also declare an
already installed package. `status` reports how installed packages compare with
your declarations. Read [manifests](/manifests) before using `drop` or absent entries.

## Preview an update

```sh
pacvamp update --dry-run
```

The preview includes holds and AUR blockers. It may fetch recipes and signed feeds,
but it does not approve commits or install packages. See
[updates and blocked packages](/update-policy) before running `pacvamp update`.

For AUR software, follow the separate [review and build workflow](/aur). For an
existing machine, [import](/migration) can preview package declarations without
changing installed packages or granting trust to their binaries.
