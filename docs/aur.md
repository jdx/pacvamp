---
description: Review an AUR recipe, approve its exact commit, and build or install it with pacvamp.
---

# Review and build AUR packages

An AUR package is a build recipe supplied by its maintainer. Pacvamp evaluates
that recipe, records approval for an exact Git commit, and runs makepkg under
configured confinement. Review and confinement reduce exposure; they do not
certify that the installed package is safe.

Use a non-root account on a disposable Arch machine with Git, makepkg, and the
required build tools. Check `pacvamp doctor` first. The default jail requires
working Landlock and seccomp enforcement; unavailable enforcement fails the build.

## Find and review

```sh
pacvamp search --aur google-chrome
pacvamp aur review google-chrome
```

Review fetches metadata and Git history, shows the PKGBUILD or its diff from the
last approved commit, and reports findings and published verdicts. Nothing is built.
Read the recipe, source URLs, checksums, and install scripts, not just the summary.

To inspect one candidate and the policy automation would apply, replace
`COMMIT_SHA` with the full commit shown in the review:

```sh
pacvamp aur review google-chrome --commit COMMIT_SHA --unattended
```

## Approve a commit

```sh
pacvamp aur approve google-chrome --commit COMMIT_SHA
```

This reviews the selected commit and asks for confirmation before writing approval
in `pacvamp.lock`. A new recipe commit needs a new review. Approval does not override
an explicit managed install-script denial, and a VCS recipe may fetch source content
that is not pinned by the recipe commit.

## Build or install

To build the approved commit without installing its output:

```sh
pacvamp aur build google-chrome
```

The command reports the artifact paths and retains a [local receipt](/build-receipts).
It may need to install missing repository build dependencies on the host. For
build dependencies isolated from the host package database, provision a
[clean image](/clean-chroot) and use its disposable-image workflow.

For the combined review, approval, build, and installation flow:

```sh
pacvamp install --aur google-chrome
```

This checks the current candidate again; earlier review does not silently authorize
a newer commit. `pacvamp add --aur google-chrome` also records a manifest declaration.

## Understand the build boundary

Source fetching and verification run in their own confined workspace with network
access. Building uses verified sources in a separate private run directory, with
network denied unless configuration grants it. A network grant is a package-name
list under `[policy.aur] allow_network_build`; it is not a commit-scoped approval.
Managed policy may deny that grant.

All makepkg phases run under supervision and [resource limits](/build-controls).
The default filesystem/socket jail does not provide a complete process or
Unix-socket namespace. The optional [clean-chroot backend](/clean-chroot) adds
namespace isolation. Package installation itself is performed by pacman outside
the build jail.

## When a build or update is blocked

Read the finding and its requested action. Age floors may resolve with time;
changed sources, commit drift, or install-script policy need review or a deliberate
configuration decision. `-y` does not dismiss these findings. See
[blocked updates](/update-policy) for retained versions and retry times.

Keep failed build logs and receipts while investigating. Use [cache status](/cache)
before cleanup, and consult the [AUR command reference](/cli/pacvamp/aur) for receipt
comparison, replay, and advanced build options.
