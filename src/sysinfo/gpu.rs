use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn lookup(vendor: &str, device: &str) -> Option<String> {
    let file = File::open("/usr/share/hwdata/pci.ids").ok()?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut line = String::new();
    let mut in_vendor = false;

    loop {
        line.clear();
        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if !in_vendor {
            if let Some(rest) = line.strip_prefix(vendor) {
                in_vendor = rest.starts_with("  ");
            }
            continue;
        }
        let rest = line.strip_prefix('\t')?;

        let Some(rest) = rest.strip_prefix(device) else {
            continue;
        };
        let Some(name) = rest.strip_prefix("  ") else {
            continue;
        };
        let name = name.trim_end();

        return Some(
            name.split_once('[')
                .and_then(|(_, m)| m.split_once(']'))
                .map(|(m, _)| m.trim())
                .unwrap_or(name)
                .to_string(),
        );
    }
}

fn is_card(name: &str) -> bool {
    name.strip_prefix("card")
        .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()))
}

fn get_cards() -> Vec<String> {
    let Ok(dir) = fs::read_dir("/sys/class/drm") else {
        return Vec::new();
    };
    let mut cards: Vec<String> = dir
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| is_card(n))
        .collect();
    cards.sort_by_key(|n| n[4..].parse::<u32>().unwrap_or(u32::MAX));
    cards
}

fn read_id(card: &str, file: &str) -> Option<String> {
    let s = fs::read_to_string(format!("/sys/class/drm/{card}/device/{file}")).ok()?;
    Some(s.trim().strip_prefix("0x")?.to_string())
}

pub fn get_gpu_name() -> Option<String> {
    let names: Vec<String> = get_cards()
        .iter()
        .filter_map(|card| {
            let vendor = read_id(card, "vendor")?;
            let device = read_id(card, "device")?;
            lookup(&vendor, &device)
        })
        .collect();

    if names.is_empty() {
        None
    } else {
        Some(names.join(", "))
    }
}
