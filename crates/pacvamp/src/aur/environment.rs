//! Explicit devtools provisioning and disposable dependency environments.
use crate::engine::sudo::{Context, Invocation};
use eyre::{Result, bail};
use std::path::{Path, PathBuf};

pub fn privileged(program: &str, args: Vec<String>) -> Result<()> {
    if !matches!(program, "cp" | "rm" | "mkarchroot" | "arch-nspawn") {
        bail!("unsupported privileged image tool: {program}");
    }
    let invocation = system_invocation(program, args, false)?;
    eprintln!("{}", invocation.display());
    use std::os::fd::AsFd as _;
    if !invocation
        .command()
        .env("PATH", "/usr/local/sbin:/usr/local/bin:/usr/bin")
        .stdout(std::process::Stdio::from(
            std::io::stderr().as_fd().try_clone_to_owned()?,
        ))
        .status()?
        .success()
    {
        bail!("{program} failed");
    }
    Ok(())
}
fn system_invocation(program: &str, args: Vec<String>, noninteractive: bool) -> Result<Invocation> {
    // These are Arch system tools. Never elevate a caller's PATH override.
    let mut context = Context::detect(Default::default());
    context.sudo = Some(PathBuf::from("/usr/bin/sudo"));
    context.interactive &= !noninteractive;
    Ok(Invocation::new(Path::new("/usr/bin").join(program), args).elevated(&context)?)
}

fn cleanup_args(directory: &Path) -> Result<Vec<String>> {
    use std::os::unix::fs::MetadataExt as _;
    let metadata = std::fs::symlink_metadata(directory)?;
    // Anchor the privileged process to the original private directory before
    // any recipe runs. Renames and symlink swaps cannot redirect later cleanup.
    let script = r#"set -euo pipefail
cd -- "$1"
test "$(/usr/bin/stat -c '%d:%i' .)" = "$2"
printf 'ready\n'
while IFS= read -r ignored; do :; done
exec /usr/bin/rm -rf -- root
"#;
    Ok(vec![
        "--noprofile".into(),
        "--norc".into(),
        "-p".into(),
        "-c".into(),
        script.into(),
        "pacvamp-image-cleanup".into(),
        arg(directory)?,
        format!("{}:{}", metadata.dev(), metadata.ino()),
    ])
}

fn arg(path: &Path) -> Result<String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| eyre::eyre!("image paths must be UTF-8"))
}
pub fn new_root(path: &Path) -> Result<()> {
    match path.symlink_metadata() {
        Ok(_) => bail!("image destination already exists"),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }
    if !path.is_absolute() || path == Path::new("/") {
        bail!("image destination must be a new absolute path other than /");
    }
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        bail!("image destination cannot contain ..");
    }
    Ok(())
}
pub fn initialize(destination: &Path, packages: &[String]) -> Result<()> {
    new_root(destination)?;
    validate_packages(packages)?;
    let mut args = vec![
        "-C".into(),
        "/etc/pacman.conf".into(),
        arg(destination)?,
        "base-devel".into(),
    ];
    args.extend_from_slice(packages);
    privileged("mkarchroot", args)?;
    super::chroot::host(destination)?;
    Ok(())
}
pub fn clone_image(root: &Path, destination: &Path) -> Result<()> {
    super::chroot::host(root)?;
    new_root(destination)?;
    privileged(
        "cp",
        vec![
            "-a".into(),
            "--reflink=auto".into(),
            "--".into(),
            arg(&root.canonicalize()?)?,
            arg(destination)?,
        ],
    )
}
pub fn validate_packages(packages: &[String]) -> Result<()> {
    for package in packages {
        if package.starts_with('-')
            || package.is_empty()
            || !package
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"@._+-/".contains(&c))
        {
            bail!("invalid repository package target {package:?}");
        }
    }
    Ok(())
}
pub fn update(root: &Path, packages: &[String], yes: bool) -> Result<()> {
    super::chroot::host(root)?;
    validate_packages(packages)?;
    let mut args = vec![
        arg(root)?,
        "pacman".into(),
        "-Syu".into(),
        "--needed".into(),
    ];
    if yes {
        args.push("--noconfirm".into());
    }
    args.push("--".into());
    args.extend_from_slice(packages);
    privileged("arch-nspawn", args)
}

