use std::time::Duration;

use tokio::{
    net::TcpStream,
    time::timeout,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reachability {
    Reachable,
    Unreachable,
    Timeout,
}

impl Reachability {
    pub fn is_reachable(&self) -> bool {
        matches!(self, Self::Reachable)
    }
}

pub async fn tcp_reachable(
    host: &str,
    port: u16,
    timeout_duration: Duration,
) -> Reachability {
    let address = format!("{host}:{port}");

    match timeout(
        timeout_duration,
        TcpStream::connect(address),
    )
    .await
    {
        Ok(Ok(_)) => Reachability::Reachable,

        Ok(Err(_)) => Reachability::Unreachable,

        Err(_) => Reachability::Timeout,
    }
}

pub async fn internet_reachable(
    timeout_duration: Duration,
) -> Reachability {
    /*
     * TCP/443 is used instead of ICMP here because some networks
     * intentionally block ICMP while still providing Internet access.
     */
    tcp_reachable(
        "1.1.1.1",
        443,
        timeout_duration,
    )
    .await
}