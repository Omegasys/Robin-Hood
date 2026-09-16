#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub rating: String,
    pub statistics: String,
    pub success: bool,
    pub message: Option<String>,
}

impl TestResult {
    pub fn success(
        name: impl Into<String>,
        rating: impl Into<String>,
        statistics: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            rating: rating.into(),
            statistics: statistics.into(),
            success: true,
            message: None,
        }
    }

    pub fn failure(
        name: impl Into<String>,
        rating: impl Into<String>,
        statistics: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            rating: rating.into(),
            statistics: statistics.into(),
            success: false,
            message: Some(message.into()),
        }
    }

    pub fn yum(
        name: impl Into<String>,
        statistics: impl Into<String>,
    ) -> Self {
        Self::success(name, "YUM", statistics)
    }

    pub fn meh(
        name: impl Into<String>,
        statistics: impl Into<String>,
    ) -> Self {
        Self::success(name, "MEH", statistics)
    }

    pub fn yuck(
        name: impl Into<String>,
        statistics: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::failure(name, "YUCK", statistics, message)
    }
}