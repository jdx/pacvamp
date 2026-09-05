//! Optional delegated cgroup v2 control, with an out-of-group death watcher.
use crate::build_process::Limits;
use eyre::{Context as _, Result, bail};
use std::{
    fs,
    io::{BufRead as _, Write as _},
    os::unix::process::CommandExt as _,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

pub fn validate(root: &Path) -> Result<()> {
    if !root.is_absolute()
        || nix::sys::statfs::statfs(root)?.filesystem_type()
            != nix::sys::statfs::CGROUP2_SUPER_MAGIC
    {
        bail!("build cgroup root must be an absolute delegated cgroup v2 directory");
    }
    Ok(())
}
pub fn controls(limits: &Limits) -> Result<Vec<(&'static str, String)>> {
    limits.validate()?;
    Ok(vec![
        ("memory.max", (limits.memory_mb * 1024 * 1024).to_string()),
        ("memory.swap.max", "0".into()),
        ("pids.max", limits.processes.to_string()),
        ("cpu.max", format!("{} 100000", limits.cpu_percent * 1000)),
    ])
}
pub struct Group {
    pub path: PathBuf,
    watcher: Option<Child>,
}
impl Group {
    pub fn create(root: &Path, limits: &Limits, helper: &Path) -> Result<Self> {
        validate(root)?;
        let root = root.canonicalize()?;
        let unique = tempfile::Builder::new()
            .prefix("pacvamp-cgroup-")
            .tempdir()?;
        let path = root.join(unique.path().file_name().unwrap());
        fs::create_dir(&path).wrap_err("creating delegated build cgroup")?;
        let mut group = Self {
            path,
            watcher: None,
        };
        for (name, value) in controls(limits)? {
            fs::write(group.path.join(name), value).wrap_err_with(|| {
                format!("setting cgroup {name}; delegate cpu, memory and pids controllers")
            })?;
        }
        // The watcher never joins the group. Its stdin remains open only in
        // the supervisor; CLOEXEC prevents a recipe inheriting that lease.
        group.watcher = Some(
            Command::new(helper)
                .process_group(0)
                .arg("__cgroup-watch")
                .arg(&group.path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?,
        );
        let watcher = group.watcher.as_mut().unwrap();
        let mut ready = String::new();
        std::io::BufReader::new(watcher.stdout.take().unwrap()).read_line(&mut ready)?;
        if ready != "ready\n" {
            bail!("build cgroup watcher failed to start");
        }
        Ok(group)
    }
}
impl Drop for Group {
    fn drop(&mut self) {
        if let Some(mut watcher) = self.watcher.take() {
            drop(watcher.stdin.take());
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
            loop {
                match watcher.try_wait() {
                    Ok(Some(_)) | Err(_) => break,
                    Ok(None) if std::time::Instant::now() >= deadline => {
                        // Keep cleanup independent without blocking the CLI on
                        // a task stuck in uninterruptible kernel sleep.
                        let _ = std::thread::Builder::new()
                            .name("cgroup-reaper".into())
                            .spawn(move || watcher.wait());
                        return;
                    }
                    Ok(None) => std::thread::sleep(std::time::Duration::from_millis(20)),
                }
            }
        }
        // The watcher may have died independently; retain parent-side cleanup.
        if self.path.exists() {
            let _ = cleanup(&self.path);
        }
    }
}
pub fn join(path: &Path) -> Result<()> {
    validate(path)?;
    fs::write(path.join("cgroup.procs"), std::process::id().to_string())
        .wrap_err("joining build cgroup; run the supervisor inside the delegated subtree and delegate its root cgroup.procs")
}
pub fn cleanup(path: &Path) -> Result<()> {
    validate(path)?;
    fs::write(path.join("cgroup.kill"), "1")?;
    for _ in 0..100 {
        match fs::remove_dir(path) {
            Ok(()) => return Ok(()),
            Err(err) if matches!(err.raw_os_error(), Some(libc::EBUSY | libc::ENOTEMPTY)) => {
                std::thread::sleep(std::time::Duration::from_millis(20))
            }
            Err(err) => return Err(err.into()),
        }
    }
    bail!(
        "build cgroup still populated after SIGKILL: {}",
        path.display()
    )
}
pub fn watch(path: &Path) -> Result<()> {
    validate(path)?;
    // Open the kill control before acknowledging startup, so failure is visible.
    let mut kill = fs::OpenOptions::new()
        .write(true)
        .open(path.join("cgroup.kill"))?;
    println!("ready");
    std::io::stdout().flush()?;
    std::io::copy(&mut std::io::stdin().lock(), &mut std::io::sink())?;
    kill.write_all(b"1")?;
    wait_for_removal(|| fs::remove_dir(path), std::thread::sleep)
}

fn wait_for_removal(
    mut remove: impl FnMut() -> std::io::Result<()>,
    mut wait: impl FnMut(std::time::Duration),
) -> Result<()> {
    let mut delay = std::time::Duration::from_millis(20);
    loop {
        match remove() {
            Ok(()) => return Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(err) if matches!(err.raw_os_error(), Some(libc::EBUSY | libc::ENOTEMPTY)) => {
                wait(delay);
                delay = (delay * 2).min(std::time::Duration::from_secs(1));
            }
            Err(err) => return Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delayed_watcher_does_not_block_group_drop() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("group");
        fs::create_dir(&path).unwrap();
        let watcher = Command::new("/bin/sh")
            .args(["-c", "cat >/dev/null; sleep 4; rmdir -- \"$1\"", "watcher"])
            .arg(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let group = Group {
            path: path.clone(),
            watcher: Some(watcher),
        };
        let start = std::time::Instant::now();
        drop(group);
        assert!(start.elapsed() < std::time::Duration::from_secs(3));
        assert!(path.exists());
        while path.exists() {
            assert!(start.elapsed() < std::time::Duration::from_secs(10));
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    #[test]
    fn watcher_retries_beyond_old_cutoff() {
        let mut attempts = 0;
        let mut waits = 0;
        wait_for_removal(
            || {
                attempts += 1;
                if attempts <= 150 {
                    Err(std::io::Error::from_raw_os_error(if attempts % 2 == 0 {
                        libc::EBUSY
                    } else {
                        libc::ENOTEMPTY
                    }))
                } else {
                    Ok(())
                }
            },
            |delay| {
                assert!(delay <= std::time::Duration::from_secs(1));
                waits += 1;
            },
        )
        .unwrap();
        assert_eq!(waits, 150);
    }

    #[test]
    fn watcher_stops_on_removal_or_permanent_error() {
        assert!(
            wait_for_removal(
                || Err(std::io::Error::from_raw_os_error(libc::ENOENT)),
                |_| panic!("unexpected retry"),
            )
            .is_ok()
        );
        assert!(
            wait_for_removal(
                || Err(std::io::Error::from_raw_os_error(libc::EACCES)),
                |_| panic!("unexpected retry"),
            )
            .is_err()
        );
    }
}
