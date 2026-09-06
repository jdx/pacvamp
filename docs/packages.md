---
description: Search, install, remove, and inspect packages, with command-specific preview and automation behavior.
---

# Package operations

Use `install` and `remove` for installed package state. Use [manifests](/manifests)
when you also want to record what the machine should have. These workflows share
policy checks, but only the declarative commands edit your package choices.

## Find a package

```sh
pacvamp search helix
pacvamp info helix
pacvamp search --aur google-chrome
```

Repository search uses local sync databases in `pacman.conf` order. An explicit
`repo/name`, such as `extra/helix`, selects a repository. AUR search is separate and
fetches metadata; it does not approve a recipe.

## Install and remove

```sh
pacvamp install helix --dry-run
pacvamp install helix
pacvamp remove helix --dry-run
pacvamp remove helix
```

Review the affected packages before confirming. Removal also removes unneeded
dependencies by default; `--keep-deps` retains them. Neither command edits the
manifest, so `apply` may later reinstall a package you removed imperatively.

Run as your normal user. Pacvamp elevates the pacman transaction with sudo when
needed; `-y` does not supply sudo credentials. AUR builds must run as a non-root
user. See the [AUR guide](/aur) for `install --aur`.

## Inspect installed packages

```sh
pacvamp list --explicit
pacvamp list --orphans
pacvamp list --unverified
pacvamp audit
pacvamp pacnew
```

`audit` matches installed versions against Arch's security tracker. It is not an
AUR malware scan. `pacnew` lists configuration files that need attention; inspect
them with `pacvamp pacnew --diff` and merge changes manually; pacvamp does not
merge these files.

`pacvamp present helix` exits successfully when a matching package or provider is
installed. `pacvamp missing helix` succeeds when it is absent. With several names,
`present` requires **all** to be present and `missing` requires **all** to be absent.

## Previews and automation

Flags are command-specific. `--json` does not always mean a preview, and not every
command supports it.

| Command | Effect |
| --- | --- |
| `install -n` / `remove -n` | Show the package plan and command without installing or removing packages |
| `install --json` / `remove --json` | Return the plan without performing the transaction |
| `update -n` / `update --json` | Preview; may populate recipe and feed caches |
| `plan --json` | Report manifest differences without changing declarations or packages |
| `add -n` / `drop -n` | **Edit the user manifest**, then preview package changes |
| `aur build --json` | **Build** the approved recipe and report artifact paths as JSON |

`install -y` and `remove -y` refuse plans with warnings. `update -y` can skip
blocked AUR candidates and held packages while other work succeeds; its default
signed-custom-repository warning is an explicit exception. Do not interpret a
successful update exit as proof that every package was upgraded.

Consult the [client reference](/cli/pacvamp/) for each command's flags. For
manifest-only previews, use `plan`; for update results and retry actions, see
[update policy](/update-policy).
