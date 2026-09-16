use crate::{
    config::Settings,
    ratings::Rating,
    result::TestResult,
    statistics::Statistics,
};

use std::time::{Duration, Instant};
use tokio::net::UdpSocket;

pub async fn test(
    settings: &Settings,
    name: &str,
    address: &str,
    port: u16,
    payload: &[u8],
) -> TestResult {
    let target = format!("{address}:{port}");
    let mut statistics = Statistics::new();

    for _ in 0..=settings.retries {
        let start = Instant::now();

        let socket = match UdpSocket::bind("0.0.0.0:0").await {
            Ok(socket) => socket,
            Err(_) => {
                statistics.record_failure();
                continue;
            }
        };

        if socket.connect(&target).await.is_err() {
            statistics.record_failure();
            continue;
        }

        if socket.send(payload).await.is_err() {
            statistics.record_failure();
            continue;
        }

        let mut buffer = [0u8; 4096];

        let result = tokio::time::timeout(
            Duration::from_secs(settings.timeout_seconds),
            socket.recv(&mut buffer),
        )
        .await;

        match result {
            Ok(Ok(_bytes)) => {
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
            format!(
                "No UDP response received from {target}; \
                 UDP services may intentionally ignore probes"
            ),
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