---
description: Deploy and inspect the independent proof-of-concept pacvamp registry using the repository’s existing workflow and services.
---

# Run the reference registry

The deployment in this repository serves the independent proof-of-concept
repository at `repo.pacvamp.com`. It does not share keys or storage with OPR.
This guide is for operators of that deployment; package users should start with
[installation](/install).

The scripts and Caddy configuration target that hostname and layout. To operate a
separate registry, adapt the hostname, package/key configuration, and workflow to
your own infrastructure; do not reuse the public registry's identity or trust roots.

## Layout and publishing flow

```text
/srv/pacvamp/
  sync/pacvamp/os/x86_64/    mutable publishing workspace
  store/
    snapshots/<id>/        fixed package sets and signed release manifests
    channels/              edge, rc, stable pointers
    keys/                  public keys served by Caddy
```

The `publish` script builds the pacvamp package and the canary, writes provenance,
gates package signatures, updates the signed pacman database, and writes a signed
index. The `snapshot` script cuts a release, runs the consistency suite, and advances
edge/rc. The installer then explicitly promotes rc to stable on each deployment.
This is consistency-tested publishing, not an Omarchy desktop acceptance suite.

## Prepare the host and workflow

Use a dedicated Arch Linux VM with storage mounted at `/srv/pacvamp`. The reference
scripts assume systemd, a dedicated `pacvamp-registry` account, and SSH access as
root or an account with noninteractive sudo. The installer creates the account,
installs dependencies, and upgrades the host's packages.

Configure a GitHub environment named `registry`:

| Kind | Name | Value |
| --- | --- | --- |
| Variable | `REGISTRY_SSH_HOST` | Registry VM hostname or address |
| Variable | `REGISTRY_SSH_PORT` | SSH port; default 22 |
| Variable | `REGISTRY_SSH_USER` | Deployment account |
| Secret | `REGISTRY_SSH_PRIVATE_KEY` | Deployment SSH key |
| Secret | `REGISTRY_SSH_KNOWN_HOSTS` | Independently verified SSH host-key line |

For the reference hostname, configure DNS and allow ports 80/443 to reach Caddy.
Its configuration handles TLS. Use pinned SSH host verification for deployment.

## Deploy a selected revision

Run the **registry deploy** workflow manually with `deploy_ref` set to the branch,
tag, or commit you intend to deploy. Its default is `registry-deployment`.
The workflow is not automatically triggered by a successful CI run; choose a
revision whose validation you have reviewed.

The workflow resolves an immutable Git commit, builds pacvamp and pacvamp-repo from
it, installs the separately pinned packslip CLI, and constructs the deployment
bundle. It sends the bundle over SSH and runs `deploy/registry/bin/install` as root.

The installer preserves existing keys, updates the deployment configuration and
package inputs, runs publish/snapshot, promotes the current rc snapshot to stable,
and enables Caddy and the daily snapshot timer. Publishing skips package filenames
already present; subsequent deployments still invoke publishing and promotion.
A failure should be investigated before retrying, especially after partial publish.

## Signing custody

| Identity | Role | Reference location |
| --- | --- | --- |
| Feed key | Signed index, release manifests, tool index | `/etc/pacvamp-registry/index.key` |
| Build key | Package provenance envelopes | `/etc/pacvamp-registry/build.key` |
| OpenPGP key | Packages and pacman databases | `/var/lib/pacvamp-registry/gnupg` |

The installer generates missing keys on the host and does not send private keys
to GitHub. **All three identities currently live on one host.** A compromised host
therefore compromises this custody boundary; the signer gate alone does not fix it.

Separate signer custody, hardware-backed integration, rotation, and recovery remain
[roadmap work](https://github.com/jdx/pacvamp/blob/main/PLAN.md#registry-operations).
Publish your public fingerprints through an independent trust channel. The
[public trust-roots page](/trust) applies to the reference registry only.

## Inspect and recover a deployment

Inspect services and their retained logs on the registry host:

```sh
sudo systemctl status pacvamp-registry-publish.service pacvamp-registry-snapshot.service
sudo journalctl -u pacvamp-registry-publish.service -u pacvamp-registry-snapshot.service
sudo systemctl list-timers pacvamp-registry-snapshot.timer
```

The generated configuration is `/etc/pacvamp-registry/registry.env`. It records
paths, the repository fingerprint, and the source commit. Do not replace it with
the example file without supplying every field required by the current publisher.

After resolving a failure, rerun the affected services deliberately:

```sh
sudo systemctl start pacvamp-registry-publish.service
sudo systemctl start pacvamp-registry-snapshot.service
sudo -u pacvamp-registry pacvamp-repo snapshot \
  --store /srv/pacvamp/store --key /etc/pacvamp-registry/index.key status
```

Inspect the reported release before manually promoting a chosen `SNAPSHOT_ID`:

```sh
sudo -u pacvamp-registry pacvamp-repo snapshot \
  --store /srv/pacvamp/store --key /etc/pacvamp-registry/index.key \
  promote --channel stable --id SNAPSHOT_ID
```

Use [snapshot hold/promotion controls](/spec/snapshot-store) when a release should
be withdrawn. Do not delete keys or rewrite published package bytes to resolve a
failed deploy. The [provenance spec](/spec/provenance) describes signer refusal.

## Validate as a client

Use a disposable Arch VM and follow the [installation guide](/install). Install
the pacvamp package and canary, then inspect policy and repository metadata:

```sh
pacvamp doctor
pacvamp info pacvamp
pacvamp info pacvamp-registry-canary
```

The **registry smoke** workflow can be dispatched manually and runs on pull
requests affecting its workflow or registry deployment files. It is not automatically
triggered by every deployment. Run it as a separate acceptance step and retain its
results with the deployed revision. See [development](/development) for the broader
fixture, container, and VM tests.
