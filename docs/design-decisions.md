---
description: The durable principles and tradeoffs behind pacvamp’s package, policy, evidence, and publishing boundaries.
---

# Design decisions

These decisions explain the current design and constrain future work. They were
established in the original September 2026 design and are maintained here as the
implementation evolves. They are not claims that every adopting distro has deployed
the design. Current limitations are listed in [project status](/project-status);
unresolved work belongs in the [roadmap](https://github.com/jdx/pacvamp/blob/main/PLAN.md).

## Keep pacman compatible

Pacman performs transactions; native readers provide fast queries without linking
libalpm. The local database format remains compatible with pacman so existing Arch
tools and pacman itself remain usable. A future native engine must keep this
compatibility, including transaction semantics, hooks, and scriptlets.

The database crate uses small direct parsers for pacman configuration, database
entries, package metadata, and version comparison. This keeps dependencies limited
and allows comparison against pacman's own version behavior. Reconsider external
ALPM libraries when implementing a native resolver or installer.

## Separate declared intent from recorded fact

A manifest expresses what the user or distro wants. A lockfile records reviewed
AUR commits. The ledger records what actually completed and what evidence was
accepted. Importing a package name or matching an installed version cannot manufacture
provenance. Interrupted bookkeeping requires explicit recovery.

## Make source choices explicit

Repository order and origin remain visible. The AUR is an explicit source, not a
silent fallback after a repository miss. A repository label identifies origin;
verification establishes what evidence is available for that package.

## Bind review to a commit

An AUR approval belongs to a pkgbase and a Git commit. Build the exported approved
object and review again when the recipe changes. Separate source verification from
building so recipe code in the fetching phase cannot plant later build inputs.
A pinned recipe is not necessarily a pinned upstream VCS source or a reproducible build.

## Make automation stricter

An unattended run cannot interpret a warning on the user's behalf. Candidate
findings that an interactive user can review become blockers in automation.
Updates report and skip held/blocked packages rather than implying everything
advanced. The signed-custom-repository warning exception is documented explicitly
in [update policy](/update-policy).

## Respect an administrator's policy floor

Ordinary configuration may tighten mandatory ages, signature requirements, and
build controls, but cannot disable protections the managed layer requires.
Resource budgets reverse the comparison: a lower maximum is stricter.
[Configuration](/configuration) documents the actual merge rules and exceptions.

## Treat findings as risk signals

The policy engine flags recipe changes and known risk patterns; it does not claim
to detect all malware. A signature identifies an authenticated statement, not benign
behavior. Published reviewer verdicts use a common subject/reviewer/result shape,
so static analysis, human review, and other producers can integrate independently.
The client does not run an external reviewer just because its kind has a policy weight.

AI reviewer verdicts default to warnings. Moving a reviewer to a blocking role
requires evidence about its false-positive rate and operational consequences,
not simply adding another producer.

## Separate signing roles and custody

Use an Ed25519/minisign-format feed key independently from pacman's OpenPGP key,
so feed verification and key rotation do not depend on the package-signing identity.
Build keys sign provenance; the signer gate verifies it before using the package key.

Separate keys on one host do not isolate compromise. The target deployment separates
the repository signer from the build host and uses hardware-backed custody where
supported, with rehearsed rotation and recovery. The proof-of-concept registry
currently uses single-host custody. Hardware key support must be implemented and
validated before it can be described as active protection.

Public transparency makes submitted statements inspectable; its value depends on
which proofs and log signatures the consumer actually verifies. The
[provenance spec](/spec/provenance) distinguishes the client and signer paths.

## Keep custom repositories usable and visible

Users deliberately configure third-party repositories. The default policy warns
about signed custom repositories, while unsigned repositories are denied unattended.
A distro floor or paranoid policy can deny custom repositories entirely. Do not
silently attribute OPR's evidence to another repository.

## Prefer tested releases over an arbitrary mirror delay

Immutable snapshots make promotion and rollback explicit. Channel pointers separate
publishing, testing, and release. The operator defaults are a three-day stable soak,
90-day retention, and 365-day retention for snapshots that reached stable.
Operators can configure those values; they are not promises about a live mirror.

A suite reports what it checked. A consistency test, boot fixture, and desktop
hardware matrix provide different evidence. Labels must not imply wider coverage
than the reported suite supports.

## Choose the installer by scope

System packages go through pacvamp; per-user versioned tools stay with mise.
Vendor-built system software still needs a package when it integrates with boot,
privileged services, drivers, or system desktop files. A user tool can instead be
mirrored with its evidence in a [tool channel](/spec/tool-channel).

Packslip is the vendor-neutral release format shared by those paths. Its format,
verifier, and CLI live in the separate packslip project. Pacvamp owns repository
policy and evidence levels, not the upstream format.

The included mise backend plugin bridges tool channels until native integration
is adopted. Keeping the publisher format independent lets another distro or a
company operate a compatible channel.

## Keep the implementation boundaries small

Use usage-rs for command definitions, generated documentation, and completions;
use ratatui for in-process interactive views. Repository operators invoke
pacvamp-repo commands and retain ownership of their build/review infrastructure.
A GUI store, flatpak/snap integration, pacman's entire flag surface, and a built-in
AI reviewer are outside the initial design.
