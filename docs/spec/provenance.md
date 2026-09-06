---
description: Package provenance envelopes, signer-gate checks, and the limits of client transparency verification.
---

# Build provenance

Version 1, draft. What a repository's build host attaches to every
package it builds, and how a client checks it.

## The envelope

`<package>.provenance.json` is a [DSSE](https://github.com/secure-systems-lab/dsse)
envelope:

```json
{
  "payloadType": "application/vnd.in-toto+json",
  "payload": "<base64 statement>",
  "signatures": [{ "keyid": "5A0A0B8B9C6D7E1F", "sig": "<base64 Ed25519>" }]
}
```

The signature is a raw Ed25519 signature over the DSSE pre-authentication
encoding of the payload type and payload, made with a build key in the
minisign-compatible format. The key id is the minisign key
id. A repository lists the build keys it accepts in its index under
`build_keys`, and marks a package's `evidence.build_provenance` only when
the envelope verifies with one of them and the subject digest matches the
package file.

`.sigstore.json` is reserved for a Sigstore bundle carrying the same statement.
It is not currently verified as an alternative build-provenance format. Rekor
uploads from `attest` use the envelope plus `.rekor.json` described below.

## The statement

An in-toto Statement v1 with the SLSA v1 provenance predicate:

```json
{
  "_type": "https://in-toto.io/Statement/v1",
  "subject": [{ "name": "mise-bin-2026.9.1-1-x86_64.pkg.tar.zst", "digest": { "sha256": "..." } }],
  "predicateType": "https://slsa.dev/provenance/v1",
  "predicate": {
    "buildDefinition": {
      "buildType": "https://pacvamp.com/build/makepkg/v1",
      "externalParameters": {
        "pkgbase": "mise-bin",
        "source": "https://github.com/omacom/omarchy-pkgs",
        "commit": "..."
      },
      "resolvedDependencies": [
        { "uri": "https://github.com/jdx/mise/releases/download/v2026.9.1/mise-v2026.9.1-linux-x64.tar.xz", "digest": { "sha256": "..." } }
      ]
    },
    "runDetails": {
      "builder": { "id": "pacvamp-repo attest 5A0A0B8B9C6D7E1F" },
      "metadata": { "invocationId": "...", "finishedOn": "2026-09-03T06:00:00Z" }
    }
  }
}
```

- `externalParameters` name the PKGBUILD repository and the exact commit
  that was built.
- `resolvedDependencies` list every source artifact makepkg fetched with
  its digest. For a vendor-built package this is the vendor's release
  artifact, which is how a client chains from the OPR build to the
  vendor's packslip without downloading the artifact again.
- `builder.id` names the tool and the build key.

## Producing it

```
pacvamp-repo attest --key build.key --pkgbase mise-bin \
  --source https://github.com/omacom/omarchy-pkgs --commit <sha> \
  --dependency <uri>=<sha256> ... <package files>
```

The command reads a build-key seed file. Keep it dedicated to build provenance.
Hardware-backed custody is a deployment objective requiring a supported signing
integration; a seed file does not provide it. The separate signer host checks
the envelope before signing a package using the implemented gate below.

## Transparency

`pacvamp-repo attest --rekor <log>` uploads each envelope to a Rekor
instance as a `dsse` entry, with the build key as the verifier in SPKI
PEM, and stores the log's answer beside the package as
`<package>.rekor.json`: the entry uuid, log index, log id, integration
time, the canonical body, the inclusion proof and signed entry timestamp.
The public log is `https://rekor.sigstore.dev`. Anyone can then find every
provenance statement a build key ever signed, so a key abused on a
compromised build host leaves a public trail.

## The signer gate

The repository GPG key lives on a separate signer host. `pacvamp-repo sign`
runs there and signs a package only after:

1. the provenance envelope beside it verifies with an allowlisted build
   key, carries the SLSA provenance predicate, and names the package's
   digest as a subject;
2. with `--require-rekor`, a stored log entry exists, is a `dsse` entry
   whose payload hash is the envelope's payload, and carries an inclusion
   proof;
3. with `--index <file>`, the index lists the package with the same
   digest.

Only then does it run `gpg --detach-sign` with the repository key. Any
failure refuses the package and the command exits non-zero. This limits access
to the repository key when signer custody is actually separate; it does not
independently prove that a compromised builder's statement is truthful.
`--dry-run` reports without signing; `--json` changes output format and still
signs unless combined with `--dry-run`.

The inclusion proof is verified: the entry body is the Merkle leaf, the
proof must reach the stated root, and the checkpoint the log returns
must commit to that root and tree size. With `--rekor-pubkey <pem>` the
checkpoint's signature is verified with the log's ECDSA P-256 key too,
which is what ties the proof to the public log rather than to whatever
answered the upload. The signed entry timestamp is stored but not yet
verified.

## Consuming it

`pacvamp-repo index` verifies envelopes against the accepted build keys and
records the result. Client transactions check the supported sidecar according to
`policy.trust.provenance`; the accepted build keys travel in the authenticated
index. See [transaction enforcement](/spec/repository-feeds#transaction-enforcement).

The separate `pacvamp verify` report fetches the advertised envelope and checks
its signature and package subject. Its `.rekor.json` check binds the entry body
to the envelope's payload and requires an inclusion proof to be present. That
client check does **not** verify the Merkle path or checkpoint signature. The
signer gate's stronger proof checks must not be attributed to this report.

See [security boundaries](/security-model#publisher-verification-and-remaining-gaps)
and the [sign reference](/cli/pacvamp-repo/sign) for configuration and limits.
