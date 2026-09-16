use std::{
    fs,
    io,
    net::IpAddr,
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Resolver {
    pub address: IpAddr,
}

pub fn system_resolvers() -> io::Result<Vec<Resolver>> {
    let path = Path::new("/etc/resolv.conf");

    let contents = fs::read_to_string(path)?;

    Ok(parse_resolv_conf(&contents))
}

fn parse_resolv_conf(contents: &str) -> Vec<Resolver> {
    let mut resolvers = Vec::new();

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();

        if parts.next() != Some("nameserver") {
            continue;
        }

        let address = match parts.next() {
            Some(value) => value,
            None => continue,
        };

        if let Ok(ip) = address.parse::<IpAddr>() {
            resolvers.push(Resolver { address: ip });
        }
    }

    resolvers
}

pub fn resolver_addresses() -> io::Result<Vec<IpAddr>> {
    Ok(system_resolvers()?
        .into_iter()
        .map(|resolver| resolver.address)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resolvers() {
        let contents = "\
nameserver 1.1.1.1
nameserver 8.8.8.8
nameserver 2606:4700:4700::1111
";

        let resolvers = parse_resolv_conf(contents);

        assert_eq!(resolvers.len(), 3);

        assert_eq!(
            resolvers[0].address,
            "1.1.1.1".parse::<IpAddr>().unwrap()
        );

        assert_eq!(
            resolvers[1].address,
            "8.8.8.8".parse::<IpAddr>().unwrap()
        );
    }
}