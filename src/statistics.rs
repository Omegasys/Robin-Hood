use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Statistics {
    pub attempts: u32,
    pub successes: u32,
    pub failures: u32,
    pub latencies_ms: Vec<f64>,
}

impl Statistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_success(&mut self, duration: Duration) {
        self.attempts += 1;
        self.successes += 1;
        self.latencies_ms.push(duration.as_secs_f64() * 1000.0);
    }

    pub fn record_failure(&mut self) {
        self.attempts += 1;
        self.failures += 1;
    }

    pub fn packet_loss_percent(&self) -> f64 {
        if self.attempts == 0 {
            return 0.0;
        }

        (self.failures as f64 / self.attempts as f64) * 100.0
    }

    pub fn min_latency_ms(&self) -> Option<f64> {
        self.latencies_ms.iter().copied().reduce(f64::min)
    }

    pub fn max_latency_ms(&self) -> Option<f64> {
        self.latencies_ms.iter().copied().reduce(f64::max)
    }

    pub fn average_latency_ms(&self) -> Option<f64> {
        if self.latencies_ms.is_empty() {
            return None;
        }

        Some(
            self.latencies_ms.iter().sum::<f64>()
                / self.latencies_ms.len() as f64,
        )
    }

    pub fn median_latency_ms(&self) -> Option<f64> {
        if self.latencies_ms.is_empty() {
            return None;
        }

        let mut values = self.latencies_ms.clone();
        values.sort_by(|a, b| a.total_cmp(b));

        let middle = values.len() / 2;

        if values.len() % 2 == 0 {
            Some((values[middle - 1] + values[middle]) / 2.0)
        } else {
            Some(values[middle])
        }
    }

    pub fn jitter_ms(&self) -> Option<f64> {
        if self.latencies_ms.len() < 2 {
            return None;
        }

        let mut differences = Vec::new();

        for pair in self.latencies_ms.windows(2) {
            differences.push((pair[1] - pair[0]).abs());
        }

        Some(
            differences.iter().sum::<f64>()
                / differences.len() as f64,
        )
    }

    pub fn summary(&self) -> String {
        if self.attempts == 0 {
            return "No attempts".to_string();
        }

        let average = self
            .average_latency_ms()
            .map(|v| format!("{v:.1} ms avg"))
            .unwrap_or_else(|| "no latency".to_string());

        let min = self
            .min_latency_ms()
            .map(|v| format!("{v:.1} ms min"))
            .unwrap_or_else(|| "-".to_string());

        let max = self
            .max_latency_ms()
            .map(|v| format!("{v:.1} ms max"))
            .unwrap_or_else(|| "-".to_string());

        format!(
            "{} | {} | {:.1}% loss | {} / {}",
            average,
            format!("{min}, {max}"),
            self.packet_loss_percent(),
            self.successes,
            self.attempts
        )
    }
}