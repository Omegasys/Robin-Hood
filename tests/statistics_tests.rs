use red_robin::statistics::Statistics;
use std::time::Duration;

#[test]
fn new_statistics_have_zero_attempts() {
    let statistics = Statistics::new();

    assert_eq!(statistics.attempts, 0);
    assert_eq!(statistics.successes, 0);
    assert_eq!(statistics.failures, 0);
    assert!(statistics.latencies_ms.is_empty());
}

#[test]
fn successful_attempt_is_recorded() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(50));

    assert_eq!(statistics.attempts, 1);
    assert_eq!(statistics.successes, 1);
    assert_eq!(statistics.failures, 0);
    assert_eq!(statistics.latencies_ms.len(), 1);
}

#[test]
fn failed_attempt_is_recorded() {
    let mut statistics = Statistics::new();

    statistics.record_failure();

    assert_eq!(statistics.attempts, 1);
    assert_eq!(statistics.successes, 0);
    assert_eq!(statistics.failures, 1);
}

#[test]
fn packet_loss_is_calculated() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(20));
    statistics.record_success(Duration::from_millis(30));
    statistics.record_failure();
    statistics.record_failure();

    assert!((statistics.packet_loss_percent() - 50.0).abs() < f64::EPSILON);
}

#[test]
fn zero_attempts_have_zero_packet_loss() {
    let statistics = Statistics::new();

    assert_eq!(statistics.packet_loss_percent(), 0.0);
}

#[test]
fn minimum_latency_is_calculated() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(50));
    statistics.record_success(Duration::from_millis(20));
    statistics.record_success(Duration::from_millis(80));

    assert_eq!(
        statistics.min_latency_ms().unwrap(),
        20.0
    );
}

#[test]
fn maximum_latency_is_calculated() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(50));
    statistics.record_success(Duration::from_millis(20));
    statistics.record_success(Duration::from_millis(80));

    assert_eq!(
        statistics.max_latency_ms().unwrap(),
        80.0
    );
}

#[test]
fn average_latency_is_calculated() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(20));
    statistics.record_success(Duration::from_millis(40));
    statistics.record_success(Duration::from_millis(60));

    assert_eq!(
        statistics.average_latency_ms().unwrap(),
        40.0
    );
}

#[test]
fn median_latency_works_with_odd_number_of_values() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(10));
    statistics.record_success(Duration::from_millis(50));
    statistics.record_success(Duration::from_millis(30));

    assert_eq!(
        statistics.median_latency_ms().unwrap(),
        30.0
    );
}

#[test]
fn median_latency_works_with_even_number_of_values() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(10));
    statistics.record_success(Duration::from_millis(30));
    statistics.record_success(Duration::from_millis(50));
    statistics.record_success(Duration::from_millis(70));

    assert_eq!(
        statistics.median_latency_ms().unwrap(),
        40.0
    );
}

#[test]
fn jitter_is_calculated() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(10));
    statistics.record_success(Duration::from_millis(20));
    statistics.record_success(Duration::from_millis(40));

    let jitter = statistics.jitter_ms().unwrap();

    assert_eq!(jitter, 15.0);
}

#[test]
fn jitter_requires_two_samples() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(10));

    assert!(statistics.jitter_ms().is_none());
}

#[test]
fn empty_statistics_have_no_summary_attempts() {
    let statistics = Statistics::new();

    assert_eq!(
        statistics.summary(),
        "No attempts"
    );
}

#[test]
fn summary_contains_useful_statistics() {
    let mut statistics = Statistics::new();

    statistics.record_success(Duration::from_millis(20));
    statistics.record_success(Duration::from_millis(40));
    statistics.record_failure();

    let summary = statistics.summary();

    assert!(summary.contains("30.0 ms avg"));
    assert!(summary.contains("20.0 ms min"));
    assert!(summary.contains("40.0 ms max"));
    assert!(summary.contains("33.3% loss"));
    assert!(summary.contains("2 / 3"));
}