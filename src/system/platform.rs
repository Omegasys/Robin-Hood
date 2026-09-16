use std::process::Command;

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub operating_system: String,
    pub architecture: String,
    pub kernel: Option<String>,
    pub hostname: Option<String>,
}

pub fn get_platform_info() -> PlatformInfo {
    PlatformInfo {
        operating_system: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        kernel: kernel_version(),
        hostname: hostname(),
    }
}

fn kernel_version() -> Option<String> {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let version = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if version.is_empty() {
        None
    } else {
        Some(version)
    }
}

fn hostname() -> Option<String> {
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        if !hostname.is_empty() {
            return Some(hostname);
        }
    }

    let output = Command::new("hostname")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let hostname = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if hostname.is_empty() {
        None
    } else {
        Some(hostname)
    }
}

impl PlatformInfo {
    pub fn summary(&self) -> String {
        let kernel = self
            .kernel
            .as_deref()
            .unwrap_or("unknown");

        format!(
            "{} {} | kernel {}",
            self.operating_system,
            self.architecture,
            kernel
        )
    }
}