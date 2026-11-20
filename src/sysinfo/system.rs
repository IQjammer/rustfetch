use std::env;
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

pub fn user() -> Option<String> {
    if let Ok(u) = env::var("USER").or_else(|_| env::var("LOGNAME"))
        && !u.is_empty()
    {
        return Some(u);
    }

    let raw = fs::read_to_string("/etc/passwd").ok()?;

    raw.lines().find_map(|line| {
        let mut parts = line.split(':');

        let name = parts.next()?;
        let uid = parts.nth(1)?.parse::<u32>().ok()?;

        (uid == 1000).then(|| name.to_string())
    })
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

pub fn kernel() -> String {
    let Ok(content) = fs::read_to_string("/proc/version") else {
        return String::from("None");
    };

    let clean = content
        .split_once(" (")
        .map_or(content.as_str(), |(v, _)| v);

    clean.replace(" version", "").trim().to_string()
}

pub fn uptime() -> Option<String> {
    let content = fs::read_to_string("/proc/uptime").ok()?;

    let (secs_str, _) = content.split_once('.')?;

    let total_secs = secs_str.parse::<u32>().ok()?;

    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let res = if days > 0 {
        format!("{days} days, {hours} hours, {minutes} minutes")
    } else if hours > 0 {
        format!("{hours} hours, {minutes} minutes")
    } else if minutes > 0 {
        format!("{minutes} minutes, {seconds} seconds")
    } else {
        format!("{seconds} seconds")
    };

    Some(res)
}
