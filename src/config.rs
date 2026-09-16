use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,

    #[serde(default)]
    pub tests: Vec<TestConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    #[serde(default = "default_retries")]
    pub retries: u32,

    #[serde(default = "default_parallel")]
    pub parallel: bool,

    #[serde(default = "default_true")]
    pub show_statistics: bool,

    #[serde(default = "default_true")]
    pub show_failed_tests: bool,

    #[serde(default)]
    pub rating: RatingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RatingConfig {
    #[serde(default = "default_yum_latency")]
    pub yum_max_latency_ms: u64,

    #[serde(default = "default_meh_latency")]
    pub meh_max_latency_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub test_type: String,

    pub target: Option<String>,
    pub server: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timeout_seconds: default_timeout(),
            retries: default_retries(),
            parallel: default_parallel(),
            show_statistics: true,
            show_failed_tests: true,
            rating: RatingConfig::default(),
        }
    }
}

impl Default for RatingConfig {
    fn default() -> Self {
        Self {
            yum_max_latency_ms: default_yum_latency(),
            meh_max_latency_ms: default_meh_latency(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Unable to read configuration: {}", path.display()))?;

        let config: Config = serde_yaml::from_str(&contents)
            .with_context(|| format!("Invalid YAML configuration: {}", path.display()))?;

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.tests.is_empty() {
            anyhow::bail!("Configuration contains no tests");
        }

        if self.settings.timeout_seconds == 0 {
            anyhow::bail!("timeout_seconds must be greater than zero");
        }

        if self.settings.rating.yum_max_latency_ms
            >= self.settings.rating.meh_max_latency_ms
        {
            anyhow::bail!(
                "yum_max_latency_ms must be lower than meh_max_latency_ms"
            );
        }

        for test in &self.tests {
            if test.name.trim().is_empty() {
                anyhow::bail!("A test has an empty name");
            }

            if test.test_type.trim().is_empty() {
                anyhow::bail!(
                    "Test '{}' does not specify a test type",
                    test.name
                );
            }
        }

        Ok(())
    }
}

pub fn load_config(path: Option<&Path>) -> Result<Config> {
    match path {
        Some(path) => Config::load(path),

        None => {
            let default_path = default_config_path();

            if default_path.exists() {
                Config::load(&default_path)
            } else {
                Ok(default_config())
            }
        }
    }
}

fn default_config_path() -> PathBuf {
    if let Some(home) = std::env::var_os("HOME") {
        PathBuf::from(home)
            .join(".config")
            .join("redrobin")
            .join("config.yml")
    } else {
        PathBuf::from("redrobin.yml")
    }
}

fn default_config() -> Config {
    Config {
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
            TestConfig {
                name: "Cloudflare DNS".to_string(),
                test_type: "icmp".to_string(),
                target: Some("1.1.1.1".to_string()),
                server: None,
                domain: None,
                url: None,
                address: None,
                port: None,
            },
            TestConfig {
                name: "Quad9 DNS".to_string(),
                test_type: "icmp".to_string(),
                target: Some("9.9.9.9".to_string()),
                server: None,
                domain: None,
                url: None,
                address: None,
                port: None,
            },
        ],
    }
}

fn default_timeout() -> u64 {
    3
}

fn default_retries() -> u32 {
    2
}

fn default_parallel() -> bool {
    true
}

fn default_true() -> bool {
    true
}

fn default_yum_latency() -> u64 {
    100
}

fn default_meh_latency() -> u64 {
    500
}