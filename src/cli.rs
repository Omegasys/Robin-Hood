use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "redrobin",
    version,
    about = "A configurable Linux network connectivity and diagnostics tester",
    long_about = "Red Robin tests network connectivity using configurable \
                  ICMP, DNS, HTTP/HTTPS, Tor, I2P, and other connection methods."
)]
pub struct Cli {
    /// Path to a custom YAML configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Show detailed information about each test
    #[arg(short, long)]
    pub verbose: bool,

    /// Show only failed tests
    #[arg(short, long)]
    pub quiet: bool,

    /// Output results as JSON
    #[arg(short, long)]
    pub json: bool,

    /// Show version information
    #[arg(short = 'V', long)]
    pub version: bool,
}