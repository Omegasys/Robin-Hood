use red_robin::cli::Cli;
use clap::Parser;

#[test]
fn cli_parses_without_arguments() {
    let cli = Cli::try_parse_from(["redrobin"])
        .expect("CLI should parse without arguments");

    assert!(!cli.verbose);
    assert!(!cli.quiet);
    assert!(!cli.json);
    assert!(!cli.version);
    assert!(cli.config.is_none());
}

#[test]
fn cli_parses_verbose() {
    let cli = Cli::try_parse_from([
        "redrobin",
        "--verbose",
    ])
    .unwrap();

    assert!(cli.verbose);
}

#[test]
fn cli_parses_short_verbose() {
    let cli = Cli::try_parse_from([
        "redrobin",
        "-v",
    ])
    .unwrap();

    assert!(cli.verbose);
}

#[test]
fn cli_parses_quiet() {
    let cli = Cli::try_parse_from([
        "redrobin",
        "--quiet",
    ])
    .unwrap();

    assert!(cli.quiet);
}

#[test]
fn cli_parses_json() {
    let cli = Cli::try_parse_from([
        "redrobin",
        "--json",
    ])
    .unwrap();

    assert!(cli.json);
}

#[test]
fn cli_parses_version() {
    let cli = Cli::try_parse_from([
        "redrobin",
        "--version",
    ])
    .unwrap();

    assert!(cli.version);
}

#[test]
fn cli_parses_config_path() {
    let cli = Cli::try_parse_from([
        "redrobin",
        "--config",
        "/tmp/redrobin.yml",
    ])
    .unwrap();

    assert_eq!(
        cli.config.unwrap().to_string_lossy(),
        "/tmp/redrobin.yml"
    );
}

#[test]
fn cli_parses_multiple_options() {
    let cli = Cli::try_parse_from([
        "redrobin",
        "--config",
        "custom.yml",
        "--verbose",
        "--json",
    ])
    .unwrap();

    assert!(cli.verbose);
    assert!(cli.json);
    assert_eq!(
        cli.config.unwrap().to_string_lossy(),
        "custom.yml"
    );
}