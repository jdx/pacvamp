---
description: Configure AUR policy, repository trust, update hooks, and managed floors in pacvamp.toml.
---

# Configuration

Settings live in the same layered `pacvamp.toml` files as your
[package declarations](/manifests#layers-and-removal). Later ordinary layers replace
scalar settings; lists append unique values. Managed policy is applied afterwards.

## Set an AUR policy

For example, require a recipe commit to be at least three days old:

```toml
[policy.aur]
min_commit_age = "72h"
```

Ages accept units such as `h`, `d`, and `w`; `"0"` disables an age floor when
managed policy permits it. A minimum age delays eligibility; it does not establish
that a package is safe. Run `pacvamp aur review PACKAGE --unattended` to see the
policy applied to a specific recipe.

## Defaults

These are built-in defaults before distro, user, and managed configuration.
All names in this table are relative to `[policy]`.

| Setting | Default | Meaning |
| --- | --- | --- |
| `mode` | `"warn"` | Interactive findings policy; unattended checks are stricter |
| `paranoid` | `false` | Enable the strict bundle described below |
| `aur.min_commit_age` | `"48h"` | Minimum recipe commit age |
| `aur.min_package_age` | `"14d"` | Minimum age of a new AUR package |
| `aur.min_votes` | `10` | First-install reputation threshold |
| `aur.jail` | `true` | Require the filesystem and socket build jail |
| `aur.chroot` | `false` | Opt into the provisioned clean-image backend |
| `aur.chroot_root` | `"/var/lib/pacvamp/chroot/root"` | Base image path |
| `aur.cgroup_root` | Unset | Opt into a delegated cgroup v2 subtree |
| `aur.allow_network_build` | `[]` | Package names granted build network access |
| `aur.install_scripts` | `"approve"` | Install-script policy: allow, approve, or deny |
| `repo.min_release_age.arch` / `.opr` / `.custom` | `"0"` | Additional repository release-age floors |
| `repo.min_release_age_excludes` | `[]` | Package names excluded from those floors |
| `trust.index` / `trust.provenance` | `"verify"` | Verify available evidence; `"required"` makes missing evidence a policy failure |
| `trust.no_downgrade` | `true` | Prevent decreases in recorded evidence level |
| `trust.advisories` | `"on"` | Consume advisories; `"required"` tightens missing/stale-feed policy |
| `trust.custom_repos` | `"warn"` | Custom repository policy: allow, warn, or deny |

`trust.reviewers` defaults to `gate` for static, antivirus, and human reviewers,
and `warn` for AI and reproducibility reviewers. These weights consume published
verdicts; they do not run those reviewers on your machine.

See [build controls](/build-controls) for the resource-limit defaults and
[clean chroots](/clean-chroot) for image provisioning.

## Managed policy

Administrators put mandatory policy in `/etc/pacvamp/managed.toml`. An additional
file can be selected with `PACVAMP_MANAGED_CONFIG_PATH`. Keep these files and their
selection under administrative control.

```toml
[policy.aur]
min_commit_age = "48h"
jail = true
install_scripts = "deny"
deny_network_build = ["example-package"]
```

Age and reputation floors take the higher value; required protections cannot be
turned off by a user layer; stricter ranked policy wins. Managed build resource
limits are upper bounds, so users can lower them. Managed image/cgroup roots and
reviewer weights override user values. A managed network deny list removes grants
for the named packages. Managed release-age exclusions replace the user list.

To enable the strict bundle:

```toml
[policy]
paranoid = true
```

It denies policy findings, requires the jail, denies install scripts and custom
repositories, clears build-network grants, and requires index, provenance, and
advisory evidence. Ensure the intended publishers supply that evidence before
choosing this policy. `doctor` reports the effective controls.

`policy.safe` and `policy.scanner.socket_token` are unsupported and rejected.
There is no configuration switch that makes package execution a safe preview;
use the relevant [preview command](/packages#previews-and-automation).

## Updates and publishers

`[update]` accepts `ignore`, `ignore_group`, `overwrite`, `pre_hooks`, and
`post_hooks` lists. Defaults are empty. Hooks are administrator/user-supplied shell
commands outside the build jail; they can mutate the host. Configure them only
when their ordering and execution identity suit the machine.

`[channel] snapshot_base` selects the snapshot store, and `tools_base` selects the
tool-channel base. Both default to unset. Use publisher-provided URLs and keys;
see [snapshots](/snapshots) and the [tool-channel specification](/spec/tool-channel).

After editing settings, run `pacvamp doctor` and preview the intended operation.
A configuration file describes requested policy; [protection status](/protection-status)
shows what the current machine and feeds can support.
