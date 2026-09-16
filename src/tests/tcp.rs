use crate::{
    config::Settings,
    ratings::Rating,
    result::TestResult,
    statistics::Statistics,
};

use std::time::{Duration, Instant};
use tokio::net::TcpStream;

pub async fn test(
    settings: &Settings,
    name: &str,
    address: &str,
    port: u16,
) -> TestResult {
    let target = format!("{address}:{port}");
    let mut statistics = Statistics::new();

    for _ in 0..=settings.retries {
        let start = Instant::now();

        let result = tokio::time::timeout(
            Duration::from_secs(settings.timeout_seconds),
            TcpStream::connect(&target),
        )
        .await;

        match result {
            Ok(Ok(_stream)) => {
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
            format!("TCP connection to {target} failed"),
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