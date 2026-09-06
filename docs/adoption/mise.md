---
description: Coordinate external mise adoption of packslip publishing, verification, and vetted tool channels.
---

# Adopting packslip and the tool channel in mise

These are mise changes, outside this repository; they are listed so the
two projects move in step.

## 1. Publish a packslip from mise's own release workflow

Use packslip's upstream [publishing workflow](https://github.com/jdx/packslip#add-it-to-a-github-release)
after release artifacts are built and uploaded. Publish the resulting
`packslip.sigstore.json` bundle and a signed release list when using list-based
discovery. Pin the key or workflow identity consumers are expected to verify.

For the package publisher, see the [vendor pipeline](/spec/vendor-pipeline).
For the tool channel, supply the explicit vendor public key that its current
consumer requires; keyless package generation does not imply keyless tool-channel
support.

## 2. Verify packslips in the `github` and `http` backends

When a release carries a packslip and the registry entry (or tool
option) pins the vendor key, verify the document before trusting a
checksum, record `provenance = "packslip"` and the evidence level in
`mise.lock`, and apply no-downgrade on the level. The `packslip` crate
from crates.io provides verification and artifact selection.

## 3. The tool channel

As an integration step, register the included `tool-channel` backend plugin so
users can install it by name. Until then, follow the
[plugin installation instructions](https://github.com/jdx/pacvamp/tree/main/plugins/mise-tool-channel). Later,
a setting listing channel URLs consulted before the registry for any
tool the channel vets, with a paranoid rule that refuses unvetted
versions of vetted tools, retires the plugin.

## 4. On Omarchy

Have the `pacman` and `aur` bootstrap managers delegate to `pacvamp
install` and `pacvamp install --aur`, so mise's "my machine" config keeps
working and gains pacvamp's guarantees. Stop forcing a zero minimum
release age in the Omarchy update step once the tool channel covers the
tools it was for.

## Validate the integration

Test a valid release, a changed signer, a rolled-back list, a held tool version,
and an unavailable required feed. Confirm that tools remain per-user and versioned.
Native backend support and registry enrollment must be verified in mise itself;
this repository's [tool-channel format](/spec/tool-channel) is the shared contract.
