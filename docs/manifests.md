---
description: Declare packages in layered TOML, understand add and drop, and preview convergence before applying it.
---

# Package manifests

A manifest records which packages should be present or absent. Pacvamp compares
those declarations with pacman's installed database. It does not remove every
undeclared package or pin every repository package to an exact version.

## Declare a package

```sh
pacvamp add helix
pacvamp status
```

`add` writes the user manifest and converges the named packages immediately. Other
declarations wait for `apply`. To edit declarations without installing, edit the
TOML directly, then run `plan`.

The user file is `$XDG_CONFIG_HOME/pacvamp/pacvamp.toml`, or
`~/.config/pacvamp/pacvamp.toml` when `XDG_CONFIG_HOME` is unset:

```toml
[packages]
helix = {}
google-chrome = { source = "aur" }
libreoffice-fresh = { state = "absent" }
nvidia-580xx-utils = { hold = true }
```

These are examples, not a recommended package set. An absent declaration requests
removal. A hold prevents upgrades; it does not capture an exact version. An AUR
source selects the AUR explicitly but does not approve its recipe commit.

## Preview and apply

```sh
pacvamp plan
pacvamp plan --json
pacvamp apply --dry-run
pacvamp apply
```

`plan` reports required installs and removals, including which file declared each
package. With `--detailed-exitcode`, it returns 2 for differences and 0 for no
differences; errors remain failures. `apply` performs the declared changes after
confirmation. It is not a substitute for `update`.

::: warning Declaration changes are immediate
`add --dry-run` and `drop --dry-run` still write the user manifest. They only
preview the subsequent package transaction. Use `plan` to inspect existing
declarations without editing them.
:::

## Layers and removal

Pacvamp reads these files from lowest to highest precedence:

1. `/etc/pacvamp/pacvamp.toml`
2. `/etc/pacvamp/conf.d/*.toml`, sorted by filename
3. The invoking user's `pacvamp.toml`

For a package name, the last declaration replaces the earlier declaration.
Mandatory policy is applied separately; see [configuration](/configuration).

`pacvamp drop helix` deletes the user declaration and removes the installed
package only if no lower layer still declares it present. Dropping a user choice
therefore restores the distro's default. To explicitly override a lower-layer
package with an absent declaration, use `pacvamp add --absent helix`.

Run user declarations as the intended login user. `sudo pacvamp add` selects the
root invocation's configuration rather than maintaining your ordinary user's choices.

## Manifest, lockfile, and ledger

| File | Purpose |
| --- | --- |
| `pacvamp.toml` | Desired packages and configuration |
| `pacvamp.lock`, beside the user manifest | Reviewed AUR commits and acknowledged review evidence |
| `/var/lib/pacvamp/state.json` | Root-owned transaction history and accepted package evidence |

Copying a manifest and lockfile preserves declarations and reviewed recipe commits.
It does not reproduce every installed byte or authenticate a binary already on the
second machine. [Import](/migration) previews declarations for existing packages;
[recovery](/recovery) handles interrupted ledger updates.
