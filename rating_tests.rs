use red_robin::config::RatingConfig;
use red_robin::ratings::Rating;

fn test_config() -> RatingConfig {
    RatingConfig {
        yum_max_latency_ms: 100,
        meh_max_latency_ms: 500,
    }
}

#[test]
fn low_latency_is_yum() {
    let rating = Rating::from_latency(25.0, &test_config());

    assert_eq!(rating, Rating::Yum);
}

#[test]
fn boundary_yum_latency_is_yum() {
    let rating = Rating::from_latency(100.0, &test_config());

    assert_eq!(rating, Rating::Yum);
}

#[test]
fn medium_latency_is_meh() {
    let rating = Rating::from_latency(250.0, &test_config());

    assert_eq!(rating, Rating::Meh);
}

#[test]
fn boundary_meh_latency_is_meh() {
    let rating = Rating::from_latency(500.0, &test_config());

    assert_eq!(rating, Rating::Meh);
}

#[test]
fn high_latency_is_yuck() {
    let rating = Rating::from_latency(501.0, &test_config());

    assert_eq!(rating, Rating::Yuck);
}

#[test]
fn successful_connection_is_yum() {
    assert_eq!(
        Rating::from_success(true),
        Rating::Yum
    );
}

#[test]
fn failed_connection_is_yuck() {
    assert_eq!(
        Rating::from_success(false),
        Rating::Yuck
    );
}

#[test]
fn rating_strings_are_correct() {
    assert_eq!(Rating::Yum.as_str(), "YUM");
    assert_eq!(Rating::Meh.as_str(), "MEH");
    assert_eq!(Rating::Yuck.as_str(), "YUCK");
}

#[test]
fn rating_display_is_correct() {
    assert_eq!(Rating::Yum.to_string(), "YUM");
    assert_eq!(Rating::Meh.to_string(), "MEH");
    assert_eq!(Rating::Yuck.to_string(), "YUCK");
}