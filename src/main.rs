mod cli;
mod config;
mod result;
mod runner;

use anyhow::Result;
use clap::Parser;

use cli::Cli;
use config::load_config;
use runner::Runner;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("Red Robin {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config_path = cli.config.as_deref();

    let configuration = load_config(config_path)?;

    let runner = Runner::new(configuration);

    let results = runner.run().await;

    if cli.json {
        println!("{}", serde_json_output(&results));
    } else {
        print_results(&results, cli.verbose);
    }

    if results.iter().any(|result| !result.success) {
        std::process::exit(1);
    }

    Ok(())
}

fn print_results(results: &[result::TestResult], verbose: bool) {
    println!();
    println!("Red Robin Network Test");
    println!("────────────────────────────────────────────────────────────────────────");
    println!(
        "{:<28} {:<10} {}",
        "LOCATION / TARGET",
        "STATUS",
        "STATISTICS"
    );
    println!("────────────────────────────────────────────────────────────────────────");

    for result in results {
        println!(
            "{:<28} {:<10} {}",
            result.name,
            result.rating,
            result.statistics
        );

        if verbose {
            if let Some(message) = &result.message {
                println!("  └─ {}", message);
            }
        }
    }

    println!("────────────────────────────────────────────────────────────────────────");

    let successful = results.iter().filter(|result| result.success).count();
    let total = results.len();

    println!();
    println!("Connections: {}/{} successful", successful, total);
}

fn serde_json_output(results: &[result::TestResult]) -> String {
    let mut output = String::from("[\n");

    for (index, result) in results.iter().enumerate() {
        output.push_str("  {\n");
        output.push_str(&format!(
            "    \"name\": \"{}\",\n",
            escape_json(&result.name)
        ));
        output.push_str(&format!(
            "    \"rating\": \"{}\",\n",
            escape_json(&result.rating)
        ));
        output.push_str(&format!(
            "    \"statistics\": \"{}\",\n",
            escape_json(&result.statistics)
        ));
        output.push_str(&format!("    \"success\": {}\n", result.success));

        if index + 1 == results.len() {
            output.push_str("  }\n");
        } else {
            output.push_str("  },\n");
        }
    }

    output.push(']');
    output
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}