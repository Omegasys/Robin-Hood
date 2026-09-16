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
    address: &str,
    port: u16,
) -> TestResult {
    let proxy = format!("socks5h://{}:{}", address, port);

    let client = match reqwest::Client::builder()
        .proxy(
            reqwest::Proxy::all(&proxy)
                .expect("validated SOCKS proxy URL"),
        )
        .timeout(Duration::from_secs(settings.timeout_seconds))
        .build()
    {
        Ok(client) => client,

        Err(error) => {
            return TestResult::yuck(
                name,
                "Tor client error",
                error.to_string(),
            );
        }
    };

    let mut statistics = Statistics::new();

    for _ in 0..=settings.retries {
        let start = Instant::now();

        match client
            .get("https://check.torproject.org/")
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
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
                "Tor SOCKS proxy {}:{} could not establish a connection",
                address, port
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
            "{} | SOCKS5 {}:{}",
            statistics.summary(),
            address,
            port
        ),
    )
}