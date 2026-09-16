use std::process::Command;

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub command: String,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct DependencyStatus {
    pub dependency: Dependency,
    pub available: bool,
}

pub fn default_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "Ping".to_string(),
            command: "ping".to_string(),
            required: false,
        },
        Dependency {
            name: "IP".to_string(),
            command: "ip".to_string(),
            required: false,
        },
        Dependency {
            name: "Hostname".to_string(),
            command: "hostname".to_string(),
            required: false,
        },
    ]
}

pub fn check_dependency(
    dependency: &Dependency,
) -> DependencyStatus {
    DependencyStatus {
        dependency: dependency.clone(),
        available: command_exists(&dependency.command),
    }
}

pub fn check_all() -> Vec<DependencyStatus> {
    default_dependencies()
        .iter()
        .map(check_dependency)
        .collect()
}

pub fn command_exists(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .map(|output| {
            output.status.success()
                || !output.stdout.is_empty()
                || !output.stderr.is_empty()
        })
        .unwrap_or(false)
}

impl DependencyStatus {
    pub fn status_string(&self) -> &'static str {
        if self.available {
            "available"
        } else if self.dependency.required {
            "missing"
        } else {
            "optional"
        }
    }
}