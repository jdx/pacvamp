use pacvamp::aur::cache::{inventory, lease};
use std::{
    collections::BTreeSet,
    fs,
    time::{Duration, SystemTime},
};
#[test]
fn retention_protects_active_recent_and_referenced_runs() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join(".pacvamp-build/runs");
    for name in ["old", "installed", "recent"] {
        let run = root.join(name);
        fs::create_dir_all(&run).unwrap();
        fs::write(run.join("receipt.json"), b"receipt").unwrap();
        if name != "recent" {
            fs::File::open(&run)
                .unwrap()
                .set_modified(SystemTime::now() - Duration::from_secs(86400 * 40))
                .unwrap();
        }
    }
    let active = lease(dir.path(), false).unwrap();
    assert!(lease(dir.path(), true).is_err());
    assert!(lease(dir.path(), false).is_ok());
    drop(active);
    let _prune = lease(dir.path(), true).unwrap();
    let protected = BTreeSet::from([root.join("installed/receipt.json").canonicalize().unwrap()]);
    let runs = inventory(dir.path(), &protected, 30, Some(0)).unwrap();
    assert_eq!(
        runs.iter()
            .filter(|r| r.prune)
            .map(|r| r.path.file_name().unwrap().to_str().unwrap())
            .collect::<Vec<_>>(),
        ["old"]
    );
}

#[test]
fn live_estimates_tolerate_directory_removal() {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    let dir = tempfile::tempdir().unwrap();
    let run = dir.path().join(".pacvamp-build/runs/live");
    fs::create_dir_all(&run).unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let done = stop.clone();
    let writer = std::thread::spawn(move || {
        while !done.load(Ordering::Relaxed) {
            fs::create_dir_all(run.join("builddir/subdir")).unwrap();
            fs::write(run.join("builddir/subdir/file"), b"building").unwrap();
            fs::remove_dir_all(run.join("builddir")).unwrap();
        }
    });
    let results = (0..500)
        .map(|_| inventory(dir.path(), &BTreeSet::new(), 30, None))
        .collect::<Vec<_>>();
    stop.store(true, Ordering::Relaxed);
    writer.join().unwrap();
    for result in results {
        result.unwrap();
    }
}

#[test]
fn unreadable_runs_are_reported_and_do_not_hide_other_candidates() {
    use std::os::unix::fs::PermissionsExt as _;
    if nix::unistd::geteuid().is_root() {
        return; // Root bypasses the permission failure this test exercises.
    }
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join(".pacvamp-build/runs");
    for name in ["unreadable", "eligible"] {
        let run = root.join(name);
        fs::create_dir_all(run.join("sources")).unwrap();
        fs::File::open(&run)
            .unwrap()
            .set_modified(SystemTime::now() - Duration::from_secs(86400 * 40))
            .unwrap();
    }
    let hidden = root.join("unreadable/sources");
    fs::set_permissions(&hidden, fs::Permissions::from_mode(0o000)).unwrap();
    let result = inventory(dir.path(), &BTreeSet::new(), 30, Some(0));
    fs::set_permissions(&hidden, fs::Permissions::from_mode(0o700)).unwrap();
    let runs = result.unwrap();
    let unknown = runs
        .iter()
        .find(|r| r.path.ends_with("unreadable"))
        .unwrap();
    assert!(
        unknown.bytes.is_none() && unknown.error.is_some() && unknown.protected && !unknown.prune
    );
    assert!(
        runs.iter()
            .find(|r| r.path.ends_with("eligible"))
            .unwrap()
            .prune
    );
}

#[test]
fn prune_removes_read_only_trees_without_changing_link_targets() {
    use std::os::unix::fs::PermissionsExt as _;
    let dir = tempfile::tempdir().unwrap();
    let run = dir.path().join("run");
    let outside = dir.path().join("outside");
    fs::create_dir_all(run.join("sources/subdir")).unwrap();
    fs::create_dir(&outside).unwrap();
    fs::write(run.join("sources/subdir/file"), b"source").unwrap();
    fs::write(outside.join("file"), b"untouched").unwrap();
    std::os::unix::fs::symlink(&outside, run.join("sources/link")).unwrap();
    fs::hard_link(outside.join("file"), run.join("sources/hardlink")).unwrap();
    fs::set_permissions(outside.join("file"), fs::Permissions::from_mode(0o444)).unwrap();
    for path in [
        &run,
        &run.join("sources"),
        &run.join("sources/subdir"),
        &outside,
    ] {
        fs::set_permissions(path, fs::Permissions::from_mode(0o555)).unwrap();
    }
    let result = pacvamp::aur::cache::remove_run(&run);
    let outside_mode = fs::metadata(&outside).unwrap().permissions().mode() & 0o777;
    let file_mode = fs::metadata(outside.join("file"))
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    fs::set_permissions(&outside, fs::Permissions::from_mode(0o700)).unwrap();
    result.unwrap();
    assert!(!run.exists());
    assert_eq!(outside_mode, 0o555);
    assert_eq!(file_mode, 0o444);
    assert_eq!(fs::read(outside.join("file")).unwrap(), b"untouched");
}
