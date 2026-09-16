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
        .https_only(true)
        .build()
    {
        Ok(client) => client,

        Err(error) => {
            return TestResult::yuck(
                name,
                "Client error",
                format!("Unable to create TLS client: {error}"),
            );
        }
    };

    let mut statistics = Statistics::new();

    for _ in 0..=settings.retries {
        let start = Instant::now();

        match client.head(url).send().await {
            Ok(response) if response.version() == reqwest::Version::HTTP_2
                || response.version() == reqwest::Version::HTTP_11 =>
            {
                statistics.record_success(start.elapsed());
            }

            Ok(_) => {
                statistics.record_success(start.elapsed());
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
            "TLS connection could not be established",
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
            "{} | TLS connection established",
            statistics.summary()
        ),
    )
}