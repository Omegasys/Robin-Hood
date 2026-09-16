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
    onion_url: &str,
    tor_address: &str,
    tor_port: u16,
) -> TestResult {
    let proxy = format!(
        "socks5h://{}:{}",
        tor_address,
        tor_port
    );

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
    let mut status = None;

    for _ in 0..=settings.retries {
        let start = Instant::now();

        match client.get(onion_url).send().await {
            Ok(response) => {
                status = Some(response.status().as_u16());

                if response.status().is_success() {
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
                "Onion service unavailable; status: {:?}",
                status
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
            "{} | HTTP {} | Tor {}:{}",
            statistics.summary(),
            status.unwrap_or(0),
            tor_address,
            tor_port
        ),
    )
}