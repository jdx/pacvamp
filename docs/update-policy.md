---
description: Preview updates, understand retained versions and AUR blockers, and decide when to retry or review.
---

# Updates and blocked packages

`pacvamp update` handles repository upgrades followed by AUR upgrades, with the
manifest's holds and policy. Preview before running a transaction:

```sh
pacvamp update --dry-run
pacvamp update --json
```

These previews use current local sync databases and may fetch recipes and signed
feeds into the cache. They do not refresh pacman's databases, approve commits, or
install packages. Execution refreshes databases unless disabled and reviews
candidates again, so a preview does not authorize a later build.

## Run an update

```sh
pacvamp update
```

Review the combined plan before confirming. Use `--no-aur` for repository upgrades
only, or `--aur-only` for AUR upgrades only. AUR work must run as the non-root build
user. `--wait` queues behind another update instead of failing immediately.

Configured pre/post hooks implement the surrounding distro workflow. Pacvamp does
not create filesystem backups itself. After the update, inspect orphan and pacnew
reports. Orphans are removed only when requested with `--prune-orphans`.

## Read a hold or blocker

Each hold or AUR blocker includes the installed version that remains, its reason,
and the next action. A timed blocker includes its earliest eligible UTC time,
assuming the candidate and policy stay unchanged.

| Report | Next step |
| --- | --- |
| Candidate is younger than the age floor | Retry at the reported time, provided the candidate has not changed |
| Package is explicitly held | Review the owning manifest or update ignore setting |
| Recipe or source change needs review | Run the reported review command for the exact candidate commit |
| Required evidence is unavailable or invalid | Resolve the publisher/key/feed problem and rerun the preview |
| Install scripts are denied | Approval alone cannot override the configured denial |

Waiting is not reported as sufficient when any finding still requires review or a
policy decision. Unapproved VCS recipes also require review unattended because
their sources may move independently of the recipe commit.

## Unattended results

```sh
pacvamp update -y
```

Unattended candidate warnings become blockers; blocked AUR packages and held
repository packages can remain installed while other updates proceed. A successful
exit therefore does **not** mean every package was upgraded. Keep the hold, blocker,
and skipped-package reports when integrating automation.

With the default `trust.custom_repos = "warn"`, a signed custom repository warns
and is permitted during updates. Unsigned custom repositories are denied unattended,
and configured trust requirements still apply. This exception does not mean all
warnings are bypassed, and it is distinct from install/remove warning refusal.

## Structured previews

In JSON, `holds` covers repository age floors and configured holds; `blocked`
covers unattended AUR policy. Each entry contains `name`, `reason`, `installed`,
`eligible_at` (Unix seconds or null), and `next_step`. The review command names the
exact candidate commit. Treat these as preview results, not an execution receipt.

See [configuration](/configuration) for policy and hook settings, the
[update reference](/cli/pacvamp/update) for flags, and [recovery](/recovery) if an
operation was interrupted.
