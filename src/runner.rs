use crate::{
    config::{Config, TestConfig},
    result::TestResult,
};

pub struct Runner {
    config: Config,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Vec<TestResult> {
        if self.config.settings.parallel {
            self.run_parallel().await
        } else {
            self.run_sequential().await
        }
    }

    async fn run_sequential(&self) -> Vec<TestResult> {
        let mut results = Vec::new();

        for test in &self.config.tests {
            results.push(self.run_test(test).await);
        }

        results
    }

    async fn run_parallel(&self) -> Vec<TestResult> {
        let mut handles = Vec::new();

        for test in self.config.tests.clone() {
            let settings = self.config.settings.clone();

            handles.push(tokio::spawn(async move {
                run_single_test(&settings, &test).await
            }));
        }

        let mut results = Vec::new();

        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(error) => results.push(TestResult::yuck(
                    "Internal test",
                    "Task failed",
                    error.to_string(),
                )),
            }
        }

        results
    }

    async fn run_test(&self, test: &TestConfig) -> TestResult {
        run_single_test(&self.config.settings, test).await
    }
}

async fn run_single_test(
    settings: &crate::config::Settings,
    test: &TestConfig,
) -> TestResult {
    match test.test_type.to_lowercase().as_str() {
        "icmp" => run_icmp_placeholder(settings, test).await,

        "dns" => TestResult::yuck(
            &test.name,
            "DNS tester not yet implemented",
            "pending",
        ),

        "http" => TestResult::yuck(
            &test.name,
            "HTTP tester not yet implemented",
            "pending",
        ),

        "https" => TestResult::yuck(
            &test.name,
            "HTTPS tester not yet implemented",
            "pending",
        ),

        "tcp" => TestResult::yuck(
            &test.name,
            "TCP tester not yet implemented",
            "pending",
        ),

        "udp" => TestResult::yuck(
            &test.name,
            "UDP tester not yet implemented",
            "pending",
        ),

        "tls" => TestResult::yuck(
            &test.name,
            "TLS tester not yet implemented",
            "pending",
        ),

        "tor" => TestResult::yuck(
            &test.name,
            "Tor tester not yet implemented",
            "pending",
        ),

        "onion" => TestResult::yuck(
            &test.name,
            "Onion-service tester not yet implemented",
            "pending",
        ),

        "i2p" => TestResult::yuck(
            &test.name,
            "I2P tester not yet implemented",
            "pending",
        ),

        unknown => TestResult::yuck(
            &test.name,
            "Unknown",
            format!("Unsupported test type: {}", unknown),
        ),
    }
}

async fn run_icmp_placeholder(
    _settings: &crate::config::Settings,
    test: &TestConfig,
) -> TestResult {
    let target = match &test.target {
        Some(target) => target,
        None => {
            return TestResult::yuck(
                &test.name,
                "No target",
                "missing target",
            );
        }
    };

    /*
     * ICMP implementation will be added in the network/test modules.
     *
     * We deliberately keep the runner independent from the eventual
     * implementation so that ICMP, DNS, HTTPS, Tor, I2P, etc. can each
     * provide their own TestResult.
     */

    TestResult::yuck(
        &test.name,
        format!("ICMP target: {} | tester pending", target),
        "not yet implemented",
    )
}