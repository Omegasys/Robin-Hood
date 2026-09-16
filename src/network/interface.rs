use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub is_up: bool,
    pub mac_address: Option<String>,
}

pub fn list_interfaces() -> io::Result<Vec<NetworkInterface>> {
    let base = Path::new("/sys/class/net");

    let mut interfaces = Vec::new();

    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };

        let is_up = read_operstate(&path)
            .map(|state| state == "up")
            .unwrap_or(false);

        let mac_address = read_mac_address(&path);

        interfaces.push(NetworkInterface {
            name,
            is_up,
            mac_address,
        });
    }

    interfaces.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(interfaces)
}

fn read_operstate(interface_path: &Path) -> io::Result<String> {
    let path = interface_path.join("operstate");

    Ok(fs::read_to_string(path)?
        .trim()
        .to_string())
}

fn read_mac_address(interface_path: &Path) -> Option<String> {
    let path = interface_path.join("address");

    fs::read_to_string(path)
        .ok()
        .map(|mac| mac.trim().to_string())
}

pub fn interface_exists(name: &str) -> bool {
    let path = PathBuf::from("/sys/class/net").join(name);
    path.exists()
}

pub fn interface_is_up(name: &str) -> bool {
    let path = PathBuf::from("/sys/class/net")
        .join(name)
        .join("operstate");

    match fs::read_to_string(path) {
        Ok(state) => state.trim() == "up",
        Err(_) => false,
    }
}