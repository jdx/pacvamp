//! Retention for build runs, separate from recipe checkouts and lock files.
use eyre::{Context as _, Result};
use nix::fcntl::{Flock, FlockArg};
use serde::Serialize;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

pub fn lease(cache: &Path, prune: bool) -> Result<Flock<fs::File>> {
    fs::create_dir_all(cache.join(".locks/cache"))?;
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(cache.join(".locks/cache/retention"))?;
    Flock::lock(
        file,
        if prune {
            FlockArg::LockExclusiveNonblock
        } else {
            FlockArg::LockSharedNonblock
        },
    )
    .map_err(|(_, err)| eyre::eyre!(err))
    .wrap_err("cache is busy; retry after builds or pruning finish")
}

#[derive(Debug, Serialize)]
pub struct Run {
    pub path: PathBuf,
    pub bytes: Option<u64>,
    pub error: Option<String>,
    pub removed: bool,
    pub age_seconds: u64,
    pub protected: bool,
    pub prune: bool,
}

pub fn inventory(
    cache: &Path,
    protected: &BTreeSet<PathBuf>,
    days: u64,
    max_bytes: Option<u64>,
) -> Result<Vec<Run>> {
    let root = cache.join(".pacvamp-build/runs");
    if !root.exists() {
        return Ok(Vec::new());
    }
    let root = root.canonicalize()?;
    let mut runs = Vec::new();
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let path = entry.path();
        let age_seconds = entry
            .metadata()?
            .modified()?
            .elapsed()
            .unwrap_or_default()
            .as_secs();
        let referenced = protected.iter().any(|p| p.starts_with(&path));
        let (bytes, error) = match crate::build_process::disk_size(&path) {
            Ok(bytes) => (Some(bytes), None),
            Err(err) => (None, Some(format!("cannot size run; retained: {err:#}"))),
        };
        let protected = referenced || age_seconds < 3600 || error.is_some();
        runs.push(Run {
            bytes,
            error,
            removed: false,
            path,
            age_seconds,
            protected,
            prune: false,
        });
    }
    runs.sort_by_key(|run| std::cmp::Reverse(run.age_seconds));
    let mut retained = runs
        .iter()
        .fold(0u64, |n, r| n.saturating_add(r.bytes.unwrap_or(0)));
    for run in &mut runs {
        run.prune = !run.protected
            && (run.age_seconds >= days.saturating_mul(86400)
                || max_bytes.is_some_and(|max| retained > max));
        if run.prune {
            retained = retained.saturating_sub(run.bytes.unwrap_or(0));
        }
    }
    Ok(runs)
}

/// Make owned directories writable without following links or changing files,
/// then remove the run. A read-only file needs no chmod to unlink it.
pub fn remove_run(path: &Path) -> Result<()> {
    use nix::{
        dir::{Dir, OwningIter},
        errno::Errno,
        fcntl::{AtFlags, OFlag},
        sys::stat::{Mode, SFlag, fchmod, fstat, fstatat},
    };
    use std::os::fd::{AsFd as _, OwnedFd};
    let flags = OFlag::O_RDONLY | OFlag::O_DIRECTORY | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC;
    fn frame(dir: Dir) -> Result<(OwnedFd, OwningIter)> {
        let stat = fstat(&dir)?;
        fchmod(&dir, Mode::from_bits_truncate(stat.st_mode) | Mode::S_IRWXU)?;
        Ok((dir.as_fd().try_clone_to_owned()?, dir.into_iter()))
    }
    let mut stack = vec![frame(Dir::open(path, flags, Mode::empty())?)?];
    while let Some((fd, entries)) = stack.last_mut() {
        let Some(entry) = entries.next() else {
            stack.pop();
            continue;
        };
        let entry = entry?;
        let name = entry.file_name();
        if matches!(name.to_bytes(), b"." | b"..") {
            continue;
        }
        let stat = match fstatat(&*fd, name, AtFlags::AT_SYMLINK_NOFOLLOW) {
            Ok(stat) => stat,
            Err(Errno::ENOENT) => continue,
            Err(err) => return Err(err.into()),
        };
        if SFlag::from_bits_truncate(stat.st_mode) & SFlag::S_IFMT == SFlag::S_IFDIR {
            match Dir::openat(&*fd, name, flags, Mode::empty()) {
                Ok(child) => stack.push(frame(child)?),
                Err(Errno::ENOENT | Errno::ENOTDIR | Errno::ELOOP) => {}
                Err(err) => return Err(err.into()),
            }
        }
    }
    fs::remove_dir_all(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn retention_package_has_an_independent_lock() {
        let dir = tempfile::tempdir().unwrap();
        let lease = super::lease(dir.path(), false).unwrap();
        let _package = crate::aur::locking::acquire(dir.path(), "retention").unwrap();
        drop(lease);
        let _prune = super::lease(dir.path(), true).unwrap();
    }
}
