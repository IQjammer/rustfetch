mod sysinfo;

fn main() {
    let out = format!(
        "User: {}\nHost: {}\nOS: {}\nKernel: {}\nCPU: {}\nGPU: {}\nRAM: {:.1} GiB\nUptime: {}\n",
        sysinfo::user().unwrap_or_default(),
        sysinfo::host(),
        sysinfo::os(),
        sysinfo::kernel(),
        sysinfo::cpu_g().unwrap_or_default(),
        sysinfo::get_gpu_name().unwrap_or_default(),
        sysinfo::ram_gib().unwrap_or_default(),
        sysinfo::uptime().unwrap_or_default(),
    );
    print!("{out}");
}
