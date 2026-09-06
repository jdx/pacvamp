---
layout: home
title: Package management for pacman systems
description: Install packages, review AUR recipes, and declare your machine’s package choices with pacvamp, a proof-of-concept pacman frontend.
---

<div class="pv-home">

<div class="pv-intro">
<div>
<p class="pv-eyebrow">pacvamp / documentation</p>
<h1>Know what you install.</h1>
<p class="pv-lead">One command line for pacman repositories and the AUR. Preview package changes, review the recipes you build, and keep a record of your choices.</p>
</div>
<img src="/logo.svg" alt="pacvamp vampire logo" width="144" height="144">
</div>

::: warning Proof of concept
pacvamp is experimental, unsupported, and not ready for real use. Try it on a disposable machine. Read the [status and limitations](/project-status) before installing.
:::

<div class="pv-start">

## Start with a preview

After [installing pacvamp](/install), inspect a package and the transaction it would require:

```sh
pacvamp search helix
pacvamp info helix
pacvamp install helix --dry-run
```

The preview shows the affected packages, their origin, and the pacman command. It does not install anything. The [first steps guide](/getting-started) walks through installation, checks, and your first package choice.

</div>

## Find your next step

<div class="pv-paths">

<div>

### Use packages

[Install and remove](/packages), [declare packages in a manifest](/manifests), or [import an existing machine](/migration). Learn when a command changes your declarations and when it changes installed packages.

</div>
<div>

### Review and update

[Review an AUR recipe](/aur), [understand a blocked update](/update-policy), or [check active protections](/protection-status). Policy and evidence depend on your configuration and publisher.

</div>
<div>

### Publish and integrate

[Run a registry](/operations/registry) or follow the [OPR](/adoption/opr), [Omarchy](/adoption/omarchy), and [mise](/adoption/mise) adoption guides. These are separate operator workflows.

</div>

</div>

## How pacvamp fits

pacvamp reads pacman’s databases and delegates package transactions to pacman. It adds manifests, commit-bound AUR approval, build confinement, and checks for signed repository evidence. It does not make arbitrary packages safe, and a repository name alone does not establish provenance.

User-scoped development tools stay with mise. Repositories can publish verified vendor releases through the [tool channel](/spec/tool-channel); [packslip](/spec/packslip) supplies the external signed release format.

For command details, use the [client reference](/cli/pacvamp/) or [repository reference](/cli/pacvamp-repo/). To work on pacvamp, start with [development](/development) and [architecture](/architecture). The [roadmap](https://github.com/jdx/pacvamp/blob/main/PLAN.md) tracks remaining work.

</div>
