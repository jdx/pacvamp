---
description: Build pacvamp, run fixture and Arch acceptance tests, and regenerate the documentation website.
---

# Development

The workspace contains the pacvamp client, pacvamp-repo publisher, shared CLI
support, policy engine, and native pacman database readers. Read the
[architecture](/architecture) before changing a cross-cutting workflow.

## Prerequisites

Install Rust at or above the workspace's `rust-version` in `Cargo.toml`, plus mise.
Run `mise install` from the checkout for the pinned task tools. Compilation uses
the repository's mr-boxington cargo wrapper. AUR and kernel acceptance need Linux;
the Arch E2E task also requires Docker.

```sh
mise install
mise run build
mise run test:unit
mise run lint
```

`test:unit` runs the workspace's Rust unit and integration suites. Integration
fixtures use fake commands on a temporary PATH, local databases, HTTP servers,
and Git repositories. They do not install packages on the host. The mise backend
plugin test runs when mise is on PATH.

## Acceptance tests

```sh
mise run test:e2e
mise run test
```

`test:e2e` builds the workspace and runs real pacman/makepkg transactions in Arch
containers. `test` includes both the Rust suites and that mandatory E2E task.
Missing Docker or required container capabilities fails instead of silently skipping.

CI also runs the full Rust suite on Arch, mandatory kernel enforcement tests,
and a QEMU Arch lifecycle test across two boots. The VM test runs separately from
`mise run test`; its requirements and retained logs are described in the
[VM harness](https://github.com/jdx/pacvamp/tree/main/harness/vm).
The fixture feature `test-pacman` explicitly selects fake pacman where real
`/usr/bin/pacman` would otherwise be used.

See [security acceptance](/security-testing) for adversarial boundaries and
[search benchmarks](https://github.com/jdx/pacvamp/tree/main/benchmarks) for the
reproducible corpus and performance thresholds. `mise run ci` collects the local
lint, build, test, render, and docs tasks; CI's VM and host setup remain separate.

## Edit documentation

Handwritten guides and specifications live under `docs`. Command help comes from
Rust `usage-rs` declarations; `docs/cli` and its KDL files are generated outputs.
Edit the Rust description, then regenerate:

```sh
mise run render
mise run docs:dev
```

The docs development task regenerates the CLI reference, installs website
dependencies, and starts VitePress. For subsequent prose/style edits, use
`bun run docs:dev` to start VitePress without rebuilding the Rust workspace.

Before submitting:

```sh
mise run docs:build
mise run render:check
```

Include the generated reference in the change. `render:check` compares it with Git,
so it expects the generated changes to be recorded in the submitted commit.
`bun run docs:build` builds the current Markdown directly, including the social-image
tests and output checks, without regenerating command help.

Keep existing URLs when reorganizing guides. Update navigation and contextual
links when adding a page. The README introduces the project; the website documents
current behavior; [PLAN.md](https://github.com/jdx/pacvamp/blob/main/PLAN.md) tracks
future work. Protocol specs remain draft while the project is experimental.

## Submit changes

Use Conventional Commit titles such as `docs: clarify aur approval`. Open pull
requests ready for review, never as drafts. Explain behavior changes and relevant
validation; update the affected guide or specification with implementation changes.
The [design decisions](/design-decisions) record durable constraints, not a list of
historical implementation tasks.
