use red_robin::config::{
    Config,
    RatingConfig,
    Settings,
    TestConfig,
};

#[test]
fn valid_config_passes_validation() {
    let config = Config {
        settings: Settings::default(),
        tests: vec![
            TestConfig {
                name: "Google DNS".to_string(),
                test_type: "icmp".to_string(),
                target: Some("8.8.8.8".to_string()),
                server: None,
                domain: None,
                url: None,
                address: None,
                port: None,
            },
        ],
    };

    assert!(config.validate().is_ok());
}

#[test]
fn empty_tests_fail_validation() {
    let config = Config {
        settings: Settings::default(),
        tests: Vec::new(),
    };

    assert!(config.validate().is_err());
}

#[test]
fn zero_timeout_fails_validation() {
    let mut settings = Settings::default();
    settings.timeout_seconds = 0;

    let config = Config {
        settings,
        tests: vec![TestConfig {
            name: "Test".to_string(),
            test_type: "icmp".to_string(),
            target: Some("1.1.1.1".to_string()),
            server: None,
            domain: None,
            url: None,
            address: None,
            port: None,
        }],
    };

    assert!(config.validate().is_err());
}

#[test]
fn invalid_rating_thresholds_fail_validation() {
    let settings = Settings {
        rating: RatingConfig {
            yum_max_latency_ms: 500,
            meh_max_latency_ms: 100,
        },
        ..Settings::default()
    };

    let config = Config {
        settings,
        tests: vec![TestConfig {
            name: "Test".to_string(),
            test_type: "icmp".to_string(),
            target: Some("1.1.1.1".to_string()),
            server: None,
            domain: None,
            url: None,
            address: None,
            port: None,
        }],
    };

    assert!(config.validate().is_err());
}

#[test]
fn empty_test_name_fails_validation() {
    let config = Config {
        settings: Settings::default(),
        tests: vec![TestConfig {
            name: "   ".to_string(),
            test_type: "icmp".to_string(),
            target: Some("1.1.1.1".to_string()),
            server: None,
            domain: None,
            url: None,
            address: None,
            port: None,
        }],
    };

    assert!(config.validate().is_err());
}

#[test]
fn empty_test_type_fails_validation() {
    let config = Config {
        settings: Settings::default(),
        tests: vec![TestConfig {
            name: "Test".to_string(),
            test_type: "   ".to_string(),
            target: Some("1.1.1.1".to_string()),
            server: None,
            domain: None,
            url: None,
            address: None,
            port: None,
        }],
    };

    assert!(config.validate().is_err());
}