pub struct Disposable {
    pub root: PathBuf,
    _directory: tempfile::TempDir,
    cleanup: Option<std::process::Child>,
}
impl Disposable {
    pub fn prepare(
        root: &Path,
        packages: &[String],
        artifacts: &[PathBuf],
        yes: bool,
    ) -> Result<Self> {
        validate_packages(packages)?;
        // Copy and verify artifacts before asking root to consume them.
        // Arch normally mounts /tmp on tmpfs. Keep potentially multi-gigabyte
        // clones on the user's cache volume, independently of TMPDIR.
        let images = super::cache_dir().join(".pacvamp-images");
        std::fs::create_dir_all(&images)?;
        let directory = tempfile::Builder::new()
            .prefix("pacvamp-image-")
            .tempdir_in(&images)?;
        let artifacts_dir = directory.path().join("artifacts");
        std::fs::create_dir(&artifacts_dir)?;
        let mut copies = Vec::new();
        for (i, artifact) in artifacts.iter().enumerate() {
            let (receipt, _) = super::receipt::for_artifact(artifact)?;
            let name = artifact
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| eyre::eyre!("invalid artifact name"))?;
            let copy = artifacts_dir.join(format!("dependency-{i}.pkg.tar.zst"));
            std::fs::copy(artifact, &copy)?;
            if receipt.outputs.get(name) != Some(&packslip::digest_file(&copy)?.0) {
                bail!("dependency artifact changed while copying");
            }
            copies.push(copy);
        }
        let mut image = Self {
            root: directory.path().join("root"),
            _directory: directory,
            cleanup: None,
        };
        clone_image(root, &image.root)?;
        if !packages.is_empty() {
            update(&image.root, packages, yes)?;
        }
        if !copies.is_empty() {
            // arch-nspawn exposes the private artifact directory read-only.
            let mut args = vec![
                arg(&image.root)?,
                format!(
                    "--bind-ro={}:{}",
                    arg(&artifacts_dir)?,
                    "/pacvamp-dependencies"
                ),
                "pacman".into(),
                "-U".into(),
            ];
            if yes {
                args.push("--noconfirm".into());
            }
            args.push("--".into());
            for copy in copies {
                args.push(format!(
                    "/pacvamp-dependencies/{}",
                    copy.file_name().unwrap().to_string_lossy()
                ));
            }
            privileged("arch-nspawn", args)?;
        }
        super::chroot::host(&image.root)?;
        image.arm_cleanup()?;
        Ok(image)
    }
    fn arm_cleanup(&mut self) -> Result<()> {
        use std::{io::BufRead as _, os::unix::process::CommandExt as _, process::Stdio};
        // Authorize cleanup now, not in Drop after a potentially long build.
        // The stdin pipe is CLOEXEC and the process group is separate, so a
        // recipe cannot retain it and terminal cancellation cannot kill it.
        let context = Context::detect(Default::default());
        // Any password prompt must happen in the foreground, before starting
        // the background cleaner. Batch mode instead fails before the recipe.
        if !context.is_root
            && context.interactive
            && !std::process::Command::new("/usr/bin/sudo")
                .arg("-v")
                .status()?
                .success()
        {
            bail!("could not authorize disposable image cleanup");
        }
        self.cleanup = Some(
            system_invocation("bash", cleanup_args(self._directory.path())?, true)?
                .command()
                .env("PATH", "/usr/bin")
                .process_group(0)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?,
        );
        let child = self.cleanup.as_mut().unwrap();
        let mut ready = String::new();
        std::io::BufReader::new(child.stdout.take().unwrap()).read_line(&mut ready)?;
        if ready != "ready\n" {
            bail!("privileged image cleanup failed to start");
        }
        Ok(())
    }
}
impl Drop for Disposable {
    fn drop(&mut self) {
        if let Some(mut cleanup) = self.cleanup.take() {
            drop(cleanup.stdin.take());
            if matches!(cleanup.wait(), Ok(status) if status.success()) {
                return;
            }
            eprintln!(
                "privileged cleanup failed for disposable image {}",
                self.root.display()
            );
            // Never retry via a pathname after an anchored cleaner failed.
            return;
        }
        // Provisioning failed before the cleaner was armed. Best-effort cleanup
        // may need fresh authorization; successful normal cleanup never does.
        if self.root.exists()
            && let Err(err) = privileged(
                "rm",
                vec![
                    "-rf".into(),
                    "--".into(),
                    self.root.to_string_lossy().into_owned(),
                ],
            )
        {
            eprintln!(
                "could not remove disposable image {}: {err:#}",
                self.root.display()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cleanup_refuses_a_replaced_directory_before_startup() {
        use std::{
            fs,
            process::{Command, Stdio},
        };
        let outer = tempfile::tempdir().unwrap();
        let private = outer.path().join("private");
        let outside = outer.path().join("outside");
        fs::create_dir_all(&private).unwrap();
        fs::create_dir_all(outside.join("root")).unwrap();
        fs::write(outside.join("root/keep"), b"untouched").unwrap();
        let args = super::cleanup_args(&private).unwrap();
        fs::rename(&private, outer.path().join("moved")).unwrap();
        std::os::unix::fs::symlink(&outside, &private).unwrap();
        assert!(
            !Command::new("/usr/bin/bash")
                .args(args)
                .stdin(Stdio::null())
                .status()
                .unwrap()
                .success()
        );
        assert_eq!(fs::read(outside.join("root/keep")).unwrap(), b"untouched");
    }
    #[test]
    fn cleanup_stays_anchored_after_parent_path_replacement() {
        use std::{
            fs,
            io::BufRead as _,
            process::{Command, Stdio},
        };
        let outer = tempfile::tempdir().unwrap();
        let private = outer.path().join("private");
        let moved = outer.path().join("moved");
        let outside = outer.path().join("outside");
        fs::create_dir_all(private.join("root")).unwrap();
        fs::create_dir_all(outside.join("root")).unwrap();
        fs::write(outside.join("root/keep"), b"untouched").unwrap();
        let mut child = Command::new("/usr/bin/bash")
            .args(super::cleanup_args(&private).unwrap())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut ready = String::new();
        std::io::BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut ready)
            .unwrap();
        assert_eq!(ready, "ready\n");
        fs::rename(&private, &moved).unwrap();
        std::os::unix::fs::symlink(&outside, &private).unwrap();
        drop(child.stdin.take());
        assert!(child.wait().unwrap().success());
        assert!(!moved.join("root").exists());
        assert_eq!(fs::read(outside.join("root/keep")).unwrap(), b"untouched");
    }
}
