mod sysinfo;

fn main() {
    let (mem, swap) = sysinfo::mem().unwrap_or_default();
    let out = format!(
        "User: {}\nHost: {}\nOS: {}\nKernel: {}\nCPU: {}\nGPU: {}\nRAM: {:.1} GiB\nSwap: {:.1} GiB\nUptime: {}\n",
        sysinfo::user().unwrap_or_default(),
        sysinfo::host(),
        sysinfo::os(),
        sysinfo::kernel(),
        sysinfo::cpu_g().unwrap_or_default(),
        sysinfo::get_gpu_name().unwrap_or_default(),
        mem,
        swap,
        sysinfo::uptime().unwrap_or_default(),
    );
    print!("{out}");
}
