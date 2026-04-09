use std::process::Command;

fn cargo_bin() -> Command {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "-p", "kreuzcrawl-cli", "--"]);
    cmd
}

#[test]
fn test_cli_help() {
    let output = cargo_bin().arg("--help").output().expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap().to_lowercase();
    assert!(stdout.contains("scrape"));
    assert!(stdout.contains("crawl"));
    assert!(stdout.contains("map"));
}

#[test]
fn test_cli_scrape_help() {
    let output = cargo_bin().args(["scrape", "--help"]).output().expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.to_lowercase().contains("url"));
}

#[test]
fn test_cli_crawl_help() {
    let output = cargo_bin().args(["crawl", "--help"]).output().expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap().to_lowercase();
    assert!(stdout.contains("depth"));
    assert!(stdout.contains("max-pages"));
}

#[test]
fn test_cli_map_help() {
    let output = cargo_bin().args(["map", "--help"]).output().expect("failed");
    assert!(output.status.success());
}
