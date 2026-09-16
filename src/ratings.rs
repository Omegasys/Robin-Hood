use crate::config::RatingConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
    Yum,
    Meh,
    Yuck,
}

impl Rating {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Yum => "YUM",
            Self::Meh => "MEH",
            Self::Yuck => "YUCK",
        }
    }

    pub fn from_latency(latency_ms: f64, config: &RatingConfig) -> Self {
        if latency_ms <= config.yum_max_latency_ms as f64 {
            Self::Yum
        } else if latency_ms <= config.meh_max_latency_ms as f64 {
            Self::Meh
        } else {
            Self::Yuck
        }
    }

    pub fn from_success(success: bool) -> Self {
        if success {
            Self::Yum
        } else {
            Self::Yuck
        }
    }
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}