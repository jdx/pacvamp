---
description: How pacvamp uses the external packslip v1 release format and where repository policy begins.
---

# Packslip

Pacvamp consumes [packslip 1.x](https://packslip.dev/release/v1/) from crates.io.
The manifest format and verifier are maintained in [jdx/packslip](https://github.com/jdx/packslip);
this repository does not ship the packslip CLI.

A release is one signed `packslip.sigstore.json` bundle. The verifier checks
the signature, signer policy, document validity, and any supplied artifact
files. Pacvamp adds repository policy: evidence floors, minimum release age,
and persistent rollback protection.

See the [vendor pipeline](vendor-pipeline.md) for package declarations,
signed release lists, monorepos, and repackager attestations, and the
[tool channel](tool-channel.md) for publishing verified tools to mise.

## Separate CLI

Install the separate CLI with `cargo install packslip --version '=1.0.0' --locked`.
Its [CLI reference](https://packslip.dev/cli/) and
[publishing guide](https://packslip.dev/docs/publishing/) are upstream.

For the source of the upstream format, see the
[packslip specification](https://github.com/jdx/packslip/blob/main/docs/spec/packslip.md).
Pacvamp's [architecture](/architecture#publishing-and-adoption) describes the
client and publisher boundaries.
