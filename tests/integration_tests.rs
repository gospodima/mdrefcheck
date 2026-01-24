use predicates::prelude::*;
use std::fs;

#[test]
fn test_help_flag() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Check markdown references"));
}

#[test]
fn test_fails_on_broken_links() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("bad.md");
    fs::write(&file_path, "[broken link](./nonexistent.md)").unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg(dir.path());

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("File not found"));
}

#[test]
fn test_succeeds_on_valid_links() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("good.md");
    let target_path = dir.path().join("target.md");

    fs::write(&file_path, "[valid link](./target.md)").unwrap();
    fs::write(&target_path, "# Target").unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg(dir.path());

    cmd.assert().success();
}

#[test]
fn test_image_local_missing_cli() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("img2.md");
    fs::write(&file_path, "![img](./missing.png)").unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg(dir.path());

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Image not found"));
}

#[test]
fn test_mailto_invalid_cli() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("mail.md");
    fs::write(&file_path, "[contact](mailto:not-an-email)").unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg(dir.path());

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Invalid email"));
}

#[test]
fn test_heading_anchor_cli() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a.md");
    let b = dir.path().join("b.md");

    fs::write(&b, "# Intro").unwrap();
    fs::write(&a, "[goto](./b.md#intro)").unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg(dir.path());
    cmd.assert().success();
}

#[test]
fn test_exclude_path_cli() {
    let dir = tempfile::tempdir().unwrap();
    let bad = dir.path().join("bad.md");
    fs::write(&bad, "[broken](./missing.md)").unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    // exclude the input root itself -> scanner pre-filters and returns no files
    cmd.arg("--exclude").arg(dir.path()).arg(dir.path());

    // excluded input should result in no files scanned
    cmd.assert().success();
}

#[test]
fn test_ignore_regex_skips_validation_cli() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("x.md");
    fs::write(&f, "[skipme](skip:whatever)").unwrap();

    // without ignore, this would error (treated as file path)
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg("--ignore").arg("^skip:").arg(dir.path());

    // --ignore should prevent validation of that link
    cmd.assert().success();
}

#[test]
fn test_multiple_headings_cli() {
    let dir = tempfile::tempdir().unwrap();
    let tgt = dir.path().join("tgt.md");
    let src = dir.path().join("src.md");

    fs::write(&tgt, "# One\n## Two\n### Three").unwrap();
    fs::write(
        &src,
        "[a](./tgt.md#one)\n[b](./tgt.md#two)\n[c](./tgt.md#three)",
    )
    .unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg(dir.path());
    cmd.assert().success();
}

#[test]
fn test_repeated_headings_cli() {
    let dir = tempfile::tempdir().unwrap();
    let tgt = dir.path().join("tgt.md");
    let src = dir.path().join("src.md");

    // three identical headings -> anchors: intro, intro-1, intro-2
    fs::write(&tgt, "# Intro\n# Intro\n# Intro").unwrap();
    fs::write(
        &src,
        "[a](./tgt.md#intro)\n[b](./tgt.md#intro-1)\n[c](./tgt.md#intro-2)",
    )
    .unwrap();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd.arg(dir.path());
    cmd.assert().success();

    // referencing a non-existent numbered anchor should fail
    fs::write(&src, "[d](./tgt.md#intro-3)").unwrap();
    let mut cmd2 = assert_cmd::cargo::cargo_bin_cmd!("mdrefcheck");
    cmd2.arg(dir.path());
    cmd2.assert()
        .failure()
        .stdout(predicate::str::contains("Missing heading"));
}
