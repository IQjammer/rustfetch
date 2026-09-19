use std::fs;

fn get_user(raw_file: &str) -> Option<&str> {
    raw_file.lines().find_map(|line| {
        let mut parts = line.split(':');

        let user = parts.next()?;
        let uid = parts.nth(1)?;

        let uid = uid.parse::<u32>().ok()?;

        if uid == 1000 { Some(user) } else { None }
    })
}

fn get_cpu(raw_file: &str) -> Option<&str> {
    raw_file.lines().find_map(|line| {
        let rest = line.strip_prefix("model name")?;
        let (_, model) = rest.split_once(':')?;
        Some(model.trim())
    })
}

fn get_gpu(gpu: &str, gpu_list: &str) -> Option<String> {
    let gpu_clean = gpu.strip_prefix("0x").unwrap_or(gpu).trim();
    for i in gpu_list.trim().lines() {
        if !i.starts_with('\t') || !i.contains(gpu_clean) {
            continue;
        }

        if let Some((_, model)) = i.split_once("[") {
            return Some(model.trim_end_matches(']').to_string());
        }
    }
    None
}

fn get_ram(ram: &str) -> u64 {
    ram.lines()
        .find_map(|line| {
            let rest = line.strip_prefix("MemTotal:")?;
            let kb = rest.split_whitespace().next()?;
            let kb = kb.parse::<u64>().ok()?;
            Some(kb / 1024 / 1024)
        })
        .unwrap_or(0)
}

fn get_os(os_raw: &str) -> Option<&str> {
    for i in os_raw.lines() {
        if !i.starts_with("PRETTY_NAME=") {
            continue;
        }

        if let Some((_, dist)) = i.split_once('"') {
            return Some(dist.trim_end_matches('"'));
        }
    }
    None
}

fn main() {
    let user = fs::read_to_string("/etc/passwd").unwrap_or_default();
    let cpu = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();

    let gpu = fs::read_to_string("/sys/class/drm/card0/device/device").unwrap_or_default();
    let gpu_list = fs::read_to_string("/usr/share/hwdata/pci.ids").unwrap_or_default();

    let ram = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let os = fs::read_to_string("/etc/os-release").unwrap_or_default();

    let host = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").unwrap_or_default();

    println!("User: {}", get_user(&user).unwrap_or("unknown"));
    println!("Host: {}", host.trim());
    println!("OS: {}", get_os(&os).unwrap_or("unknown"));
    println!("CPU: {}", get_cpu(&cpu).unwrap_or_default());
    println!("GPU: {}", get_gpu(&gpu, &gpu_list).unwrap_or_default());
    println!("RAM: {} Gb", get_ram(&ram));
}
