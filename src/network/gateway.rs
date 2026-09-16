use std::{
    net::{IpAddr, SocketAddr},
    process::Stdio,
};

use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct Gateway {
    pub address: IpAddr,
    pub interface: Option<String>,
}

pub async fn default_gateway() -> Option<Gateway> {
    #[cfg(target_os = "linux")]
    {
        linux_default_gateway().await
    }

    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

#[cfg(target_os = "linux")]
async fn linux_default_gateway() -> Option<Gateway> {
    let output = Command::new("ip")
        .args(["route", "show", "default"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);

    parse_linux_default_route(&text)
}

fn parse_linux_default_route(output: &str) -> Option<Gateway> {
    let line = output.lines().find(|line| {
        line.split_whitespace()
            .next()
            .is_some_and(|value| value == "default")
    })?;

    let parts: Vec<&str> = line.split_whitespace().collect();

    let gateway = parts
        .iter()
        .position(|value| *value == "via")
        .and_then(|index| parts.get(index + 1))
        .and_then(|value| value.parse::<IpAddr>().ok())?;

    let interface = parts
        .iter()
        .position(|value| *value == "dev")
        .and_then(|index| parts.get(index + 1))
        .map(|value| value.to_string());

    Some(Gateway {
        address: gateway,
        interface,
    })
}

pub fn gateway_socket(gateway: &Gateway) -> SocketAddr {
    SocketAddr::new(gateway.address, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_route() {
        let output =
            "default via 192.168.1.1 dev eth0 proto dhcp metric 100\n";

        let gateway = parse_linux_default_route(output).unwrap();

        assert_eq!(
            gateway.address,
            "192.168.1.1".parse::<IpAddr>().unwrap()
        );

        assert_eq!(
            gateway.interface.as_deref(),
            Some("eth0")
        );
    }
}