mod sysinfo;

fn main() {
    let out = format!(
        "User: {}\nHost: {}\nOS: {}\nCPU: {}\nGPU: {}\nRAM: {:.1} GiB\n",
        sysinfo::user(),
        sysinfo::host(),
        sysinfo::os(),
        sysinfo::cpu_g().unwrap(),
        sysinfo::get_gpu_name().unwrap(),
        sysinfo::ram_gib().unwrap(),
    );
    print!("{out}");
}
