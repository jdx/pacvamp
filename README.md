<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/wordmark-dark.svg">
    <img alt="pacvamp" src="assets/wordmark.svg" width="420">
  </picture>
</p>

pacvamp is a pacman frontend for installing, removing, and updating packages from
distribution repositories and the AUR. It adds transaction previews, package
manifests, commit-bound AUR review, confined builds, and verification of signed
repository evidence. Pacman still performs package transactions.

> [!WARNING]
> pacvamp is a proof of concept: experimental, unsupported, and not ready for real
> use. Try it on a disposable machine. See the [status and limitations](https://pacvamp.com/project-status).

## Get started

Follow the [installation guide](https://pacvamp.com/install) to verify the registry
key and install the signed Arch package. Then try a few commands:

```sh
pacvamp doctor                    # inspect policy and available protections
pacvamp search helix              # search configured repository databases
pacvamp install helix --dry-run   # preview packages and the pacman command
pacvamp install helix             # install after reviewing the plan
```

To record a lasting package choice, use `pacvamp add helix`: it writes your user
manifest **and installs the package**. `pacvamp plan` previews changes needed to
satisfy all declarations; `pacvamp apply` performs them. See
[first steps](https://pacvamp.com/getting-started) and
[manifests](https://pacvamp.com/manifests).

AUR packages require an explicit choice, such as `pacvamp install --aur PACKAGE`.
Read the [AUR guide](https://pacvamp.com/aur) before building. Approval applies to a
recipe commit; it does not certify that the resulting package is safe.

## Documentation

- [Package operations](https://pacvamp.com/packages), [configuration](https://pacvamp.com/configuration), and [updates](https://pacvamp.com/update-policy)
- [Client CLI](https://pacvamp.com/cli/pacvamp/) and [repository CLI](https://pacvamp.com/cli/pacvamp-repo/)
- [Architecture](docs/architecture.md), [design decisions](docs/design-decisions.md), and [roadmap](PLAN.md)

The repository also includes `pacvamp-repo` for publishing signed feeds,
provenance, snapshots, and vendor packages, plus a mise tool-channel backend
plugin. Start with the [registry guide](https://pacvamp.com/operations/registry)
or the [adoption guides](https://pacvamp.com/adoption/opr).
[Packslip](https://pacvamp.com/spec/packslip) is an external crate and CLI used for
signed vendor releases; it is maintained separately.

## Develop

Install Rust and [mise](https://mise.jdx.dev), then:

```sh
mise install
mise run build        # build the workspace
mise run test:unit    # Rust unit and integration tests
mise run test         # also run mandatory Arch E2E tests; requires Docker
mise run lint         # formatting and static checks
mise run docs:dev     # regenerate CLI docs and start VitePress
```

See [development](docs/development.md) for prerequisites, documentation generation,
and the distinction between fixture, Arch-container, and VM acceptance tests.
