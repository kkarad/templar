extern crate tempfile;

use std::path::{PathBuf, Path};
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::TempDir;

use indoc::indoc;

#[test]
fn help() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());

    cmd.arg("-h");
    cmd.assert().success().stdout(predicate::str::contains("Usage:"));

    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains("Usage:"));
}

#[test]
fn version() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());

    let expected = format!("Templar version: {}\n", env!("CARGO_PKG_VERSION"));
    cmd.arg("--version");
    cmd.assert().success().stdout(predicate::str::similar(expected));
}

#[test]
fn release() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--parse");
    cmd.assert().success().stdout(predicate::str::starts_with("Release { name: \"test\", "));
}

#[test]
fn parse_current_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-c").arg("1.1").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("current_version: \"1.1\""));
}

#[test]
fn parse_current_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--current").arg("1.1").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("current_version: \"1.1\""));
}

#[test]
fn validate_missing_current_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-c").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: -c"));
}

#[test]
fn validate_missing_current_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--current").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: --current"));
}

#[test]
fn parse_next_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-n").arg("1.2").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("next_version: \"1.2\""));
}

#[test]
fn parse_next_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--next").arg("1.2").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("next_version: \"1.2\""));
}

#[test]
fn validate_missing_next_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-n").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: -n"));
}

#[test]
fn validate_missing_next_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--next").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: --next"));
}

#[test]
fn parse_tweet_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-t").arg("hello!").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("tweet: \"hello!\""));
}

#[test]
fn parse_tweet_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--tweet").arg("hello!").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("tweet: \"hello!\""));
}

#[test]
fn validate_missing_tweet_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-t").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: -t"));
}

#[test]
fn validate_missing_tweet_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--tweet").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: --tweet"));
}

#[test]
fn parse_pvt_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-p").arg("70-90").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("pvt_line_range: \"70-90\""));
}

#[test]
fn parse_pvt_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--pvt-line-range").arg("70-90").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("pvt_line_range: \"70-90\""));
}

#[test]
fn validate_missing_pvt_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-p").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: -p"));
}

#[test]
fn validate_missing_pvt_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--pvt-line-range").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: --pvt-line-range"));
}

#[test]
fn parse_jiras_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-j").arg("JIRA-1").arg("JIRA-2").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("jiras: [\"JIRA-1\", \"JIRA-2\"]"));
}

#[test]
fn parse_jiras_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--jiras").arg("JIRA-1").arg("JIRA-2").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("jiras: [\"JIRA-1\", \"JIRA-2\"]"));
}

#[test]
fn validate_missing_jiras_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-j").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: -j"));
}

#[test]
fn validate_missing_jiras_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--jiras").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: --jiras"));
}

#[test]
fn parse_wip_jiras_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-w").arg("JIRA-1").arg("JIRA-2").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("wip_jiras: [\"JIRA-1\", \"JIRA-2\"]"));
}

#[test]
fn parse_wip_jiras_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--wip-jiras").arg("JIRA-1").arg("JIRA-2").arg("--parse");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Release { name: \"test\", "))
        .stdout(predicate::str::contains("wip_jiras: [\"JIRA-1\", \"JIRA-2\"]"));
}

#[test]
fn validate_missing_wip_jiras_short_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("-w").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: -w"));
}

#[test]
fn validate_missing_wip_jiras_long_option_value() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test").arg("--wip-jiras").arg("--parse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::starts_with("error: Missing option value(s) for: --wip-jiras"));
}

// to view println: cargo test -- --nocapture
#[test]
fn conf_file_is_created_with_default_content_when_it_does_not_exists() {
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_default_conf(tmp_dir.path());
    cmd.arg("release").arg("test");
    cmd.assert().success();

    let conf_file: PathBuf = [tmp_dir.path().to_str().unwrap(), ".templar.toml"].iter().collect();
    assert!(conf_file.exists(), format!("file doesn't exist: {:?}", conf_file));

    let res = std::fs::read_to_string(conf_file);
    assert!(res.is_ok());
    assert!(res.unwrap().starts_with("# Templar Configuration"));
}

#[test]
fn validate_unknown_release_name_during_arg_parsing() {
    let conf = indoc!(r#"
        [[release]]
        name = "a_release"
    "#);
    let tmp_dir = TempDir::new().expect("temp_dir failed");
    let mut cmd = templar_cmd_with_conf(tmp_dir.path(), conf);
    cmd.arg("release").arg("unknown_release").arg("--parse");
    cmd.assert().failure()
        .stderr(predicate::str::starts_with("error: Unknown release: unknown_release"));
}

fn templar_cmd_with_default_conf(home_dir: &Path) -> Command {
    let conf = indoc!(r#"
        # Templar Configuration
        [[release]]
        name = "test"
    "#);
    templar_cmd_with_conf(home_dir, conf)
}

fn templar_cmd_with_conf(home_dir: &Path, conf: &'_ str) -> Command {
    let tmp_dir = home_dir.to_str().unwrap();
    let conf_file: PathBuf = [tmp_dir, ".templar.toml" ].iter().collect();
    std::fs::write(conf_file, conf).unwrap();
    let mut cmd = Command::cargo_bin("templar").unwrap();
    cmd.arg("--home").arg(tmp_dir);
    cmd
}