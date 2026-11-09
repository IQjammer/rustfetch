use std::fs::File;
use std::io::{self, Read};

pub fn cpu_g() -> io::Result<String> {
    let mut file = File::open("/proc/cpuinfo")?;
    let mut buf = [0u8; 1024];
    let n = file.read(&mut buf)?;
    let text = std::str::from_utf8(&buf[..n]).unwrap_or("");

    for line in text.lines() {
        if let Some((_, model)) = line.split_once(':')
            && line.starts_with("model name")
        {
            return Ok(model.trim().to_string());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "model name not found",
    ))
}
