use crate::{
    config::Settings,
    ratings::Rating,
    result::TestResult,
    statistics::Statistics,
};

use std::{
    net::{IpAddr, SocketAddr},
    time::{Duration, Instant},
};

use tokio::net::UdpSocket;

pub async fn test(
    settings: &Settings,
    name: &str,
    server: &str,
    domain: &str,
) -> TestResult {
    let server_ip: IpAddr = match server.parse() {
        Ok(ip) => ip,
        Err(_) => {
            return TestResult::yuck(
                name,
                "Invalid DNS server",
                format!("Invalid IP address: {server}"),
            );
        }
    };

    let address = SocketAddr::new(server_ip, 53);

    let mut statistics = Statistics::new();

    for _ in 0..=settings.retries {
        let start = Instant::now();

        let socket = match UdpSocket::bind("0.0.0.0:0").await {
            Ok(socket) => socket,
            Err(error) => {
                statistics.record_failure();

                return TestResult::yuck(
                    name,
                    statistics.summary(),
                    format!("Unable to create UDP socket: {error}"),
                );
            }
        };

        let query_id = (start.elapsed().subsec_nanos() & 0xffff) as u16;

        let packet = build_dns_query(query_id, domain);

        if socket.send_to(&packet, address).await.is_err() {
            statistics.record_failure();
            continue;
        }

        let mut buffer = [0u8; 4096];

        let received = tokio::time::timeout(
            Duration::from_secs(settings.timeout_seconds),
            socket.recv_from(&mut buffer),
        )
        .await;

        match received {
            Ok(Ok((size, _source))) if size >= 12 => {
                if valid_dns_response(&buffer[..size], query_id) {
                    statistics.record_success(start.elapsed());
                } else {
                    statistics.record_failure();
                }
            }

            _ => {
                statistics.record_failure();
            }
        }
    }

    if statistics.successes == 0 {
        return TestResult::yuck(
            name,
            statistics.summary(),
            format!(
                "DNS query for {domain} failed through {server}"
            ),
        );
    }

    let latency = statistics.average_latency_ms().unwrap_or(f64::MAX);

    let rating = Rating::from_latency(
        latency,
        &settings.rating,
    );

    TestResult::success(
        name,
        rating.as_str(),
        format!(
            "{} | {} -> {}",
            statistics.summary(),
            server,
            domain
        ),
    )
}

fn build_dns_query(id: u16, domain: &str) -> Vec<u8> {
    let mut packet = Vec::with_capacity(512);

    packet.extend_from_slice(&id.to_be_bytes());

    // Standard recursive DNS query.
    packet.extend_from_slice(&0x0100u16.to_be_bytes());

    // One question.
    packet.extend_from_slice(&1u16.to_be_bytes());

    // No answers.
    packet.extend_from_slice(&0u16.to_be_bytes());

    // No authority records.
    packet.extend_from_slice(&0u16.to_be_bytes());

    // No additional records.
    packet.extend_from_slice(&0u16.to_be_bytes());

    for label in domain.split('.') {
        let bytes = label.as_bytes();

        if bytes.len() > 63 {
            return packet;
        }

        packet.push(bytes.len() as u8);
        packet.extend_from_slice(bytes);
    }

    packet.push(0);

    // QTYPE = A
    packet.extend_from_slice(&1u16.to_be_bytes());

    // QCLASS = IN
    packet.extend_from_slice(&1u16.to_be_bytes());

    packet
}

fn valid_dns_response(packet: &[u8], id: u16) -> bool {
    if packet.len() < 12 {
        return false;
    }

    let response_id = u16::from_be_bytes([packet[0], packet[1]]);

    if response_id != id {
        return false;
    }

    // QR bit must indicate a response.
    packet[2] & 0x80 != 0
}