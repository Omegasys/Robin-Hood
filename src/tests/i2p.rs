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
    let proxy_url = format!(
        "http://{}:{}",
        address,
        port
    );

    let proxy = match reqwest::Proxy::http(&proxy_url) {
        Ok(proxy) => proxy,

        Err(error) => {
            return TestResult::yuck(
                name,
                "Proxy error",
                error.to_string(),
            );
        }
    };

    let client = match reqwest::Client::builder()
        .proxy(proxy)
        .timeout(Duration::from_secs(settings.timeout_seconds))
        .build()
    {
        Ok(client) => client,

        Err(error) => {
            return TestResult::yuck(
                name,
                "Client error",
                error.to_string(),
            );
        }
    };

    let mut statistics = Statistics::new();

    for _ in 0..=settings.retries {
        let start = Instant::now();

        /*
         * The I2P router console is useful for determining whether
         * the local I2P installation is reachable.
         */
        let url = format!(
            "http://{}:{}/",
            address,
            port
        );

        match client.get(&url).send().await {
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
                "I2P HTTP proxy/router at {}:{} did not respond",
                address,
                port
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
            "{} | I2P {}:{}",
            statistics.summary(),
            address,
            port
        ),
    )
}