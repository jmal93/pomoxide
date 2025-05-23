use assert_cmd::Command;

#[test]
fn test_cli_start_default() {
    let mut cmd = Command::cargo_bin("pomoxide").unwrap();
    cmd.arg("start");
    cmd.assert().success();
}

#[test]
fn test_cli_clear() {
    let mut cmd = Command::cargo_bin("pomoxide").unwrap();
    cmd.arg("clear");
    cmd.assert().success();
}

#[test]
fn test_cli_finish() {
    let mut cmd = Command::cargo_bin("pomoxide").unwrap();
    cmd.arg("finish");
    cmd.assert().success();
}
