use assert_cmd::Command;

#[test]
fn test_cli_start_default() {
    let mut cmd = Command::cargo_bin("pomoxide").unwrap();
    cmd.arg("start").arg("--duration").arg("1");
    cmd.assert().success();
}

#[test]
fn test_cli_break() {
    let mut cmd = Command::cargo_bin("pomoxide").unwrap();
    cmd.arg("break");
    cmd.assert().success();
}

