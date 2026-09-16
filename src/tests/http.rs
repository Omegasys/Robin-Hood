use crate::{
    config::Settings,
    ratings::Rating,
    result::TestResult,
    statistics::Statistics,
};

use std::time::{Duration, Instant};

pub async fn test(
    settings: &Settings,
    name: &str,
    url: &str,
) -> TestResult {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(settings.timeout_seconds))
        .build()
    {
        Ok(client) => client,

        Err(error) => {
            return TestResult::yuck(
                name,
                "Client error",
                format!("Unable to create HTTP client: {error}"),
            );
        }
    };

    let mut statistics = Statistics::new();
    let mut last_status = None;

    for _ in 0..=settings.retries {
        let start = Instant::now();

        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status();
                last_status = Some(status.as_u16());

                if status.is_success() {
                    statistics.record_success(start.elapsed());
                } else {
                    statistics.record_failure();
                }
            }

            Err(_) => {
                statistics.record_failure();
            }
        }
    }

    if statistics.successes == 0 {
        return TestResult::yuck(
            name,
            statistics.summary(),
            format!(
                "HTTP connection failed; last status: {:?}",
                last_status
            ),
        );
    }

    let latency = statistics
        .average_latency_ms()
        .unwrap_or(f64::MAX);

    let rating = Rating::from_latency(
        latency,
        &settings.rating,
    );

    TestResult::success(
        name,
        rating.as_str(),
        format!(
            "{} | HTTP {}",
            statistics.summary(),
            last_status.unwrap_or(0)
        ),
    )
}