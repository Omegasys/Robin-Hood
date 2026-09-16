use crate::{
    config::Settings,
    ratings::Rating,
    result::TestResult,
    statistics::Statistics,
};

use std::{
    process::Stdio,
    time::{Duration, Instant},
};

pub async fn test(
    settings: &Settings,
    name: &str,
    target: &str,
) -> TestResult {
    let mut statistics = Statistics::new();

    for _ in 0..=settings.retries {
        let start = Instant::now();

        let timeout = Duration::from_secs(settings.timeout_seconds);

        let result = tokio::time::timeout(
            timeout,
            tokio::process::Command::new("ping")
                .arg("-c")
                .arg("1")
                .arg("-W")
                .arg(settings.timeout_seconds.to_string())
                .arg(target)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status(),
        )
        .await;

        match result {
            Ok(Ok(status)) if status.success() => {
                statistics.record_success(start.elapsed());
            }

            _ => {
                statistics.record_failure();
            }
        }
    }

    if statistics.successes == 0 {
        return TestResult::yuck(
            name,
            statistics.summary(),
            format!("ICMP connection to {target} failed"),
        );
    }

    let latency = statistics.average_latency_ms().unwrap_or(f64::MAX);

    let rating = Rating::from_latency(
        latency,
        &settings.rating,
    );

    TestResult::success(
        name,
        rating.as_str(),
        statistics.summary(),
    )
}