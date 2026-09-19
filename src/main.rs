use std::fs;

fn get_user(raw_file: &str) -> &str {
    for i in raw_file.lines() {
        if i.starts_with('_') {
            continue;
        }
        if i.contains(":1000:") {
            if let Some((user, _)) = i.split_once(':') {
                return user;
            }
        }
    }
    "unknown"
}

fn get_cpu(raw_file: &str) -> &str {
    for i in raw_file.lines() {
        if !i.starts_with("model name") {
            continue;
        }
        if let Some((_, model)) = i.split_once(':') {
            return model.trim();
        }
    }
    "unknown"
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
    for i in ram.lines() {
        if let Some((_, quantity)) = i.split_once(":") {
            let ram_clean = quantity
                .trim()
                .strip_suffix(" kB")
                .unwrap_or(quantity.trim());
            if let Ok(kb) = ram_clean.parse::<u64>() {
                let gb = kb / 1024 / 1024;
                return gb;
            }
        }
    }

    0
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
    let user = fs::read_to_string("/etc/passwd").expect("Not Found");
    let cpu = fs::read_to_string("/proc/cpuinfo").expect("Not Found");

    let gpu = fs::read_to_string("/sys/class/drm/card0/device/device").expect("Not Found");
    let gpu_list = fs::read_to_string("/usr/share/hwdata/pci.ids").expect("Not Found");

    let ram = fs::read_to_string("/proc/meminfo").expect("Not Found");
    let os = fs::read_to_string("/etc/os-release").expect("Not Found");

    let host = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").expect("Not Found");

    println!("User: {}", get_user(&user));
    println!("Host: {}", host.trim());
    println!("Os: {}", get_os(&os).unwrap_or("unknown"));
    println!("cpu: {}", get_cpu(&cpu));
    println!("GPU: {}", get_gpu(&gpu, &gpu_list).unwrap_or_default());
    println!("RAM: {} Gb", get_ram(&ram));
}
