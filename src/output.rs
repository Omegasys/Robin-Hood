use crate::{
    ratings::Rating,
    result::TestResult,
};

const WIDTH: usize = 78;

pub fn print_header() {
    println!();
    println!("Red Robin Network Test");
    println!("{}", "─".repeat(WIDTH));
    println!(
        "{:<28} {:<10} {}",
        "LOCATION / TARGET",
        "STATUS",
        "STATISTICS"
    );
    println!("{}", "─".repeat(WIDTH));
}

pub fn print_result(result: &TestResult) {
    println!(
        "{:<28} {:<10} {}",
        truncate(&result.name, 28),
        result.rating,
        result.statistics
    );
}

pub fn print_results(results: &[TestResult]) {
    print_header();

    for result in results {
        print_result(result);
    }

    print_footer(results);
}

pub fn print_verbose_result(result: &TestResult) {
    print_result(result);

    if let Some(message) = &result.message {
        println!("  └─ {}", message);
    }
}

pub fn print_footer(results: &[TestResult]) {
    println!("{}", "─".repeat(WIDTH));

    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len().saturating_sub(successful);

    println!();
    println!(
        "Connections: {}/{} successful",
        successful,
        results.len()
    );

    println!("Failed: {}", failed);

    if let Some(overall) = overall_rating(results) {
        println!("Overall: {}", overall);
    }

    println!();
}

pub fn overall_rating(results: &[TestResult]) -> Option<Rating> {
    if results.is_empty() {
        return None;
    }

    let failed = results.iter().filter(|r| !r.success).count();

    if failed == 0 {
        return Some(Rating::Yum);
    }

    if failed < results.len() {
        return Some(Rating::Meh);
    }

    Some(Rating::Yuck)
}

fn truncate(value: &str, width: usize) -> String {
    if value.chars().count() <= width {
        return value.to_string();
    }

    let truncated: String = value.chars().take(width.saturating_sub(3)).collect();

    format!("{truncated}...")
}