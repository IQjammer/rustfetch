use std::fs;
use std::fs::File;
use std::io::Read;

pub fn ram_gib() -> Option<f64> {
    let mut buf = [0u8; 128];
    let n = File::open("/proc/meminfo").ok()?.read(&mut buf).ok()?;
    let text = std::str::from_utf8(&buf[..n]).ok()?;

    let kb: u64 = text
        .lines()
        .find_map(|l| l.strip_prefix("MemTotal:"))?
        .split_whitespace()
        .next()?
        .parse()
        .ok()?;

    Some(kb as f64 / 1024.0 / 1024.0)
}

pub fn user() -> String {
    let raw = fs::read_to_string("/etc/passwd").unwrap_or_default();

    raw.lines()
        .find_map(|line| {
            let mut parts = line.split(':');

            let name = parts.next()?;
            let uid = parts.nth(1)?.parse::<u32>().ok()?;

            (uid == 1000).then(|| name.to_string())
        })
        .unwrap_or_else(|| "unknown".into())
}

pub fn host() -> String {
    let host = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").unwrap_or_default();
    host.trim().to_string()
}

pub fn os() -> String {
    let raw = fs::read_to_string("/etc/os-release").unwrap_or_default();

    parse_os(&raw).unwrap_or("unknown").to_string()
}

fn parse_os(raw: &str) -> Option<&str> {
    raw.lines().find_map(|line| {
        let val = line.strip_prefix("PRETTY_NAME=")?;
        Some(val.trim_matches('"'))
    })
}
