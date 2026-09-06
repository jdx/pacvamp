---
description: Verify the repository key, configure pacman, and install pacvamp on a disposable x86-64 Arch machine.
---

# Install pacvamp

The proof-of-concept package targets x86-64 Arch Linux. Pacvamp is experimental,
unsupported, and not ready for real use; use a disposable machine. Other
pacman-based distributions need their own compatibility checks. Read the
[status and limitations](/project-status) before changing package configuration.

You need an existing Arch installation, network access, curl/GnuPG, and permission
to administer pacman's keyring and configuration. These commands add a repository
and perform a full system upgrade.

## 1. Verify and trust the repository key

Compare the full fingerprint with the independently published
[Pacvamp trust roots](/trust):

```bash
curl --fail --show-error --silent \
  https://repo.pacvamp.com/keys/repository.asc \
  -o pacvamp-repository.asc
gpg --show-keys --fingerprint pacvamp-repository.asc
```

The fingerprint must be:

```text
E5E3 DDD7 492A C50D 42BF EEA8 D9D6 D838 ADC4 20F3
```

Import and locally trust that key for pacman:

```bash
sudo pacman-key --init
sudo pacman-key --add pacvamp-repository.asc
sudo pacman-key --lsign-key E5E3DDD7492AC50D42BFEEA8D9D6D838ADC420F3
```

## 2. Add the repository

Add this stanza once to `/etc/pacman.conf` (edit the existing stanza if present):

```ini
[pacvamp]
SigLevel = Required DatabaseRequired
Server = https://repo.pacvamp.com/channels/stable/pacvamp/os/$arch
```

Install Pacvamp:

```bash
sudo pacman -Syu pacvamp
```

The package includes the registry index key at
`/usr/share/pacvamp/keys/pacvamp-registry.pub`. Its key ID must be
`8C2D61867298C6DC`; its complete value and checksum are on the
[trust-roots page](/trust).

## 3. Check the installation

```bash
pacvamp version
pacvamp doctor
pacvamp search pacvamp
pacvamp info pacvamp
```

These checks show the installed version and the machine's configured protections.
They do not certify every installed package. Follow [first steps](/getting-started)
to preview a transaction, or [protection status](/protection-status) if `doctor`
reports a failure. Repository signing does not change pacvamp's proof-of-concept status.
