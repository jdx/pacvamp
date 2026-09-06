---
description: Stage pacvamp-repo adoption in OPR, from keys and provenance to sync gating and release promotion.
---

# Adopting pacvamp-repo in the Omarchy Package Repository

This staged integration is for OPR maintainers. It describes work in OPR's
publishing infrastructure, not an already deployed integration. The pacvamp-repo
commands are implemented; key custody, scheduling, review automation, and rollout
belong to the operator. Consult the [CLI reference](/cli/pacvamp-repo/) for flags.

Use an isolated staging repository and independently distributed trust keys before
changing client defaults. The [reference registry](/operations/registry) is a
separate deployment, not OPR.

## 1. Keys

```bash
cargo install packslip --version '=1.0.0' --locked
packslip keygen -o /etc/pacvamp-repo/index.key      # feeds, release manifests, tool index
packslip keygen -o /etc/pacvamp-repo/build.key      # the build host only
```

Publish `index.pub` through `omarchy-keyring` (clients read
`/usr/share/pacvamp/keys/*.pub`). Keep `build.key` on the build host
(the current command reads a seed file) and the GPG repository key on a separate
signer host.

## 2. The index

After every `repo-add`, run

```bash
pacvamp-repo index --repo omarchy --dir /srv/repo/omarchy/x86_64 \
  --key /etc/pacvamp-repo/index.key --build-key /etc/pacvamp-repo/build.pub
```

Clients start verifying the database digest and rollback protection at
once; evidence fields fill in as you deploy the next stages.

## 3. Provenance and the signer gate

On the build host, after `makepkg`:

```bash
pacvamp-repo attest --key /etc/pacvamp-repo/build.key --pkgbase "$pkgbase" \
  --source https://github.com/omacom/omarchy-pkgs --commit "$(git rev-parse HEAD)" \
  --dependency "$source_url=$source_sha256" *.pkg.tar.zst --rekor https://rekor.sigstore.dev
```

Supply the actual source URL/digest and repeat `--dependency` for each input.
On the signer host, instead of a bare `gpg --detach-sign`:

```bash
pacvamp-repo sign --dir /srv/repo/omarchy/x86_64 --build-key /etc/pacvamp-repo/build.pub \
  --gpg-key "$repository_fingerprint" --require-rekor --rekor-pubkey /etc/pacvamp-repo/rekor.pem
```

A package without accepted provenance is refused a signature. Provision the
trusted log public key independently. Add `--index FILE` only when the selected
index already includes the candidate package digest; otherwise publish the new
index after signing and updating the database. See [provenance](/spec/provenance)
for proof checks and the limits of builder statements.

## 4. Vendor packages

Replace the checksum-fetching `sync-upstream` step with a `vendor.toml`
per vendor package and

```bash
pacvamp-repo vendor --pkgdir pkgs/mise-bin --write
```

which rewrites the PKGBUILD from the vendor's packslip and writes the
`.vendor.json` sidecar the build ships. Vendors without a packslip keep
the repackager path (`pacvamp-repo repack`) until they publish one.

## 5. The AUR sync gate

Have repository automation invoke the gate before pulling an AUR candidate:

```bash
pacvamp-repo sync-aur --state aur-state.json --package yay --trusted-maintainer "$maintainer" \
  --verdicts /srv/repo/omarchy/x86_64/verdicts.json --key /etc/pacvamp-repo/index.key --write
```

Replace the example package and maintainer with your reviewed selections.
The command reports auto-merge, needs-review, or blocked decisions and records
eligible commits with `--write`. Your automation must perform repository edits,
open reviews, and merge PRs; the command does not operate a Git hosting service.
Humans record decisions with `pacvamp-repo verdict`; `pacvamp-repo advisories add`
publishes a block or hold. See the [sync-gate contract](/spec/sync-gate).

## 6. Snapshots

Move the mirror to a snapshot store:

```bash
pacvamp-repo snapshot --store /srv/mirror --key /etc/pacvamp-repo/index.key cut --from /srv/mirror-sync
pacvamp-repo snapshot --store /srv/mirror --key /etc/pacvamp-repo/index.key test --id SNAPSHOT_ID --suite ./omarchy-train.sh
pacvamp-repo snapshot --store /srv/mirror --key /etc/pacvamp-repo/index.key promote --channel stable
```

Serve `channels/{edge,rc,stable}` as the mirror roots. Start with a
human moving `stable`; add the QEMU suite to gate `rc`; then let the
timed soak promote.

## 7. The tool channel

For each agent CLI, a `tool.toml` and

```bash
pacvamp-repo tool-channel --store /srv/mirror --key /etc/pacvamp-repo/index.key publish --config tools/claude/tool.toml
```

on a schedule, with `promote` following the package channels.

## Verify the rollout

Set an identifiable `PACKAGER` in `makepkg.conf`. Test a missing/invalid provenance
refusal, a blocked AUR commit, a held snapshot, and client rollback against the
staging publisher. Rehearse key rotation and recovery before changing trust roots.
The [security model](/security-model) distinguishes signing roles from custody.
