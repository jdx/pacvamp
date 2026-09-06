---
description: Independently compare the public pacvamp registry’s package-signing and index-signing identities.
---

# Pacvamp trust roots

Verify these values on `pacvamp.com` before trusting keys downloaded from
`repo.pacvamp.com`. A key served by the registry cannot authenticate itself.

These are the trust roots for the public proof-of-concept registry. Operators must publish key
rotations here before using new keys. A new independent registry needs its own
keys and independent publication channel.

## Package repository key

Pacman packages and repository databases are signed by this OpenPGP key:

| Field | Value |
| --- | --- |
| Identity | `Pacvamp Registry <registry@pacvamp.com>` |
| Full fingerprint | `E5E3 DDD7 492A C50D 42BF EEA8 D9D6 D838 ADC4 20F3` |
| Long key ID | `D9D6D838ADC420F3` |

[Download the armored public key](https://repo.pacvamp.com/keys/repository.asc),
then verify the full fingerprint:

```bash
curl --fail --show-error --silent \
  https://repo.pacvamp.com/keys/repository.asc \
  -o pacvamp-repository.asc
gpg --show-keys --fingerprint pacvamp-repository.asc
```

Do not rely on the short or long key ID alone. The command must display the
full fingerprint above before you import the key into pacman's keyring.

## Registry index key

Pacvamp indexes are signed with this minisign-compatible Ed25519 public key:

| Field | Value |
| --- | --- |
| Key ID | `8C2D61867298C6DC` |
| Public-key file SHA-256 | `963e54974176571313edafd82d3ec40926e514b05cfdb29a885e95466d8abfa1` |

The complete public key is:

```text
untrusted comment: minisign public key 8C2D61867298C6DC
RWTcxphyhmEtjJyh9Gx3khORBdOW1va9iTuPrQN+U1kXBmUFz3hglCwb
```

[Download the index public key](https://repo.pacvamp.com/keys/index.pub), then
verify its checksum:

```bash
curl --fail --show-error --silent \
  https://repo.pacvamp.com/keys/index.pub \
  -o pacvamp-index.pub
printf '%s  %s\n' \
  '963e54974176571313edafd82d3ec40926e514b05cfdb29a885e95466d8abfa1' \
  pacvamp-index.pub | sha256sum --check --strict
```

The package repository key and registry index key are separate trust roots.
The build key recorded inside a signed index is evidence authenticated by that
index; clients do not need to configure it as a root of trust.

After comparing the values, continue with [installation](/install). For the role
of each key, see [registry custody](/operations/registry#signing-custody); for
client evidence checks, see the [security model](/security-model).
