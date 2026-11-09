use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_gpu_name() -> Option<String> {
    let vendor = fs::read_to_string("/sys/class/drm/card0/device/vendor").unwrap_or_default();
    let device = fs::read_to_string("/sys/class/drm/card0/device/device").unwrap_or_default();

    let vendor = vendor.trim().strip_prefix("0x")?;
    let device = device.trim().strip_prefix("0x")?;

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
                .map(|(m, _)| m.trim())?
                .to_string(),
        );
    }
}
